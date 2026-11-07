package net.cawet.luxtoncrane

/**
 * Le jeton à garder, sans les espaces autour, ou `null` si la saisie n'en est pas un.
 * Un jeton ne contient que des caractères ASCII visibles : tout autre caractère rendrait l'en-tête HTTP illisible.
 */
fun jetonSaisi(texte: String): String? {
    val jeton = texte.trim()
    if (jeton.isEmpty()) {
        return null
    }
    val toutVisible = jeton.all { caractère -> caractère in '!'..'~' }
    if (!toutVisible) {
        return null
    }
    return jeton
}
