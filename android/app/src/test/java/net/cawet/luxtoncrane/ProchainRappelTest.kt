package net.cawet.luxtoncrane

import java.time.ZoneId
import java.time.ZonedDateTime
import kotlin.test.Test
import kotlin.test.assertEquals

class ProchainRappelTest {

    private val fuseau = ZoneId.of("Europe/Paris")

    private fun instantMs(jour: Int, heure: Int, minute: Int): Long =
        ZonedDateTime.of(2026, 11, jour, heure, minute, 0, 0, fuseau).toInstant().toEpochMilli()

    @Test
    fun `avant 8 h, le rappel du matin sonne à 8 h le jour même`() {
        val maintenantMs = instantMs(9, 6, 30)

        val prochainRappelMs = instantDuProchainRappelMs(RappelDeLAgenda.MATIN, maintenantMs, fuseau)

        assertEquals(instantMs(9, 8, 0), prochainRappelMs)
    }

    @Test
    fun `après 8 h, le rappel du matin sonne le lendemain à 8 h`() {
        val maintenantMs = instantMs(9, 10, 15)

        val prochainRappelMs = instantDuProchainRappelMs(RappelDeLAgenda.MATIN, maintenantMs, fuseau)

        assertEquals(instantMs(10, 8, 0), prochainRappelMs)
    }

    @Test
    fun `à 21 h pile, le rappel du soir qui vient de sonner est replanifié le lendemain`() {
        val maintenantMs = instantMs(9, 21, 0)

        val prochainRappelMs = instantDuProchainRappelMs(RappelDeLAgenda.SOIR, maintenantMs, fuseau)

        assertEquals(instantMs(10, 21, 0), prochainRappelMs)
    }
}
