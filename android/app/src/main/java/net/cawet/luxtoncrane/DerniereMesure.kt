package net.cawet.luxtoncrane

import java.text.NumberFormat
import java.util.Locale

/** Sous 10 lx, une décimale : l'objectif de la nuit, au plus 1 lx, se lit finement. */
fun texteDeLaDernièreMesure(valeurLux: Float): String {
    val format = NumberFormat.getNumberInstance(Locale.FRENCH)
    val décimales = if (valeurLux < 10f) 1 else 0
    format.minimumFractionDigits = décimales
    format.maximumFractionDigits = décimales
    return "${format.format(valeurLux)} lx"
}
