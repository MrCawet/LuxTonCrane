package net.cawet.luxtoncrane

import android.Manifest
import android.app.Activity
import android.content.Intent
import android.content.SharedPreferences
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.graphics.Color
import android.text.InputType
import android.util.TypedValue
import android.view.Gravity
import android.view.View
import android.view.WindowInsets
import android.widget.Button
import android.widget.EditText
import android.widget.LinearLayout
import android.widget.ScrollView
import android.widget.TextView
import java.time.ZoneId

/**
 * Demande la permission d'afficher les notifications, puis démarre la mesure.
 * Planifie les rappels de l'agenda du sommeil, qui ouvrent la page web.
 * Affiche la dernière mesure en énorme, et la dernière synchronisation réussie avec le serveur, et permet de synchroniser tout de suite.
 * Le relevé du jour se lit sur la page web.
 */
class Accueil : Activity() {

    private lateinit var affichageDeLaDernièreMesure: TextView
    private lateinit var affichageDeLaSynchronisation: TextView
    private lateinit var affichageDuDernierÉchec: TextView

    // Gardé dans un champ : les préférences ne retiennent ce qui les écoute que par une référence faible.
    private val suiviDeLaSynchronisation = SharedPreferences.OnSharedPreferenceChangeListener { _, clé ->
        if (clé == LAST_SUCCESS_KEY || clé == LAST_FAILURE_TIME_KEY) {
            afficherLaDernièreSynchronisation()
        }
    }

    // Gardé dans un champ, pour la même raison.
    private val suiviDeLaDernièreMesure = SharedPreferences.OnSharedPreferenceChangeListener { _, clé ->
        if (clé == LAST_MEASURE_KEY) {
            afficherLaDernièreMesure()
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(screen())
        préférencesDeLaSynchronisation(this).registerOnSharedPreferenceChangeListener(suiviDeLaSynchronisation)
        préférencesDeLaDernièreMesure(this).registerOnSharedPreferenceChangeListener(suiviDeLaDernièreMesure)
        planifierLesRappelsDeLAgenda(this)

        if (notificationPermissionIsMissing()) {
            requestPermissions(arrayOf(Manifest.permission.POST_NOTIFICATIONS), NOTIFICATION_PERMISSION_REQUEST)
        } else {
            démarrerLaMesure()
        }
    }

    override fun onResume() {
        super.onResume()
        afficherLaDernièreMesure()
        afficherLaDernièreSynchronisation()
    }

    override fun onDestroy() {
        préférencesDeLaSynchronisation(this).unregisterOnSharedPreferenceChangeListener(suiviDeLaSynchronisation)
        préférencesDeLaDernièreMesure(this).unregisterOnSharedPreferenceChangeListener(suiviDeLaDernièreMesure)
        super.onDestroy()
    }

    override fun onRequestPermissionsResult(requestCode: Int, permissions: Array<out String>, grantResults: IntArray) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        if (requestCode == NOTIFICATION_PERMISSION_REQUEST) {
            démarrerLaMesure()
        }
    }

    private fun screen(): ScrollView {
        val column = LinearLayout(this)
        column.orientation = LinearLayout.VERTICAL

        affichageDeLaDernièreMesure = TextView(this)
        affichageDeLaDernièreMesure.setTextSize(TypedValue.COMPLEX_UNIT_SP, 72f)
        affichageDeLaDernièreMesure.gravity = Gravity.CENTER
        column.addView(affichageDeLaDernièreMesure)

        val message = TextView(this)
        message.text = "Mesure de la lumière en cours, écran allumé."
        column.addView(message)

        affichageDeLaSynchronisation = TextView(this)
        column.addView(affichageDeLaSynchronisation)

        affichageDuDernierÉchec = TextView(this)
        affichageDuDernierÉchec.setTextColor(Color.rgb(176, 0, 32))
        column.addView(affichageDuDernierÉchec)

        val bouton = Button(this)
        bouton.text = "Synchroniser maintenant"
        bouton.setOnClickListener { demanderLaSynchronisation() }
        column.addView(bouton)

        champDuJeton(column)

        val screen = ScrollView(this)
        screen.addView(column)
        keepAwayFromSystemBars(screen)
        return screen
    }

