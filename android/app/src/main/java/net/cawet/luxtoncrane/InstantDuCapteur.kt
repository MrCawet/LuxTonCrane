package net.cawet.luxtoncrane

/**
 * Le capteur date ses valeurs depuis le démarrage du téléphone ; le journal attend une heure réelle.
 * Les deux horloges sont lues par l'appelant, pour que ce calcul reste testable.
 */
fun instantMsDuCapteur(horodatageCapteurNs: Long, depuisDémarrageNs: Long, maintenantMs: Long): Long {
    val anciennetéNs = depuisDémarrageNs - horodatageCapteurNs
    val anciennetéMs = anciennetéNs / 1_000_000
    return maintenantMs - anciennetéMs
}
