export interface HeuresDuRelevé {
  coucherMs: number
  leverMs: number
  coucherSouhaitéMs: number
}

export function heuresDuRelevé(
  _coucher: string,
  _lever: string,
  _coucherSouhaité: string,
  _aujourdhui: Date,
): HeuresDuRelevé {
  return { coucherMs: 0, leverMs: 0, coucherSouhaitéMs: 0 }
}