    /**
     * Depuis Android 15, l'application dessine sous la barre d'état et la barre de navigation :
     * sans cet écart, le haut de la page est caché et ne peut pas être atteint.
     */
    private fun keepAwayFromSystemBars(screen: ScrollView) {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.VANILLA_ICE_CREAM) {
            return
        }
        screen.setOnApplyWindowInsetsListener { view, insets ->
            val systemBars = insets.getInsets(WindowInsets.Type.systemBars())
            view.setPadding(systemBars.left, systemBars.top, systemBars.right, systemBars.bottom)
            insets
        }
    }

    private fun afficherLaDernièreMesure() {
        val valeurLux = dernièreMesureLux(this)
        affichageDeLaDernièreMesure.text = if (valeurLux == null) "— lx" else texteDeLaDernièreMesure(valeurLux)
    }

    private fun afficherLaDernièreSynchronisation() {
        val instantMs = dernièreSynchronisationRéussieMs(this)
        affichageDeLaSynchronisation.text = texteDeLaDernièreSynchronisation(instantMs, ZoneId.systemDefault())
        val texteDeLÉchec = texteDuDernierÉchec(dernierÉchecDeSynchronisation(this), ZoneId.systemDefault())
        affichageDuDernierÉchec.text = texteDeLÉchec ?: ""
        affichageDuDernierÉchec.visibility = if (texteDeLÉchec == null) View.GONE else View.VISIBLE
    }

    private fun demanderLaSynchronisation() {
        if (jetonDuServeur(this) == null) {
            affichageDeLaSynchronisation.text = "Aucun jeton du serveur : saisis-le d'abord, plus bas."
            return
        }
        synchroniserMaintenant(this)
        affichageDeLaSynchronisation.text = "Synchronisation demandée, elle part dès que le réseau est là."
    }

    private fun notificationPermissionIsMissing(): Boolean {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
            return false
        }
        val permission = checkSelfPermission(Manifest.permission.POST_NOTIFICATIONS)
        return permission != PackageManager.PERMISSION_GRANTED
    }

    /** Le jeton du serveur se saisit une fois ; le champ reste vide ensuite pour ne pas l'afficher. */
    private fun champDuJeton(column: LinearLayout) {
        val étiquette = TextView(this)
        column.addView(étiquette)

        val champ = EditText(this)
        champ.hint = "Jeton du serveur"
        champ.inputType = InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_VARIATION_PASSWORD
        column.addView(champ)

        val bouton = Button(this)
        bouton.text = "Enregistrer"
        column.addView(bouton)

        étiquette.text = étatDuJeton()
        bouton.setOnClickListener {
            val jeton = jetonSaisi(champ.text.toString())
            if (jeton == null) {
                étiquette.text = "Ce n'est pas un jeton : il ne contient ni espace, ni accent. Il n'est pas enregistré."
                return@setOnClickListener
            }
            garderLeJetonDuServeur(this, jeton)
            champ.text.clear()
            étiquette.text = "Nouveau jeton enregistré."
            demanderLaSynchronisation()
        }
    }

    private fun étatDuJeton(): String {
        val jetonGardé = jetonDuServeur(this) != null
        if (jetonGardé) {
            return "Jeton du serveur enregistré."
        }
        return "Aucun jeton du serveur : les mesures ne sont pas envoyées."
    }

    private fun démarrerLaMesure() {
        val service = Intent(this, ServiceDeMesure::class.java)
        startForegroundService(service)
        planifierLaSynchronisation(this)
    }

    private companion object {
        const val NOTIFICATION_PERMISSION_REQUEST = 1
    }
}
