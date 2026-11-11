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
import android.os.Handler
import android.os.IBinder
import android.os.Looper
import android.os.PowerManager
import android.os.SystemClock
import android.util.Log
import net.cawet.luxtoncrane.pont.EchecPont
import net.cawet.luxtoncrane.pont.Journal

/**
 * Service de premier plan : sans lui, Android ne transmet aucune valeur du capteur
 * à une application en arrière-plan. Écran allumé, le capteur de lumière reste ouvert ;
 * écran éteint, une alarme l'ouvre environ toutes les 9 minutes, le temps d'une mesure.
 * Il affiche la dernière mesure dans sa notification, dans les widgets et sur l'accueil.
 */
class ServiceDeMesure : Service(), SensorEventListener {

    private lateinit var journal: Journal
    private lateinit var sensorManager: SensorManager
    private lateinit var alarmManager: AlarmManager
    private lateinit var alarmIntent: PendingIntent
    private lateinit var wakeLock: PowerManager.WakeLock
    private lateinit var notificationManager: NotificationManager
    private val mainThread = Handler(Looper.getMainLooper())

    /** La mesure qui attend d'être affichée, écran allumé ; `null` si aucun affichage n'est prévu. */
    private var mesureÀAfficherLux: Float? = null

    private val afficherLaMesureEnAttente = Runnable {
        val valeurLux = mesureÀAfficherLux
        mesureÀAfficherLux = null
        if (valeurLux != null) {
            afficherLaMesure(valeurLux)
        }
    }

