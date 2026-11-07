package net.cawet.luxtoncrane

import java.io.IOException
import java.net.HttpURLConnection
import java.net.URL
import org.json.JSONArray
import org.json.JSONObject

// L'adresse n'est pas secrète : seul le jeton protège les mesures. Le jeton n'est donc pas dans l'APK, qui est public.
private const val MEASURES_URL = "https://lux-ton-crane.cawet.net/api/mesures"

/** Le serveur de mcProd, joint en HTTPS avec le jeton. Lève une `IOException` s'il ne répond pas comme prévu. */
class ServeurHttp(private val jeton: String) : ServeurDesMesures {

    private fun open(method: String): HttpURLConnection {
        val connection = URL(MEASURES_URL).openConnection() as HttpURLConnection
        connection.requestMethod = method
        connection.setRequestProperty("Authorization", "Bearer $jeton")
        connection.connectTimeout = TIMEOUT_MS
        connection.readTimeout = TIMEOUT_MS
        return connection
    }

    private fun requireStatus(connection: HttpURLConnection, expected: Int) {
        val status = connection.responseCode
        if (status != expected) {
            throw IOException("le serveur a répondu $status au lieu de $expected")
        }
    }

    override fun dernierInstantReçuMs(): Long? {
        val connection = open("GET")
        try {
            requireStatus(connection, 200)
            val body = connection.inputStream.bufferedReader().use { reader -> reader.readText() }
            val résumé = JSONObject(body)
            if (résumé.isNull("derniere")) {
                return null
            }
            val dernière = résumé.getJSONObject("derniere")
            return dernière.getLong("instant_ms")
        } finally {
            connection.disconnect()
        }
    }

    override fun envoyer(lot: List<MesureÀEnvoyer>) {
        val body = JSONArray()
        for (mesure in lot) {
            val json = JSONObject()
            json.put("valeur_lux", mesure.valeurLux.toDouble())
            json.put("instant_ms", mesure.instantMs)
            if (mesure.téléphoneCouvert != null) {
                json.put("telephone_couvert", mesure.téléphoneCouvert)
            }
            body.put(json)
        }
        val connection = open("POST")
        try {
            connection.doOutput = true
            connection.setRequestProperty("Content-Type", "application/json")
            connection.outputStream.use { output -> output.write(body.toString().toByteArray()) }
            requireStatus(connection, 204)
        } finally {
            connection.disconnect()
        }
    }

    private companion object {
        const val TIMEOUT_MS = 30_000
    }
}
