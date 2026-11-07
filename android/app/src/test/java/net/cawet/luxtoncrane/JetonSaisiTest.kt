package net.cawet.luxtoncrane

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull

class JetonSaisiTest {

    @Test
    fun `un jeton saisi est gardé sans les espaces autour`() {
        assertEquals("abc-123_XYZ", jetonSaisi("  abc-123_XYZ\n"))
    }

    @Test
    fun `un jeton vide n'est pas gardé`() {
        assertNull(jetonSaisi("   "))
    }

    @Test
    fun `un jeton avec un espace, un accent ou un caractère invisible n'est pas gardé`() {
        assertNull(jetonSaisi("abc 123"))
        assertNull(jetonSaisi("abcé123"))
        assertNull(jetonSaisi("abc​123"))
    }
}
