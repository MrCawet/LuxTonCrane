/** Les instants d'une nuit, en millisecondes depuis le 1er janvier 1970 (UTC). */
export interface HeuresDeLaNuit {
  coucherMs: number
  leverMs: number
}

export function heuresDeLaNuit(_coucher: string, _lever: string, _jour: Date): HeuresDeLaNuit {
  return { coucherMs: 0, leverMs: 0 }
}

export function coucherSouhaitéMs(_coucherSouhaité: string, _leverMs: number): number {
  return 0
}

export function heureLocale(_instantMs: number): string {
  return ''
}

export function jourLocal(_date: Date): string {
  return ''
}
