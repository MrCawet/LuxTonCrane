package net.cawet.luxtoncrane

import android.content.Context
import android.util.Log
import androidx.work.Constraints
import androidx.work.ExistingPeriodicWorkPolicy
import androidx.work.NetworkType
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
private const val WORK_NAME = "synchronisation"

/** Le jeton du serveur, saisi une fois sur l'accueil, ou `null` s'il ne l'a pas encore été. */
fun jetonDuServeur(context: Context): String? {
    val preferences = context.getSharedPreferences(PREFERENCES, Context.MODE_PRIVATE)
    return preferences.getString(TOKEN_KEY, null)
}

fun garderLeJetonDuServeur(context: Context, jeton: String) {
    val preferences = context.getSharedPreferences(PREFERENCES, Context.MODE_PRIVATE)
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
                        àEnvoyer.map { mesure -> MesureÀEnvoyer(mesure.valeurLux, mesure.instantMs) }
                    },
                    serveur = ServeurHttp(jeton),
                )
            }
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
