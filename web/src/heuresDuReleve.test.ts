import { expect, test } from 'vitest'
import { coucherSouhaitéMs, heureLocale, heuresDeLaNuit, jourLocal } from './heuresDuReleve'

// Les tests tournent dans le fuseau Europe/Paris (vite.config.ts).
const aujourdhui = new Date(2026, 10, 3)

function instantMs(jour: number, heure: number, minute: number): number {
  return new Date(2026, 10, jour, heure, minute).getTime()
}

test('le coucher est la veille du lever', () => {
  const nuit = heuresDeLaNuit('23:00', '07:00', aujourdhui)

  expect(nuit).toEqual({ coucherMs: instantMs(2, 23, 0), leverMs: instantMs(3, 7, 0) })
})

test('un coucher après minuit est le jour même du lever', () => {
  const nuit = heuresDeLaNuit('00:30', '07:00', aujourdhui)

  expect(nuit.coucherMs).toBe(instantMs(3, 0, 30))
})

test('le coucher souhaité est le soir du lever', () => {
  expect(coucherSouhaitéMs('22:30', instantMs(3, 7, 0))).toBe(instantMs(3, 22, 30))
})

test('un coucher souhaité après minuit est le lendemain du lever', () => {
  expect(coucherSouhaitéMs('00:15', instantMs(3, 7, 0))).toBe(instantMs(4, 0, 15))
})

test("l'heure et le jour d'un instant se lisent dans le fuseau du navigateur", () => {
  expect(heureLocale(instantMs(2, 23, 15))).toBe('23:15')
  expect(heureLocale(instantMs(3, 7, 5))).toBe('07:05')
  expect(jourLocal(new Date(2026, 10, 3, 9, 0))).toBe('2026-11-03')
})
