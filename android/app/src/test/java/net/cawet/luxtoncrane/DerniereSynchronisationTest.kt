package net.cawet.luxtoncrane

import java.time.ZoneId
import java.time.ZonedDateTime
import kotlin.test.Test
import kotlin.test.assertEquals

class DerniereSynchronisationTest {

    private val fuseau = ZoneId.of("Europe/Paris")

    @Test
    fun `la dernière synchronisation réussie se lit en date et heure locales`() {
        val instantMs = ZonedDateTime.of(2026, 11, 7, 15, 40, 0, 0, fuseau).toInstant().toEpochMilli()

        val texte = texteDeLaDernièreSynchronisation(instantMs, fuseau)

        assertEquals("Dernière synchronisation réussie : le 7 novembre 2026 à 15:40", texte)
    }

    @Test
    fun `sans synchronisation réussie, l'accueil le dit`() {
        val texte = texteDeLaDernièreSynchronisation(null, fuseau)

        assertEquals("Aucune synchronisation réussie pour l'instant.", texte)
    }
}