    private val screenAndAlarmReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context, intent: Intent) {
            when (intent.action) {
                Intent.ACTION_SCREEN_ON -> {
                    annulerLAlarme()
                    ouvrirLesCapteurs()
                }
                Intent.ACTION_SCREEN_OFF -> {
                    fermerLesCapteurs()
                    programmerLAlarme()
                }
                ALARM_ACTION -> {
                    programmerLAlarme()
                    prendreUneMesure()
                }
            }
        }
    }

    /** Si le téléphone est couvert, d'après la dernière valeur du capteur de proximité ; inconnu avant la première. */
    private var proximité: Boolean? = null

    /** Écran éteint, la première valeur de lumière attend celle de la proximité pour être notée avec elle. */
    private var lumièreEnAttente: ValeurDeLumière? = null

    /**
     * Écran éteint : note la première valeur de lumière avec la proximité,
     * puis referme les capteurs et laisse le processeur se rendormir.
     */
    private val mesureÉcranÉteint = object : SensorEventListener {
        override fun onSensorChanged(event: SensorEvent) {
            when (event.sensor.type) {
                Sensor.TYPE_PROXIMITY -> noterLaProximité(event)
                Sensor.TYPE_LIGHT -> lumièreEnAttente = valeurDeLumière(event)
            }
            val lumière = lumièreEnAttente ?: return
            val proximitéAttendue = sensorManager.getDefaultSensor(Sensor.TYPE_PROXIMITY) != null
            if (proximitéAttendue && proximité == null) {
                return
            }
            enregistrer(lumière)
            afficherLaMesure(lumière.valeurLux)
            terminerLaMesureÉcranÉteint()
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
            ouvrirLesCapteurs()
        } else {
            programmerLAlarme()
        }
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int = START_STICKY

    override fun onDestroy() {
        if (::journal.isInitialized) {
            unregisterReceiver(screenAndAlarmReceiver)
            annulerLAlarme()
            fermerLesCapteurs()
            terminerLaMesureÉcranÉteint()
            mainThread.removeCallbacks(afficherLaMesureEnAttente)
            journal.close()
        }
        super.onDestroy()
    }

    override fun onBind(intent: Intent?): IBinder? = null

    /** Écran allumé : chaque valeur de lumière est notée avec la dernière proximité connue. */
    override fun onSensorChanged(event: SensorEvent) {
        when (event.sensor.type) {
            Sensor.TYPE_PROXIMITY -> noterLaProximité(event)
            Sensor.TYPE_LIGHT -> {
                val lumière = valeurDeLumière(event)
                enregistrer(lumière)
                afficherBientôt(lumière.valeurLux)
            }
        }
    }

    override fun onAccuracyChanged(sensor: Sensor, accuracy: Int) = Unit

    private fun noterLaProximité(event: SensorEvent) {
        val distanceCm = event.values[0]
        proximité = téléphoneCouvert(distanceCm, portéeMaximaleCm = event.sensor.maximumRange)
    }

    /** Android réutilise l'objet `SensorEvent` : sa valeur et son instant sont copiés. */
    private fun valeurDeLumière(event: SensorEvent): ValeurDeLumière {
        val instantMs = instantMsDuCapteur(
            horodatageCapteurNs = event.timestamp,
            depuisDémarrageNs = SystemClock.elapsedRealtimeNanos(),
            maintenantMs = System.currentTimeMillis(),
        )
        return ValeurDeLumière(valeurLux = event.values[0], instantMs = instantMs)
    }

    private fun enregistrer(lumière: ValeurDeLumière) {
        try {
            journal.ajouter(lumière.valeurLux, lumière.instantMs, proximité)
        } catch (échec: EchecPont) {
            Log.e(LOG_TAG, "Une mesure n'a pas pu être enregistrée.", échec)
        }
    }

    /**
     * Écran allumé, le capteur peut donner plusieurs valeurs par seconde, et Android ignore les mises à jour
     * de notification trop rapprochées : l'affichage ne suit qu'une mesure par seconde au plus, la plus récente.
     */
    private fun afficherBientôt(valeurLux: Float) {
        val affichagePrévu = mesureÀAfficherLux != null
        mesureÀAfficherLux = valeurLux
        if (!affichagePrévu) {
            mainThread.postDelayed(afficherLaMesureEnAttente, DÉLAI_ENTRE_DEUX_AFFICHAGES_MS)
        }
    }

    private fun afficherLaMesure(valeurLux: Float) {
        transmettreLaDernièreMesure(this, valeurLux)
        val notification = notification(texteDeLaDernièreMesure(valeurLux))
        notificationManager.notify(NOTIFICATION_ID, notification)
    }

    private fun ouvrirLesCapteurs() {
        val capteurDeLumière = capteurDeLumière() ?: return
        val capteurDeProximité = capteurDeProximité()
        if (capteurDeProximité != null) {
            sensorManager.registerListener(this, capteurDeProximité, SensorManager.SENSOR_DELAY_NORMAL)
        }
        sensorManager.registerListener(this, capteurDeLumière, SensorManager.SENSOR_DELAY_NORMAL)
    }

    private fun fermerLesCapteurs() {
        sensorManager.unregisterListener(this)
        proximité = null
    }

    /**
     * Android ne garde le processeur éveillé que le temps de recevoir l'alarme,
     * alors que le capteur met un peu de temps à donner sa première valeur.
     */
    private fun prendreUneMesure() {
        val capteurDeLumière = capteurDeLumière() ?: return
        wakeLock.acquire(DURÉE_MAXIMALE_D_UNE_MESURE_MS)
        val capteurDeProximité = capteurDeProximité()
        if (capteurDeProximité != null) {
            sensorManager.registerListener(mesureÉcranÉteint, capteurDeProximité, SensorManager.SENSOR_DELAY_NORMAL)
        }
        sensorManager.registerListener(mesureÉcranÉteint, capteurDeLumière, SensorManager.SENSOR_DELAY_NORMAL)
    }

    private fun terminerLaMesureÉcranÉteint() {
        sensorManager.unregisterListener(mesureÉcranÉteint)
        lumièreEnAttente = null
        proximité = null
        relâcherLeProcesseur()
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

    private fun capteurDeProximité(): Sensor? {
        val capteurDeProximité = sensorManager.getDefaultSensor(Sensor.TYPE_PROXIMITY)
        if (capteurDeProximité == null) {
            Log.w(LOG_TAG, "Ce téléphone n'a pas de capteur de proximité : les mesures sont notées sans elle.")
        }
        return capteurDeProximité
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
        notificationManager = getSystemService(NotificationManager::class.java)
        notificationManager.createNotificationChannel(channel)

        val notification = notification(texteDeLaMesure = null)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {
            startForeground(NOTIFICATION_ID, notification, ServiceInfo.FOREGROUND_SERVICE_TYPE_SPECIAL_USE)
        } else {
            startForeground(NOTIFICATION_ID, notification)
        }
    }

    /** `texteDeLaMesure` : la dernière mesure, en titre, ou `null` avant la première. */
    private fun notification(texteDeLaMesure: String?): Notification {
        val builder = Notification.Builder(this, NOTIFICATION_CHANNEL_ID)
        if (texteDeLaMesure == null) {
            builder.setContentTitle("Mesure de la lumière en cours")
        } else {
            builder.setContentTitle(texteDeLaMesure)
            builder.setContentText("Mesure de la lumière en cours")
        }
        builder.setSmallIcon(android.R.drawable.ic_menu_view)
        builder.setOngoing(true)
        return builder.build()
    }

    private class ValeurDeLumière(val valeurLux: Float, val instantMs: Long)

    private companion object {
        const val LOG_TAG = "LuxTonCrane"
        const val NOTIFICATION_CHANNEL_ID = "mesure"
        const val NOTIFICATION_ID = 1
        const val ALARM_ACTION = "net.cawet.luxtoncrane.ALARME"
        const val INTERVALLE_ENTRE_DEUX_ALARMES_MS = 9L * 60 * 1000
        const val DÉLAI_ENTRE_DEUX_AFFICHAGES_MS = 1000L

        /** Garde-fou : si le capteur ne répond pas, le processeur se rendort quand même. */
        const val DURÉE_MAXIMALE_D_UNE_MESURE_MS = 10L * 1000
    }
}
