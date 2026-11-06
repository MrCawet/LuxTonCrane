package net.cawet.luxtoncrane

import android.app.AlarmManager
import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.content.pm.ServiceInfo
import android.hardware.Sensor
import android.hardware.SensorEvent
import android.hardware.SensorEventListener
import android.hardware.SensorManager
import android.os.Build
import android.os.IBinder
import android.os.PowerManager
import android.os.SystemClock
import android.util.Log
import net.cawet.luxtoncrane.pont.EchecPont
import net.cawet.luxtoncrane.pont.Journal

/**
 * Service de premier plan : sans lui, Android ne transmet aucune valeur du capteur
 * à une application en arrière-plan. Écran allumé, le capteur de lumière reste ouvert ;
 * écran éteint, une alarme l'ouvre environ toutes les 9 minutes, le temps d'une mesure.
 */
class ServiceDeMesure : Service(), SensorEventListener {

    private lateinit var journal: Journal
    private lateinit var sensorManager: SensorManager
    private lateinit var alarmManager: AlarmManager
    private lateinit var alarmIntent: PendingIntent
    private lateinit var wakeLock: PowerManager.WakeLock

    private val screenAndAlarmReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context, intent: Intent) {
            when (intent.action) {
                Intent.ACTION_SCREEN_ON -> {
                    annulerLAlarme()
                    ouvrirLeCapteur()
                }
                Intent.ACTION_SCREEN_OFF -> {
                    fermerLeCapteur()
                    programmerLAlarme()
                }
                ALARM_ACTION -> {
                    programmerLAlarme()
                    prendreUneMesure()
                }
            }
        }
    }

    /** Écran éteint : note la première valeur du capteur, puis le referme et laisse le processeur se rendormir. */
    private val mesureÉcranÉteint = object : SensorEventListener {
        override fun onSensorChanged(event: SensorEvent) {
            enregistrer(event)
            sensorManager.unregisterListener(this)
            relâcherLeProcesseur()
        }

        override fun onAccuracyChanged(sensor: Sensor, accuracy: Int) = Unit
    }

    override fun onCreate() {
        super.onCreate()
        startForegroundWithNotification()

        try {
            journal = Journal.ouvrir(cheminDuJournal(this))
        } catch (échec: EchecPont) {
            Log.e(LOG_TAG, "Le journal n'a pas pu être ouvert, la mesure s'arrête.", échec)
            stopSelf()
            return
        }

        sensorManager = getSystemService(SensorManager::class.java)
        alarmManager = getSystemService(AlarmManager::class.java)
        val alarm = Intent(ALARM_ACTION)
        alarm.setPackage(packageName)
        alarmIntent = PendingIntent.getBroadcast(this, 0, alarm, PendingIntent.FLAG_IMMUTABLE)
        val powerManager = getSystemService(PowerManager::class.java)
        wakeLock = powerManager.newWakeLock(PowerManager.PARTIAL_WAKE_LOCK, "LuxTonCrane:mesure")
        wakeLock.setReferenceCounted(false)

        val screenAndAlarmEvents = IntentFilter()
        screenAndAlarmEvents.addAction(Intent.ACTION_SCREEN_ON)
        screenAndAlarmEvents.addAction(Intent.ACTION_SCREEN_OFF)
        screenAndAlarmEvents.addAction(ALARM_ACTION)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            registerReceiver(screenAndAlarmReceiver, screenAndAlarmEvents, Context.RECEIVER_NOT_EXPORTED)
        } else {
            registerReceiver(screenAndAlarmReceiver, screenAndAlarmEvents)
        }

        if (powerManager.isInteractive) {
            ouvrirLeCapteur()
        } else {
            programmerLAlarme()
        }
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int = START_STICKY

    override fun onDestroy() {
        if (::journal.isInitialized) {
            unregisterReceiver(screenAndAlarmReceiver)
            annulerLAlarme()
            fermerLeCapteur()
            sensorManager.unregisterListener(mesureÉcranÉteint)
            relâcherLeProcesseur()
            journal.close()
        }
        super.onDestroy()
    }

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onSensorChanged(event: SensorEvent) = enregistrer(event)

    override fun onAccuracyChanged(sensor: Sensor, accuracy: Int) = Unit

    private fun enregistrer(event: SensorEvent) {
        val valeurLux = event.values[0]
        val instantMs = instantMsDuCapteur(
            horodatageCapteurNs = event.timestamp,
            depuisDémarrageNs = SystemClock.elapsedRealtimeNanos(),
            maintenantMs = System.currentTimeMillis(),
        )
        try {
            journal.ajouter(valeurLux, instantMs)
        } catch (échec: EchecPont) {
            Log.e(LOG_TAG, "Une mesure n'a pas pu être enregistrée.", échec)
        }
    }

    private fun ouvrirLeCapteur() {
        val capteurDeLumière = capteurDeLumière() ?: return
        sensorManager.registerListener(this, capteurDeLumière, SensorManager.SENSOR_DELAY_NORMAL)
    }

    private fun fermerLeCapteur() {
        sensorManager.unregisterListener(this)
    }

    /**
     * Android ne garde le processeur éveillé que le temps de recevoir l'alarme,
     * alors que le capteur met un peu de temps à donner sa première valeur.
     */
    private fun prendreUneMesure() {
        val capteurDeLumière = capteurDeLumière() ?: return
        wakeLock.acquire(DURÉE_MAXIMALE_D_UNE_MESURE_MS)
        sensorManager.registerListener(mesureÉcranÉteint, capteurDeLumière, SensorManager.SENSOR_DELAY_NORMAL)
    }

    private fun relâcherLeProcesseur() {
        if (wakeLock.isHeld) {
            wakeLock.release()
        }
    }

    private fun capteurDeLumière(): Sensor? {
        val capteurDeLumière = sensorManager.getDefaultSensor(Sensor.TYPE_LIGHT)
        if (capteurDeLumière == null) {
            Log.e(LOG_TAG, "Ce téléphone n'a pas de capteur de lumière.")
        }
        return capteurDeLumière
    }

    /**
     * 9 minutes est l'intervalle le plus court qu'Android accorde pendant Doze.
     * L'alarme n'est pas exacte : le système peut la décaler.
     */
    private fun programmerLAlarme() {
        val déclenchementMs = SystemClock.elapsedRealtime() + INTERVALLE_ENTRE_DEUX_ALARMES_MS
        alarmManager.setAndAllowWhileIdle(AlarmManager.ELAPSED_REALTIME_WAKEUP, déclenchementMs, alarmIntent)
    }

    private fun annulerLAlarme() {
        alarmManager.cancel(alarmIntent)
    }

    private fun startForegroundWithNotification() {
        val channel = NotificationChannel(
            NOTIFICATION_CHANNEL_ID,
            "Mesure de la lumière",
            NotificationManager.IMPORTANCE_LOW,
        )
        val notificationManager = getSystemService(NotificationManager::class.java)
        notificationManager.createNotificationChannel(channel)

        val builder = Notification.Builder(this, NOTIFICATION_CHANNEL_ID)
        builder.setContentTitle("Mesure de la lumière en cours")
        builder.setSmallIcon(android.R.drawable.ic_menu_view)
        builder.setOngoing(true)
        val notification = builder.build()

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {
            startForeground(NOTIFICATION_ID, notification, ServiceInfo.FOREGROUND_SERVICE_TYPE_SPECIAL_USE)
        } else {
            startForeground(NOTIFICATION_ID, notification)
        }
    }

    private companion object {
        const val LOG_TAG = "LuxTonCrane"
        const val NOTIFICATION_CHANNEL_ID = "mesure"
        const val NOTIFICATION_ID = 1
        const val ALARM_ACTION = "net.cawet.luxtoncrane.ALARME"
        const val INTERVALLE_ENTRE_DEUX_ALARMES_MS = 9L * 60 * 1000

        /** Garde-fou : si le capteur ne répond pas, le processeur se rendort quand même. */
        const val DURÉE_MAXIMALE_D_UNE_MESURE_MS = 10L * 1000
    }
}
