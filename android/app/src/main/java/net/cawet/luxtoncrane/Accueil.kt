package net.cawet.luxtoncrane

import android.Manifest
import android.app.Activity
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.text.InputType
import android.view.WindowInsets
import android.widget.Button
import android.widget.EditText
import android.widget.LinearLayout
import android.widget.NumberPicker
import android.widget.ScrollView
import android.widget.TextView
import java.time.LocalDate
import java.time.LocalTime
import java.time.ZoneId
import kotlin.math.roundToInt
import net.cawet.luxtoncrane.pont.EchecPont
import net.cawet.luxtoncrane.pont.Journal
import net.cawet.luxtoncrane.pont.LumiereAEviter
import net.cawet.luxtoncrane.pont.ReleveDuJour

/**
 * Demande la permission d'afficher la notification, puis démarre la mesure.
 * Affiche le relevé du jour pour les heures du coucher, du lever et du coucher souhaité choisies sur les roues.
 */
class Accueil : Activity() {

    private lateinit var roueDuCoucher: NumberPicker
    private lateinit var roueDuLever: NumberPicker
    private lateinit var roueDuCoucherSouhaité: NumberPicker
    private lateinit var affichageDuRelevé: TextView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(screen())

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

    private fun screen(): ScrollView {
        val column = LinearLayout(this)
        column.orientation = LinearLayout.VERTICAL

        val message = TextView(this)
        message.text = "Mesure de la lumière en cours, écran allumé."
        column.addView(message)

        roueDuCoucher = roue("Heure du coucher", LocalTime.of(23, 0), column)
        roueDuLever = roue("Heure du lever", LocalTime.of(7, 0), column)
        roueDuCoucherSouhaité = roue("Heure du coucher souhaitée", LocalTime.of(23, 0), column)

        val bouton = Button(this)
        bouton.text = "Voir le relevé du jour"
        bouton.setOnClickListener { afficherLeRelevé() }
        column.addView(bouton)

        affichageDuRelevé = TextView(this)
        column.addView(affichageDuRelevé)

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

    /** Une roue qui se déroule par pas de 15 min, sur toute la journée. */
    private fun roue(titre: String, heureDeDépart: LocalTime, column: LinearLayout): NumberPicker {
        val étiquette = TextView(this)
        étiquette.text = titre
        column.addView(étiquette)

        val heures = Array(PAS_PAR_JOUR) { pas -> heureDuPas(pas).toString() }
        val roue = NumberPicker(this)
        roue.minValue = 0
        roue.maxValue = PAS_PAR_JOUR - 1
        roue.displayedValues = heures
        roue.value = heureDeDépart.toSecondOfDay() / SECONDES_PAR_PAS
        column.addView(roue)
        return roue
    }

    private fun heureDuPas(pas: Int): LocalTime = LocalTime.ofSecondOfDay((pas * SECONDES_PAR_PAS).toLong())

    private fun afficherLeRelevé() {
        val heures = heuresDuRelevé(
            coucher = heureDuPas(roueDuCoucher.value),
            lever = heureDuPas(roueDuLever.value),
            coucherSouhaité = heureDuPas(roueDuCoucherSouhaité.value),
            aujourdhui = LocalDate.now(),
            fuseau = ZoneId.systemDefault(),
        )
        try {
            Journal.ouvrir(cheminDuJournal(this)).use { journal ->
                val relevé = journal.releveDuJour(heures.coucherMs, heures.leverMs, heures.coucherSouhaitéMs)
                affichageDuRelevé.text = texteDuRelevé(relevé)
            }
        } catch (échec: EchecPont) {
            affichageDuRelevé.text = "Le relevé n'a pas pu être lu : ${échec.message}"
        }
    }

    private fun texteDuRelevé(relevé: ReleveDuJour): String {
        val matin = relevé.lumiereDuMatin
        val journée = relevé.lumiereDeLaJournee
        val partÀ250Lx = journée.partDuTempsMesureA250LxOuPlus
        val partÀ250LxEnTexte = if (partÀ250Lx == null) "rien de mesuré" else pourcentage(partÀ250Lx)
        return listOf(
            "Soir, 3 h avant le coucher",
            texteDeLaLumièreÀÉviter(relevé.lumiereDuSoir, "10 lx"),
            "",
            "Nuit, du coucher au lever",
            texteDeLaLumièreÀÉviter(relevé.lumiereDeLaNuit, "1 lx"),
            "",
            "Matin, 2 h après le lever",
            "À 10 000 lx ou plus : ${minutes(matin.tempsA10000LxOuPlusMs)}",
            "Objectif tenu : ${ouiOuNon(matin.objectifTenu)}",
            "Couverture : ${pourcentage(matin.couverture)}",
            "",
            "Journée, du lever à 3 h avant le coucher souhaité",
            "Temps mesuré à 250 lx ou plus : $partÀ250LxEnTexte",
            "Objectif tenu : ${ouiOuNon(journée.objectifTenu)}",
            "Couverture : ${pourcentage(journée.couverture)}",
        ).joinToString("\n")
    }

    private fun texteDeLaLumièreÀÉviter(lumière: LumiereAEviter, seuil: String): String {
        val valeurLaPlusHaute = lumière.valeurLaPlusHauteLux
        val valeurLaPlusHauteEnTexte = if (valeurLaPlusHaute == null) "rien de mesuré" else "$valeurLaPlusHaute lx"
        return listOf(
            "Au-dessus de $seuil : ${minutes(lumière.tempsAuDessusDuSeuilMs)}",
            "Valeur la plus haute : $valeurLaPlusHauteEnTexte",
            "Objectif tenu : ${ouiOuNon(lumière.objectifTenu)}",
            "Couverture : ${pourcentage(lumière.couverture)}",
        ).joinToString("\n")
    }

    private fun minutes(duréeMs: Long): String = "${duréeMs / 60_000} min"

    private fun pourcentage(part: Double): String = "${(part * 100).roundToInt()} %"

    private fun ouiOuNon(tenu: Boolean): String = if (tenu) "oui" else "non"

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
        bouton.text = "Garder le jeton"
        column.addView(bouton)

        étiquette.text = étatDuJeton()
        bouton.setOnClickListener {
            val jeton = champ.text.toString().trim()
            if (jeton.isNotEmpty()) {
                garderLeJetonDuServeur(this, jeton)
                champ.text.clear()
                étiquette.text = étatDuJeton()
            }
        }
    }

    private fun étatDuJeton(): String {
        val jetonGardé = jetonDuServeur(this) != null
        if (jetonGardé) {
            return "Jeton du serveur gardé : les mesures sont envoyées."
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
        const val SECONDES_PAR_PAS = 15 * 60
        const val PAS_PAR_JOUR = 24 * 60 / 15
    }
}
