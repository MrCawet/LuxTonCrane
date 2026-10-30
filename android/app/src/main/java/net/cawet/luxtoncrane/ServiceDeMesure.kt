package net.cawet.luxtoncrane

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
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
import java.io.File
import net.cawet.luxtoncrane.pont.EchecPont
import net.cawet.luxtoncrane.pont.Journal

/**
 * Service de premier plan : sans lui, Android ne transmet aucune valeur du capteur
 * à une application en arrière-plan. Le capteur de lumière n'est ouvert que quand l'écran est allumé.
 */
class ServiceDeMesure : Service(), SensorEventListener {

    private lateinit var journal: Journal
    private lateinit var sensorManager: SensorManager

    private val screenReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context, intent: Intent) {
            when (intent.action) {
                Intent.ACTION_SCREEN_ON -> ouvrirLeCapteur()
                Intent.ACTION_SCREEN_OFF -> fermerLeCapteur()
            }
        }
    }

    override fun onCreate() {
        super.onCreate()
        startForegroundWithNotification()

        val cheminDuJournal = File(filesDir, JOURNAL_FILE_NAME).path
        try {
            journal = Journal.ouvrir(cheminDuJournal)
        } catch (échec: EchecPont) {
            Log.e(LOG_TAG, "Le journal n'a pas pu être ouvert, la mesure s'arrête.", échec)
            stopSelf()
            return
        }

        sensorManager = getSystemService(SensorManager::class.java)
        val screenEvents = IntentFilter()
        screenEvents.addAction(Intent.ACTION_SCREEN_ON)
        screenEvents.addAction(Intent.ACTION_SCREEN_OFF)
        registerReceiver(screenReceiver, screenEvents)

        val powerManager = getSystemService(PowerManager::class.java)
        if (powerManager.isInteractive) {
            ouvrirLeCapteur()
        }
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int = START_STICKY

    override fun onDestroy() {
        if (::journal.isInitialized) {
            unregisterReceiver(screenReceiver)
            fermerLeCapteur()
            journal.close()
        }
        super.onDestroy()
    }

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onSensorChanged(event: SensorEvent) {
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

    override fun onAccuracyChanged(sensor: Sensor, accuracy: Int) = Unit

    private fun ouvrirLeCapteur() {
        val capteurDeLumière = sensorManager.getDefaultSensor(Sensor.TYPE_LIGHT)
        if (capteurDeLumière == null) {
            Log.e(LOG_TAG, "Ce téléphone n'a pas de capteur de lumière.")
            return
        }
        sensorManager.registerListener(this, capteurDeLumière, SensorManager.SENSOR_DELAY_NORMAL)
    }

    private fun fermerLeCapteur() {
        sensorManager.unregisterListener(this)
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
        const val JOURNAL_FILE_NAME = "journal.sqlite"
        const val NOTIFICATION_CHANNEL_ID = "mesure"
        const val NOTIFICATION_ID = 1
    }
}
