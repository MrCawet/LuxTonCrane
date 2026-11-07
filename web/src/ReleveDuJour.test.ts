import { afterEach, expect, test, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import ReleveDuJour from './ReleveDuJour.vue'

const JETON = 'jeton-de-test'

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

/** Doublure du serveur : note les adresses demandées, et répond au jeton attendu avec le relevé. */
function serveurQuiRépond(): string[] {
  const adresses: string[] = []
  vi.stubGlobal('fetch', async (adresse: string, options: RequestInit) => {
    adresses.push(adresse)
    const entêtes = new Headers(options.headers)
    if (entêtes.get('Authorization') !== `Bearer ${JETON}`) {
      return new Response('', { status: 401 })
    }
    return new Response(JSON.stringify(RELEVÉ), { status: 200 })
  })
  return adresses
}

afterEach(() => {
  vi.unstubAllGlobals()
  vi.useRealTimers()
})

test('la page demande le relevé pour les heures par défaut, placées autour du lever d\'aujourd\'hui', async () => {
  vi.useFakeTimers({ toFake: ['Date'] })
  vi.setSystemTime(new Date(2026, 10, 3, 12, 0))
  const adresses = serveurQuiRépond()

  const page = mount(ReleveDuJour, { props: { jeton: JETON } })
  await page.find('button').trigger('click')
  await flushPromises()

  const coucher = new Date(2026, 10, 2, 23, 0).getTime()
  const lever = new Date(2026, 10, 3, 7, 0).getTime()
  const coucherSouhaité = new Date(2026, 10, 3, 23, 0).getTime()
  expect(adresses).toEqual([
    `/api/releve?coucher_ms=${coucher}&lever_ms=${lever}&coucher_souhaite_ms=${coucherSouhaité}`,
  ])
})

test('la page affiche les quatre fenêtres du relevé, avec les textes du téléphone', async () => {
  serveurQuiRépond()

  const page = mount(ReleveDuJour, { props: { jeton: JETON } })
  await page.find('button').trigger('click')
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
