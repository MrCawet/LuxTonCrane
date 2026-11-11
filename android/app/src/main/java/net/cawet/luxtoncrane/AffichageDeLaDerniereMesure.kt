package net.cawet.luxtoncrane

import android.appwidget.AppWidgetManager
import android.content.ComponentName
import android.content.Context
import android.content.SharedPreferences
import android.widget.RemoteViews

const val LAST_MEASURE_KEY = "derniere_mesure_lux"
private const val PREFERENCES = "derniere_mesure"

/** Les préférences où le service de mesure garde la dernière mesure, que l'accueil écoute. */
fun préférencesDeLaDernièreMesure(context: Context): SharedPreferences =
    context.getSharedPreferences(PREFERENCES, Context.MODE_PRIVATE)

/** La dernière mesure, en lx, ou `null` si le service n'en a pas encore noté. */
fun dernièreMesureLux(context: Context): Float? {
    val preferences = préférencesDeLaDernièreMesure(context)
    if (!preferences.contains(LAST_MEASURE_KEY)) {
        return null
    }
    return preferences.getFloat(LAST_MEASURE_KEY, 0f)
}

/** Garde la dernière mesure pour l'accueil, qui l'écoute, et l'affiche dans les widgets posés sur l'écran d'accueil. */
fun transmettreLaDernièreMesure(context: Context, valeurLux: Float) {
    val editor = préférencesDeLaDernièreMesure(context).edit()
    editor.putFloat(LAST_MEASURE_KEY, valeurLux)
    editor.apply()
    afficherDansLesWidgets(context, texteDeLaDernièreMesure(valeurLux))
}

fun afficherDansLesWidgets(context: Context, texte: String) {
    val view = RemoteViews(context.packageName, R.layout.widget_derniere_mesure)
    view.setTextViewText(R.id.derniere_mesure, texte)
    val widgets = ComponentName(context, WidgetDeLaDerniereMesure::class.java)
    val appWidgetManager = AppWidgetManager.getInstance(context)
    appWidgetManager.updateAppWidget(widgets, view)
}
