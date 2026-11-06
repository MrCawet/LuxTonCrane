package net.cawet.luxtoncrane

/** Une mesure du journal, telle qu'elle part au serveur. */
data class MesureÀEnvoyer(val valeurLux: Float, val instantMs: Long)

/** Le serveur qui reçoit les mesures du téléphone. */
interface ServeurDesMesures {
    /** L'instant de la dernière mesure que le serveur a reçue, ou `null` s'il n'en a reçu aucune. */
    fun dernierInstantReçuMs(): Long?

    fun envoyer(lot: List<MesureÀEnvoyer>)
}

/**
 * Envoie au serveur les mesures qu'il n'a pas encore reçues.
 * `mesuresAprès` rend les mesures du journal prises après l'instant donné, toutes si l'instant est `null`.
 */
fun synchroniser(mesuresAprès: (Long?) -> List<MesureÀEnvoyer>, serveur: ServeurDesMesures) {
}
