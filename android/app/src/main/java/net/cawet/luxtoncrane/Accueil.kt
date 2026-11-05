package net.cawet.luxtoncrane

import android.Manifest
import android.app.Activity
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.view.Gravity
import android.view.ViewGroup
import android.widget.Button
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

        // Côte à côte, les roues tiennent sur l'écran : sinon elles captent le glissement du doigt
        // et la page ne défile plus.
        val roues = LinearLayout(this)
        roues.orientation = LinearLayout.HORIZONTAL
        roueDuCoucher = roue("Coucher", LocalTime.of(23, 0), roues)
        roueDuLever = roue("Lever", LocalTime.of(7, 0), roues)
        roueDuCoucherSouhaité = roue("Coucher souhaité", LocalTime.of(23, 0), roues)
        column.addView(roues)

        val bouton = Button(this)
        bouton.text = "Voir le relevé du jour"
        bouton.setOnClickListener { afficherLeRelevé() }
        column.addView(bouton)

        affichageDuRelevé = TextView(this)
        column.addView(affichageDuRelevé)

        val screen = ScrollView(this)
        screen.addView(column)
        return screen
    }

    /** Une roue qui se déroule par pas de 15 min, sur toute la journée, sous son titre. */
    private fun roue(titre: String, heureDeDépart: LocalTime, roues: LinearLayout): NumberPicker {
        val colonne = LinearLayout(this)
        colonne.orientation = LinearLayout.VERTICAL
        colonne.gravity = Gravity.CENTER_HORIZONTAL

        val étiquette = TextView(this)
        étiquette.text = titre
        colonne.addView(étiquette)

        val heures = Array(PAS_PAR_JOUR) { pas -> heureDuPas(pas).toString() }
        val roue = NumberPicker(this)
        roue.minValue = 0
        roue.maxValue = PAS_PAR_JOUR - 1
        roue.displayedValues = heures
        roue.value = heureDeDépart.toSecondOfDay() / SECONDES_PAR_PAS
        // Sans cela, la roue prend le focus à l'ouverture et la page défile jusqu'à elle.
        roue.descendantFocusability = ViewGroup.FOCUS_BLOCK_DESCENDANTS
        colonne.addView(roue)

        val sameWidthForEachWheel = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f)
        roues.addView(colonne, sameWidthForEachWheel)
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

    private fun démarrerLaMesure() {
        val service = Intent(this, ServiceDeMesure::class.java)
        startForegroundService(service)
    }

    private companion object {
        const val NOTIFICATION_PERMISSION_REQUEST = 1
        const val SECONDES_PAR_PAS = 15 * 60
        const val PAS_PAR_JOUR = 24 * 60 / 15
    }
}
