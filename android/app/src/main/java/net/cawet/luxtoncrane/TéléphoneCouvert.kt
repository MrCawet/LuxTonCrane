package net.cawet.luxtoncrane

/**
 * Beaucoup de capteurs de proximité ne donnent que deux valeurs : 0 et leur portée maximale.
 * Sous cette portée, quelque chose est contre le téléphone.
 */
fun téléphoneCouvert(distanceCm: Float, portéeMaximaleCm: Float): Boolean = distanceCm < portéeMaximaleCm
