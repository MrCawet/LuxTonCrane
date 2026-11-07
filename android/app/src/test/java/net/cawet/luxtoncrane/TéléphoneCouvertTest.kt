package net.cawet.luxtoncrane

import kotlin.test.Test
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class TéléphoneCouvertTest {

    @Test
    fun `le téléphone est couvert quand la distance est sous la portée maximale du capteur`() {
        assertTrue(téléphoneCouvert(distanceCm = 0f, portéeMaximaleCm = 5f))
    }

    @Test
    fun `le téléphone n'est pas couvert quand la distance atteint la portée maximale du capteur`() {
        assertFalse(téléphoneCouvert(distanceCm = 5f, portéeMaximaleCm = 5f))
    }
}
