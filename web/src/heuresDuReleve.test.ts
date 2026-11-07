import { expect, test } from 'vitest'
import { heuresDuRelevé } from './heuresDuReleve'

// Les tests tournent dans le fuseau Europe/Paris (vite.config.ts).
const aujourdhui = new Date(2026, 10, 3)

function instantMs(jour: number, heure: number, minute: number): number {
  return new Date(2026, 10, jour, heure, minute).getTime()
}

test('le coucher est la veille du lever, et le coucher souhaité le soir du lever', () => {
  const heures = heuresDuRelevé('23:00', '07:00', '22:30', aujourdhui)

  expect(heures).toEqual({
    coucherMs: instantMs(2, 23, 0),
    leverMs: instantMs(3, 7, 0),
    coucherSouhaitéMs: instantMs(3, 22, 30),
  })
})

test('un coucher après minuit est le jour même du lever', () => {
  const heures = heuresDuRelevé('00:30', '07:00', '22:30', aujourdhui)

  expect(heures.coucherMs).toBe(instantMs(3, 0, 30))
})

test('un coucher souhaité après minuit est le lendemain du lever', () => {
  const heures = heuresDuRelevé('23:00', '07:00', '00:15', aujourdhui)

  expect(heures.coucherSouhaitéMs).toBe(instantMs(4, 0, 15))
})
