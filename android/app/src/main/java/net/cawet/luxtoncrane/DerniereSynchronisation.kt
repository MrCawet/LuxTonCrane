package net.cawet.luxtoncrane

import java.time.Instant
import java.time.ZoneId
import java.time.format.DateTimeFormatter
import java.util.Locale

private val FORMAT_DE_L_INSTANT = DateTimeFormatter.ofPattern("d MMMM yyyy 'à' HH:mm", Locale.FRENCH)

/** `instantMs` : l'instant de la dernière synchronisation réussie, ou `null` s'il n'y en a pas encore eu. */
fun texteDeLaDernièreSynchronisation(instantMs: Long?, fuseau: ZoneId): String {
    if (instantMs == null) {
        return "Aucune synchronisation réussie pour l'instant."
    }
    val moment = Instant.ofEpochMilli(instantMs).atZone(fuseau)
    return "Dernière synchronisation réussie : le ${FORMAT_DE_L_INSTANT.format(moment)}"
}
