/** Les instants d'une nuit, en millisecondes depuis le 1er janvier 1970 (UTC). */
export interface HeuresDeLaNuit {
  coucherMs: number
  leverMs: number
}

/**
 * Place dans le temps les heures de l'agenda (`HH:MM`), autour du lever du `jour`, dans le fuseau du navigateur :
 * le coucher est la veille s'il tombe après le lever.
 */
export function heuresDeLaNuit(coucher: string, lever: string, jour: Date): HeuresDeLaNuit {
  const décalageDuCoucher = coucher < lever ? 0 : -1
  return {
    coucherMs: instantMs(jour, décalageDuCoucher, coucher),
    leverMs: instantMs(jour, 0, lever),
  }
}

/** Le coucher souhaité (`HH:MM`) qui suit le lever : le soir même, ou le lendemain s'il tombe avant le lever. */
export function coucherSouhaitéMs(coucherSouhaité: string, leverMs: number): number {
  const lever = new Date(leverMs)
  const leSoirMême = instantMs(lever, 0, coucherSouhaité)
  if (leSoirMême > leverMs) {
    return leSoirMême
  }
  return instantMs(lever, 1, coucherSouhaité)
}

/** L'heure `HH:MM` d'un instant, dans le fuseau du navigateur. */
export function heureLocale(instantMs: number): string {
  const moment = new Date(instantMs)
  return `${deuxChiffres(moment.getHours())}:${deuxChiffres(moment.getMinutes())}`
}

/** Le jour `AAAA-MM-JJ` d'une date, dans le fuseau du navigateur. */
export function jourLocal(date: Date): string {
  return `${date.getFullYear()}-${deuxChiffres(date.getMonth() + 1)}-${deuxChiffres(date.getDate())}`
}

function deuxChiffres(nombre: number): string {
  return String(nombre).padStart(2, '0')
}

function instantMs(jour: Date, décalageEnJours: number, heure: string): number {
  const [heures, minutes] = heure.split(':').map(Number)
  const moment = new Date(jour.getFullYear(), jour.getMonth(), jour.getDate() + décalageEnJours, heures, minutes)
  return moment.getTime()
}
