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

class DernierEchecTest {

    private val fuseau = ZoneId.of("Europe/Paris")
    private val instantMs = ZonedDateTime.of(2026, 11, 7, 18, 14, 0, 0, fuseau).toInstant().toEpochMilli()

    @Test
    fun `un jeton refusé demande de le saisir à nouveau`() {
        val échec = ÉchecDeSynchronisation(instantMs, RaisonDeLÉchec.JETON_REFUSÉ)

        val texte = texteDuDernierÉchec(échec, fuseau)

        assertEquals(
            "Échec le 7 novembre 2026 à 18:14 : le serveur refuse le jeton, saisis-le à nouveau.",
            texte,
        )
    }

    @Test
    fun `un serveur injoignable sera retenté`() {
        val échec = ÉchecDeSynchronisation(instantMs, RaisonDeLÉchec.SERVEUR_INJOIGNABLE)

        val texte = texteDuDernierÉchec(échec, fuseau)

        assertEquals(
            "Échec le 7 novembre 2026 à 18:14 : le serveur n'a pas répondu comme prévu, nouvel essai plus tard.",
            texte,
        )
    }

    @Test
    fun `sans échec depuis la dernière réussite, rien n'est affiché`() {
        assertEquals(null, texteDuDernierÉchec(null, fuseau))
    }
}
