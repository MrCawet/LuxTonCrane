/** Les trois instants du relevé, en millisecondes depuis le 1er janvier 1970 (UTC). */
export interface HeuresDuRelevé {
  coucherMs: number
  leverMs: number
  coucherSouhaitéMs: number
}

/**
 * Place dans le temps les heures choisies (`HH:MM`), autour du lever d'aujourd'hui et dans le fuseau du navigateur,
 * comme le fait l'accueil Android : le coucher est la veille s'il tombe après le lever, le coucher souhaité est le
 * lendemain s'il tombe avant le lever.
 */
export function heuresDuRelevé(
  coucher: string,
  lever: string,
  coucherSouhaité: string,
  aujourdhui: Date,
): HeuresDuRelevé {
  const décalageDuCoucher = coucher < lever ? 0 : -1
  const décalageDuCoucherSouhaité = coucherSouhaité > lever ? 0 : 1
  return {
    coucherMs: instantMs(aujourdhui, décalageDuCoucher, coucher),
    leverMs: instantMs(aujourdhui, 0, lever),
    coucherSouhaitéMs: instantMs(aujourdhui, décalageDuCoucherSouhaité, coucherSouhaité),
  }
}

function instantMs(aujourdhui: Date, décalageEnJours: number, heure: string): number {
  const [heures, minutes] = heure.split(':').map(Number)
  const moment = new Date(
    aujourdhui.getFullYear(),
    aujourdhui.getMonth(),
    aujourdhui.getDate() + décalageEnJours,
    heures,
    minutes,
  )
  return moment.getTime()
}
