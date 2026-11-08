package net.cawet.luxtoncrane

import java.time.Instant
import java.time.LocalTime
import java.time.ZoneId

/** Un rappel de l'agenda du sommeil, à heure fixe, à l'heure locale du téléphone. */
enum class RappelDeLAgenda(val heure: LocalTime, val texte: String) {
    MATIN(LocalTime.of(8, 0), "Note ta nuit dans l'agenda du sommeil."),
    SOIR(LocalTime.of(21, 0), "Note la fatigue de ta journée dans l'agenda du sommeil."),
}

/**
 * Le prochain instant du rappel : aujourd'hui s'il est encore à venir, sinon demain.
 * À l'heure pile, c'est demain : le rappel qui vient de sonner se replanifie sans sonner une deuxième fois.
 */
fun instantDuProchainRappelMs(rappel: RappelDeLAgenda, maintenantMs: Long, fuseau: ZoneId): Long {
    val maintenant = Instant.ofEpochMilli(maintenantMs).atZone(fuseau)
    val aujourdhui = maintenant.toLocalDate()
    val rappelDuJour = aujourdhui.atTime(rappel.heure).atZone(fuseau)
    if (rappelDuJour.isAfter(maintenant)) {
        return rappelDuJour.toInstant().toEpochMilli()
    }
    val demain = aujourdhui.plusDays(1)
    val rappelDeDemain = demain.atTime(rappel.heure).atZone(fuseau)
    return rappelDeDemain.toInstant().toEpochMilli()
}
