package net.cawet.luxtoncrane

import android.content.Context
import java.io.File

/** Le service de mesure écrit le journal et l'accueil le lit : les deux l'ouvrent au même endroit. */
fun cheminDuJournal(context: Context): String = File(context.filesDir, "journal.sqlite").path
