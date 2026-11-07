package net.cawet.luxtoncrane

// Un lot de 1000 mesures fait environ 50 ko de JSON : il reste sous la taille de requête qu'accepte nginx par défaut (1 Mo).
private const val TAILLE_D_UN_LOT = 1_000

/** Une mesure du journal, telle qu'elle part au serveur. `téléphoneCouvert` est `null` si la proximité est inconnue. */
data class MesureÀEnvoyer(val valeurLux: Float, val instantMs: Long, val téléphoneCouvert: Boolean?)

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
    val dernierInstantReçuMs = serveur.dernierInstantReçuMs()
    val àEnvoyer = mesuresAprès(dernierInstantReçuMs)
    val lots = àEnvoyer.chunked(TAILLE_D_UN_LOT)
    for (lot in lots) {
        serveur.envoyer(lot)
    }
}
