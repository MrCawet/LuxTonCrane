package net.cawet.luxtoncrane

import java.time.LocalDate
import java.time.LocalTime
import java.time.ZoneId
import java.time.ZonedDateTime

/** Les heures du relevé du jour, en millisecondes depuis le 1er janvier 1970 à minuit (UTC). */
data class HeuresDuRelevé(val coucherMs: Long, val leverMs: Long, val coucherSouhaitéMs: Long)

/**
 * Les roues ne donnent qu'une heure de la journée : chacune est placée autour du lever d'aujourd'hui.
 * Le jour et le fuseau sont lus par l'appelant, pour que ce calcul reste testable.
 */
fun heuresDuRelevé(
    coucher: LocalTime,
    lever: LocalTime,
    coucherSouhaité: LocalTime,
    aujourdhui: LocalDate,
    fuseau: ZoneId,
): HeuresDuRelevé {
    val jourDuCoucher = if (coucher.isBefore(lever)) aujourdhui else aujourdhui.minusDays(1)
    val jourDuCoucherSouhaité = if (coucherSouhaité.isAfter(lever)) aujourdhui else aujourdhui.plusDays(1)
    return HeuresDuRelevé(
        coucherMs = instantMs(jourDuCoucher, coucher, fuseau),
        leverMs = instantMs(aujourdhui, lever, fuseau),
        coucherSouhaitéMs = instantMs(jourDuCoucherSouhaité, coucherSouhaité, fuseau),
    )
}

private fun instantMs(jour: LocalDate, heure: LocalTime, fuseau: ZoneId): Long {
    val moment = ZonedDateTime.of(jour, heure, fuseau)
    return moment.toInstant().toEpochMilli()
}
