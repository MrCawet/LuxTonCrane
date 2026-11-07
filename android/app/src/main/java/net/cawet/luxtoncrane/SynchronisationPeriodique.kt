package net.cawet.luxtoncrane

import android.content.Context
import android.content.SharedPreferences
import android.util.Log
import androidx.work.Constraints
import androidx.work.ExistingPeriodicWorkPolicy
import androidx.work.ExistingWorkPolicy
import androidx.work.NetworkType
import androidx.work.OneTimeWorkRequest
import androidx.work.PeriodicWorkRequest
import androidx.work.WorkManager
import androidx.work.Worker
import androidx.work.WorkerParameters
import java.io.IOException
import java.util.concurrent.TimeUnit
import net.cawet.luxtoncrane.pont.EchecPont
import net.cawet.luxtoncrane.pont.Journal
import org.json.JSONException

private const val LOG_TAG = "LuxTonCrane"
private const val PREFERENCES = "synchronisation"
private const val TOKEN_KEY = "jeton"
const val LAST_SUCCESS_KEY = "derniere_synchronisation_reussie_ms"
private const val WORK_NAME = "synchronisation"
private const val IMMEDIATE_WORK_NAME = "synchronisation_maintenant"

/** Les préférences où la synchronisation garde le jeton et l'instant de sa dernière réussite. */
fun préférencesDeLaSynchronisation(context: Context): SharedPreferences =
    context.getSharedPreferences(PREFERENCES, Context.MODE_PRIVATE)

/** L'instant de la dernière synchronisation réussie, ou `null` s'il n'y en a pas encore eu. */
fun dernièreSynchronisationRéussieMs(context: Context): Long? {
    val preferences = préférencesDeLaSynchronisation(context)
    if (!preferences.contains(LAST_SUCCESS_KEY)) {
        return null
    }
    return preferences.getLong(LAST_SUCCESS_KEY, 0)
}

/** Le jeton du serveur, saisi une fois sur l'accueil, ou `null` s'il ne l'a pas encore été. */
fun jetonDuServeur(context: Context): String? {
    val preferences = préférencesDeLaSynchronisation(context)
    return preferences.getString(TOKEN_KEY, null)
}

fun garderLeJetonDuServeur(context: Context, jeton: String) {
    val preferences = préférencesDeLaSynchronisation(context)
    preferences.edit().putString(TOKEN_KEY, jeton).apply()
}

/** Envoie les mesures au serveur environ toutes les 15 min, l'intervalle le plus court qu'Android accorde, quand le réseau est là. */
fun planifierLaSynchronisation(context: Context) {
    val withNetwork = Constraints.Builder()
        .setRequiredNetworkType(NetworkType.CONNECTED)
        .build()
    val request = PeriodicWorkRequest.Builder(SynchronisationPériodique::class.java, 15, TimeUnit.MINUTES)
        .setConstraints(withNetwork)
        .build()
    val workManager = WorkManager.getInstance(context)
    workManager.enqueueUniquePeriodicWork(WORK_NAME, ExistingPeriodicWorkPolicy.KEEP, request)
}

/** Envoie les mesures tout de suite, dès que le réseau est là. Un nouvel appui remplace la demande encore en attente. */
fun synchroniserMaintenant(context: Context) {
    val withNetwork = Constraints.Builder()
        .setRequiredNetworkType(NetworkType.CONNECTED)
        .build()
    val request = OneTimeWorkRequest.Builder(SynchronisationPériodique::class.java)
        .setConstraints(withNetwork)
        .build()
    val workManager = WorkManager.getInstance(context)
    workManager.enqueueUniqueWork(IMMEDIATE_WORK_NAME, ExistingWorkPolicy.REPLACE, request)
}

/** Une synchronisation, lancée par Android. Un échec est retenté plus tard : le serveur dit lui-même ce qu'il a reçu. */
class SynchronisationPériodique(context: Context, parameters: WorkerParameters) : Worker(context, parameters) {

    override fun doWork(): Result {
        val jeton = jetonDuServeur(applicationContext)
        if (jeton == null) {
            Log.i(LOG_TAG, "Pas de jeton du serveur : aucune mesure n'est envoyée.")
            return Result.success()
        }
        try {
            Journal.ouvrir(cheminDuJournal(applicationContext)).use { journal ->
                synchroniser(
                    mesuresAprès = { dernierInstantReçuMs ->
                        val àEnvoyer = journal.mesuresApres(dernierInstantReçuMs)
                        àEnvoyer.map { mesure -> MesureÀEnvoyer(mesure.valeurLux, mesure.instantMs, mesure.telephoneCouvert) }
                    },
                    serveur = ServeurHttp(jeton),
                )
            }
            val preferences = préférencesDeLaSynchronisation(applicationContext)
            preferences.edit().putLong(LAST_SUCCESS_KEY, System.currentTimeMillis()).apply()
            return Result.success()
        } catch (échec: IOException) {
            Log.w(LOG_TAG, "Synchronisation impossible, elle sera retentée.", échec)
            return Result.retry()
        } catch (échec: JSONException) {
            Log.w(LOG_TAG, "Réponse du serveur illisible, la synchronisation sera retentée.", échec)
            return Result.retry()
        } catch (échec: EchecPont) {
            Log.e(LOG_TAG, "Le journal n'a pas pu être lu pour la synchronisation.", échec)
            return Result.retry()
        }
    }
}
