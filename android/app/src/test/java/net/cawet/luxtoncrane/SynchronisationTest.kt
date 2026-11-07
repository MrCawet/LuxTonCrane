package net.cawet.luxtoncrane

import kotlin.test.Test
import kotlin.test.assertEquals

class SynchronisationTest {

    /** Doublure du serveur : garde les lots reçus en mémoire. */
    private class ServeurEnMémoire(private val dernierInstantReçuMs: Long?) : ServeurDesMesures {
        val lotsReçus = mutableListOf<List<MesureÀEnvoyer>>()

        override fun dernierInstantReçuMs(): Long? = dernierInstantReçuMs

        override fun envoyer(lot: List<MesureÀEnvoyer>) {
            lotsReçus.add(lot)
        }
    }

    private val premierInstantMs = 1_793_000_000_000

    private fun mesures(nombre: Int): List<MesureÀEnvoyer> =
        (1..nombre).map { rang -> MesureÀEnvoyer(valeurLux = 100f, instantMs = premierInstantMs + rang * 1_000L, téléphoneCouvert = null) }

    @Test
    fun `les mesures d'après la dernière reçue partent au serveur par lots de 1000`() {
        val serveur = ServeurEnMémoire(dernierInstantReçuMs = premierInstantMs)
        val àEnvoyer = mesures(1_500)
        var instantDemandéAuJournal: Long? = null

        synchroniser(
            mesuresAprès = { instantMs ->
                instantDemandéAuJournal = instantMs
                àEnvoyer
            },
            serveur = serveur,
        )

        assertEquals(premierInstantMs, instantDemandéAuJournal)
        assertEquals(listOf(1_000, 500), serveur.lotsReçus.map { lot -> lot.size })
        assertEquals(àEnvoyer, serveur.lotsReçus.flatten())
    }

    @Test
    fun `rien ne part quand le serveur a déjà tout reçu`() {
        val serveur = ServeurEnMémoire(dernierInstantReçuMs = premierInstantMs)

        synchroniser(mesuresAprès = { emptyList() }, serveur = serveur)

        assertEquals(emptyList(), serveur.lotsReçus)
    }
}
