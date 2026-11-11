package net.cawet.luxtoncrane

import kotlin.test.Test
import kotlin.test.assertEquals

class DerniereMesureTest {

    @Test
    fun `sous 10 lx, la dernière mesure s'écrit avec une décimale`() {
        assertEquals("0,4 lx", texteDeLaDernièreMesure(valeurLux = 0.42f))
    }

    @Test
    fun `à partir de 10 lx, la dernière mesure s'écrit en entier`() {
        assertEquals("320 lx", texteDeLaDernièreMesure(valeurLux = 320.4f))
    }

    @Test
    fun `les milliers de la dernière mesure sont séparés par une espace`() {
        assertEquals("12 481 lx", texteDeLaDernièreMesure(valeurLux = 12480.7f))
    }
}
