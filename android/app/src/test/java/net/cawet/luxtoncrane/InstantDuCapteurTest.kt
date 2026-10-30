package net.cawet.luxtoncrane

import kotlin.test.Test
import kotlin.test.assertEquals

class InstantDuCapteurTest {

    @Test
    fun `l'instant d'une valeur du capteur est l'heure actuelle moins son ancienneté`() {
        val horodatageCapteurNs = 5_000_000_000L
        val depuisDémarrageNs = 5_250_000_000L
        val maintenantMs = 1_793_000_000_250L

        val instantMs = instantMsDuCapteur(horodatageCapteurNs, depuisDémarrageNs, maintenantMs)

        assertEquals(1_793_000_000_000L, instantMs)
    }
}
