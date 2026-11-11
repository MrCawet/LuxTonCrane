/** Une fenêtre du relevé, telle que le serveur la rend. */
export interface FenêtreDuRelevé {
  debut_ms: number
  fin_ms: number
}

/** Le temps pendant lequel une mesure valable vaut, tel que le serveur le rend. */
export interface PériodeMesurée {
  debut_ms: number
  fin_ms: number
  valeur_lux: number
}

/**
 * Une période mesurée placée sur le graphique : `début` et `fin` vont de 0 (début de la fenêtre) à 1 (fin),
 * `hauteur` de 0 (en bas) à 1 (en haut).
 */
export interface Palier {
  début: number
  fin: number
  hauteur: number
}

// De 0,1 lx, une nuit noire, à 100 000 lx, le plein soleil : six puissances de dix.
const LUMIÈRE_DU_BAS_LX = 0.1
const LUMIÈRE_DU_HAUT_LX = 100_000

/** La hauteur d'une lumière sur une échelle logarithmique, de 0 (en bas) à 1 (en haut). */
export function hauteurDeLaLumière(valeurLux: number): number {
  if (valeurLux <= LUMIÈRE_DU_BAS_LX) {
    return 0
  }
  if (valeurLux >= LUMIÈRE_DU_HAUT_LX) {
    return 1
  }
  const étendue = Math.log10(LUMIÈRE_DU_HAUT_LX) - Math.log10(LUMIÈRE_DU_BAS_LX)
  return (Math.log10(valeurLux) - Math.log10(LUMIÈRE_DU_BAS_LX)) / étendue
}

/** Les tracés de la courbe : une suite de paliers sans trou, coupée là où le temps n'est pas mesuré. */
export function tracésDeLaCourbe(fenêtre: FenêtreDuRelevé, périodes: PériodeMesurée[]): Palier[][] {
  const duréeDeLaFenêtreMs = fenêtre.fin_ms - fenêtre.debut_ms
  const tracés: Palier[][] = []
  let finPrécédenteMs: number | null = null
  for (const période of périodes) {
    if (période.debut_ms !== finPrécédenteMs) {
      tracés.push([])
    }
    const tracéEnCours = tracés[tracés.length - 1]
    tracéEnCours.push({
      début: (période.debut_ms - fenêtre.debut_ms) / duréeDeLaFenêtreMs,
      fin: (période.fin_ms - fenêtre.debut_ms) / duréeDeLaFenêtreMs,
      hauteur: hauteurDeLaLumière(période.valeur_lux),
    })
    finPrécédenteMs = période.fin_ms
  }
  return tracés
}
