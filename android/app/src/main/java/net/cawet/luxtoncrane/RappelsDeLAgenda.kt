package net.cawet.luxtoncrane

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.net.Uri
import androidx.work.Data
import androidx.work.ExistingWorkPolicy
import androidx.work.OneTimeWorkRequest
import androidx.work.WorkManager
import androidx.work.Worker
import androidx.work.WorkerParameters
import java.time.ZoneId
import java.util.concurrent.TimeUnit

private const val PAGE_WEB = "https://lux-ton-crane.cawet.net"
private const val NOTIFICATION_CHANNEL_ID = "rappels"
private const val REMINDER_KEY = "rappel"

/**
 * Planifie les deux rappels de l'agenda du sommeil, à 8 h et à 21 h. Chaque ouverture de l'accueil les recalcule :
 * un changement d'heure ou de fuseau est ainsi rattrapé.
 */
fun planifierLesRappelsDeLAgenda(context: Context) {
    for (rappel in RappelDeLAgenda.entries) {
        planifierLeRappel(context, rappel, ExistingWorkPolicy.REPLACE)
    }
}

/**
 * WorkManager plutôt qu'une alarme : il survit au redémarrage du téléphone, sans permission d'alarme exacte.
 * Le rappel peut sonner avec quelques minutes de retard, quand Android regroupe les tâches de fond.
 */
private fun planifierLeRappel(context: Context, rappel: RappelDeLAgenda, policy: ExistingWorkPolicy) {
    val maintenantMs = System.currentTimeMillis()
    val prochainRappelMs = instantDuProchainRappelMs(rappel, maintenantMs, ZoneId.systemDefault())
    val input = Data.Builder()
        .putString(REMINDER_KEY, rappel.name)
        .build()
    val request = OneTimeWorkRequest.Builder(RappelPlanifié::class.java)
        .setInitialDelay(prochainRappelMs - maintenantMs, TimeUnit.MILLISECONDS)
        .setInputData(input)
        .build()
    val workManager = WorkManager.getInstance(context)
    workManager.enqueueUniqueWork(workName(rappel), policy, request)
}

private fun workName(rappel: RappelDeLAgenda): String = "rappel_${rappel.name.lowercase()}"

/** Un rappel de l'agenda, lancé par Android à son heure : il notifie, puis planifie celui du lendemain. */
class RappelPlanifié(context: Context, parameters: WorkerParameters) : Worker(context, parameters) {

    override fun doWork(): Result {
        val nomDuRappel = inputData.getString(REMINDER_KEY)
        val rappel = RappelDeLAgenda.entries.find { rappel -> rappel.name == nomDuRappel } ?: return Result.failure()
        notifier(rappel)
        // REPLACE annulerait ce rappel en cours : le suivant s'y ajoute, et part quand celui-ci se termine.
        planifierLeRappel(applicationContext, rappel, ExistingWorkPolicy.APPEND_OR_REPLACE)
        return Result.success()
    }

    /** Sans la permission d'afficher les notifications, Android ne montre rien. */
    private fun notifier(rappel: RappelDeLAgenda) {
        val channel = NotificationChannel(
            NOTIFICATION_CHANNEL_ID,
            "Rappels de l'agenda du sommeil",
            NotificationManager.IMPORTANCE_DEFAULT,
        )
        val notificationManager = applicationContext.getSystemService(NotificationManager::class.java)
        notificationManager.createNotificationChannel(channel)

        val pageWeb = Intent(Intent.ACTION_VIEW, Uri.parse(PAGE_WEB))
        val openPage = PendingIntent.getActivity(applicationContext, rappel.ordinal, pageWeb, PendingIntent.FLAG_IMMUTABLE)

        val builder = Notification.Builder(applicationContext, NOTIFICATION_CHANNEL_ID)
        builder.setContentTitle("Agenda du sommeil")
        builder.setContentText(rappel.texte)
        builder.setSmallIcon(android.R.drawable.ic_menu_agenda)
        builder.setContentIntent(openPage)
        builder.setAutoCancel(true)
        notificationManager.notify(notificationId(rappel), builder.build())
    }

    // Après la notification permanente de la mesure, qui porte le numéro 1.
    private fun notificationId(rappel: RappelDeLAgenda): Int = 2 + rappel.ordinal
}
