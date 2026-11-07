import { afterEach, beforeEach, expect, test, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import ReleveDuJour from './ReleveDuJour.vue'
import { JETON, doublureDuServeur } from './doublureDuServeur'

const RELEVÉ = {
  lumiere_du_soir: {
    temps_au_dessus_du_seuil_ms: 600000,
    valeur_la_plus_haute_lux: 50,
    objectif_tenu: false,
    couverture: 0.0556,
  },
  lumiere_de_la_nuit: {
    temps_au_dessus_du_seuil_ms: 0,
    valeur_la_plus_haute_lux: null,
    objectif_tenu: true,
    couverture: 0,
  },
  lumiere_du_matin: { temps_a_10_000_lx_ou_plus_ms: 2400000, objectif_tenu: true, couverture: 0.5 },
  lumiere_de_la_journee: { part_du_temps_mesure_a_250_lx_ou_plus: 0.25, objectif_tenu: false, couverture: 0.8 },
}

const COUCHER_MS = new Date(2026, 10, 7, 23, 15).getTime()
const LEVER_MS = new Date(2026, 10, 8, 7, 0).getTime()

/** Un serveur où la nuit de ce matin est notée. */
function serveurAvecLaNuitDeCeMatin() {
  const serveur = doublureDuServeur(RELEVÉ)
  serveur.nuits.set('2026-11-08', {
    jour: '2026-11-08',
    endormissement: 'calme',
    continuite: 'reveil_bref',
    coucher_ms: COUCHER_MS,
    lever_ms: LEVER_MS,
  })
  return serveur
}

beforeEach(() => {
  vi.useFakeTimers({ toFake: ['Date'] })
  vi.setSystemTime(new Date(2026, 10, 8, 20, 0))
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.useRealTimers()
})

test("le relevé prend les heures de l'agenda du matin et le coucher souhaité réglé", async () => {
  const serveur = serveurAvecLaNuitDeCeMatin()
  serveur.réglages.coucher_souhaite = '22:30'

  const page = mount(ReleveDuJour, { props: { jeton: JETON } })
  await flushPromises()
  await page.find('button[name="voir"]').trigger('click')
  await flushPromises()

  const coucherSouhaité = new Date(2026, 10, 8, 22, 30).getTime()
  const demandeDuRelevé = serveur.demandes.find((demande) => demande.adresse.startsWith('/api/releve'))
  expect(demandeDuRelevé?.adresse).toBe(
    `/api/releve?coucher_ms=${COUCHER_MS}&lever_ms=${LEVER_MS}&coucher_souhaite_ms=${coucherSouhaité}`,
  )
})

test("sans agenda ce matin, le relevé demande de le remplir d'abord", async () => {
  const serveur = doublureDuServeur(RELEVÉ)

  const page = mount(ReleveDuJour, { props: { jeton: JETON } })
  await flushPromises()
  await page.find('button[name="voir"]').trigger('click')
  await flushPromises()

  expect(page.text()).toContain("Remplis d'abord l'agenda du matin.")
  expect(serveur.demandes.some((demande) => demande.adresse.startsWith('/api/releve'))).toBe(false)
})

test('le coucher souhaité se retrouve et se règle', async () => {
  const serveur = doublureDuServeur(RELEVÉ)
  serveur.réglages.coucher_souhaite = '22:30'

  const page = mount(ReleveDuJour, { props: { jeton: JETON } })
  await flushPromises()
  const champ = page.find('input[name="coucher_souhaite"]')
  expect((champ.element as HTMLInputElement).value).toBe('22:30')

  await champ.setValue('23:45')
  await page.find('button[name="regler"]').trigger('click')
  await flushPromises()

  expect(serveur.réglages.coucher_souhaite).toBe('23:45')
})

test('la page affiche les quatre fenêtres du relevé, avec les textes du téléphone', async () => {
  serveurAvecLaNuitDeCeMatin()

  const page = mount(ReleveDuJour, { props: { jeton: JETON } })
  await flushPromises()
  await page.find('button[name="voir"]').trigger('click')
  await flushPromises()

  const texte = page.text()
  expect(texte).toContain('Soir, 3 h avant le coucher')
  expect(texte).toContain('Au-dessus de 10 lx : 10 min')
  expect(texte).toContain('Valeur la plus haute : 50 lx')
  expect(texte).toContain('Couverture : 6 %')
  expect(texte).toContain('Nuit, du coucher au lever')
  expect(texte).toContain('Au-dessus de 1 lx : 0 min')
  expect(texte).toContain('Valeur la plus haute : rien de mesuré')
  expect(texte).toContain('Matin, 2 h après le lever')
  expect(texte).toContain('À 10 000 lx ou plus : 40 min')
  expect(texte).toContain('Journée, du lever à 3 h avant le coucher souhaité')
  expect(texte).toContain('Temps mesuré à 250 lx ou plus : 25 %')
  expect(texte).toContain('Objectif tenu : non')
  expect(texte).toContain('Objectif tenu : oui')
})
