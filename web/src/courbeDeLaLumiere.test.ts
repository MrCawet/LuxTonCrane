import { expect, test } from 'vitest'
import { hauteurDeLaLumière, tracésDeLaCourbe } from './courbeDeLaLumiere'

const MINUTE_MS = 60_000
const FENÊTRE = { debut_ms: 0, fin_ms: 100 * MINUTE_MS }

test('chaque période mesurée est un palier, placé dans la fenêtre', () => {
  const périodes = [
    { debut_ms: 10 * MINUTE_MS, fin_ms: 20 * MINUTE_MS, valeur_lux: 1000 },
    { debut_ms: 20 * MINUTE_MS, fin_ms: 30 * MINUTE_MS, valeur_lux: 10 },
  ]

  const tracés = tracésDeLaCourbe(FENÊTRE, périodes)

  expect(tracés).toEqual([
    [
      { début: 0.1, fin: 0.2, hauteur: hauteurDeLaLumière(1000) },
      { début: 0.2, fin: 0.3, hauteur: hauteurDeLaLumière(10) },
    ],
  ])
})

test('le temps non mesuré coupe la courbe en deux tracés', () => {
  const périodes = [
    { debut_ms: 10 * MINUTE_MS, fin_ms: 20 * MINUTE_MS, valeur_lux: 5 },
    { debut_ms: 50 * MINUTE_MS, fin_ms: 60 * MINUTE_MS, valeur_lux: 5 },
  ]

  const tracés = tracésDeLaCourbe(FENÊTRE, périodes)

  expect(tracés).toHaveLength(2)
})

test("la hauteur suit une échelle logarithmique, de 0,1 lx en bas à 100 000 lx en haut", () => {
  const de1À10Lx = hauteurDeLaLumière(10) - hauteurDeLaLumière(1)
  const de100À1000Lx = hauteurDeLaLumière(1000) - hauteurDeLaLumière(100)

  expect(de1À10Lx).toBeCloseTo(de100À1000Lx)
  expect(hauteurDeLaLumière(0.1)).toBe(0)
  expect(hauteurDeLaLumière(100_000)).toBe(1)
})

test('une lumière nulle est tout en bas, une lumière plus forte que le haut reste en haut', () => {
  expect(hauteurDeLaLumière(0)).toBe(0)
  expect(hauteurDeLaLumière(200_000)).toBe(1)
})
