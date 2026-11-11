package net.cawet.luxtoncrane

import android.appwidget.AppWidgetManager
import android.appwidget.AppWidgetProvider
import android.content.Context

/**
 * Le widget de la taille d'une icône qui affiche la dernière mesure. Le service de mesure le met à jour ;
 * Android ne l'appelle que quand il est posé, pour qu'il montre tout de suite la dernière mesure connue.
 * Sans accent : le nom est déclaré dans le manifeste.
 */
class WidgetDeLaDerniereMesure : AppWidgetProvider() {

    override fun onUpdate(context: Context, appWidgetManager: AppWidgetManager, appWidgetIds: IntArray) {
        val valeurLux = dernièreMesureLux(context) ?: return
        afficherDansLesWidgets(context, texteDeLaDernièreMesure(valeurLux))
    }
}
