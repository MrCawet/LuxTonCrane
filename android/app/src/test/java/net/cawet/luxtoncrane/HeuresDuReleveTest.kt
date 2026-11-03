package net.cawet.luxtoncrane

import java.time.LocalDate
import java.time.LocalTime
import java.time.ZoneId
import java.time.ZonedDateTime
import kotlin.test.Test
import kotlin.test.assertEquals

class HeuresDuReleveTest {

    private val fuseau = ZoneId.of("Europe/Paris")
    private val aujourdhui = LocalDate.of(2026, 11, 3)

    private fun instantMs(jour: LocalDate, heure: LocalTime): Long {
        val moment = ZonedDateTime.of(jour, heure, fuseau)
        return moment.toInstant().toEpochMilli()
    }

    @Test
    fun `le coucher est la veille du lever, et le coucher souhaité le soir du lever`() {
        val heures = heuresDuRelevé(
            coucher = LocalTime.of(23, 0),
            lever = LocalTime.of(7, 0),
            coucherSouhaité = LocalTime.of(22, 30),
            aujourdhui = aujourdhui,
            fuseau = fuseau,
        )

        val attendues = HeuresDuRelevé(
            coucherMs = instantMs(aujourdhui.minusDays(1), LocalTime.of(23, 0)),
            leverMs = instantMs(aujourdhui, LocalTime.of(7, 0)),
            coucherSouhaitéMs = instantMs(aujourdhui, LocalTime.of(22, 30)),
        )
        assertEquals(attendues, heures)
    }

    @Test
    fun `un coucher après minuit est le jour même du lever`() {
        val heures = heuresDuRelevé(
            coucher = LocalTime.of(0, 30),
            lever = LocalTime.of(7, 0),
            coucherSouhaité = LocalTime.of(22, 30),
            aujourdhui = aujourdhui,
            fuseau = fuseau,
        )

        assertEquals(instantMs(aujourdhui, LocalTime.of(0, 30)), heures.coucherMs)
    }

    @Test
    fun `un coucher souhaité après minuit est le lendemain du lever`() {
        val heures = heuresDuRelevé(
            coucher = LocalTime.of(23, 0),
            lever = LocalTime.of(7, 0),
            coucherSouhaité = LocalTime.of(0, 15),
            aujourdhui = aujourdhui,
            fuseau = fuseau,
        )

        assertEquals(instantMs(aujourdhui.plusDays(1), LocalTime.of(0, 15)), heures.coucherSouhaitéMs)
    }
}
