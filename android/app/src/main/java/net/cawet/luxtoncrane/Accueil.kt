package net.cawet.luxtoncrane

import android.Manifest
import android.app.Activity
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.widget.TextView

/** Demande la permission d'afficher la notification, puis démarre la mesure. */
class Accueil : Activity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val message = TextView(this)
        message.text = "Mesure de la lumière en cours, écran allumé."
        setContentView(message)

        if (notificationPermissionIsMissing()) {
            requestPermissions(arrayOf(Manifest.permission.POST_NOTIFICATIONS), NOTIFICATION_PERMISSION_REQUEST)
        } else {
            démarrerLaMesure()
        }
    }

    override fun onRequestPermissionsResult(requestCode: Int, permissions: Array<out String>, grantResults: IntArray) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        if (requestCode == NOTIFICATION_PERMISSION_REQUEST) {
            démarrerLaMesure()
        }
    }

    private fun notificationPermissionIsMissing(): Boolean {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
            return false
        }
        val permission = checkSelfPermission(Manifest.permission.POST_NOTIFICATIONS)
        return permission != PackageManager.PERMISSION_GRANTED
    }

    private fun démarrerLaMesure() {
        val service = Intent(this, ServiceDeMesure::class.java)
        startForegroundService(service)
    }

    private companion object {
        const val NOTIFICATION_PERMISSION_REQUEST = 1
    }
}
