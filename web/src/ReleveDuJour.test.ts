import { afterEach, beforeEach, expect, test, vi } from 'vitest'
import { flushPromises, mount, type VueWrapper } from '@vue/test-utils'
import ReleveDuJour from './ReleveDuJour.vue'
import { JETON, doublureDuServeur } from './doublureDuServeur'

/** Un instant du 7 ou du 8 novembre 2026, dans le fuseau des tests. */
function instantMs(jour: number, heure: number, minute: number): number {
  return new Date(2026, 10, jour, heure, minute).getTime()
}

const RELEVÉ = {
  endormissement: { niveau: 'calme', objectif_tenu: true },
  continuite: { niveau: 'reveil_long', objectif_tenu: false },
  quantite_de_sommeil: { duree_ms: 27900000, objectif_tenu: true },
  lumiere_du_soir: {
    temps_au_dessus_du_seuil_ms: 600000,
    valeur_la_plus_haute_lux: 50,
    objectif_tenu: false,
    couverture: 0.0556,
    fenetre: { debut_ms: instantMs(7, 20, 15), fin_ms: instantMs(7, 23, 15) },
    periodes_mesurees: [{ debut_ms: instantMs(7, 22, 0), fin_ms: instantMs(7, 22, 10), valeur_lux: 50 }],
  },
  lumiere_de_la_nuit: {
    temps_au_dessus_du_seuil_ms: 0,
    valeur_la_plus_haute_lux: null,
    objectif_tenu: true,
    couverture: 0,
    fenetre: { debut_ms: instantMs(7, 23, 15), fin_ms: instantMs(8, 7, 0) },
    periodes_mesurees: [],
  },
  lumiere_du_matin: {
    temps_a_10_000_lx_ou_plus_ms: 2400000,
    objectif_tenu: true,
    couverture: 0.5,
    fenetre: { debut_ms: instantMs(8, 7, 0), fin_ms: instantMs(8, 9, 0) },
    periodes_mesurees: [],
  },
  lumiere_de_la_journee: {
    part_du_temps_mesure_a_250_lx_ou_plus: 0.25,
    objectif_tenu: false,
    couverture: 0.8,
    fenetre: { debut_ms: instantMs(8, 7, 0), fin_ms: instantMs(8, 19, 30) },
    periodes_mesurees: [],
  },
  fatigue: { niveau: 'genante', objectif_tenu: false },
}

// La veille, le soir est tenu et la fatigue n'a pas été notée ; aujourd'hui, le soir n'est pas tenu.
const RELEVÉS_DES_DERNIÈRES_NUITS = [
  {
    ...RELEVÉ,
    jour: '2026-11-07',
    lumiere_du_soir: { ...RELEVÉ.lumiere_du_soir, temps_au_dessus_du_seuil_ms: 0, objectif_tenu: true },
    fatigue: null,
  },
  { ...RELEVÉ, jour: '2026-11-08' },
]

const COUCHER_MS = instantMs(7, 23, 15)
const LEVER_MS = instantMs(8, 7, 0)

/** Un serveur où la nuit de ce matin est notée, et qui rend `relevé` et ceux des deux dernières nuits. */
function serveurAvecLaNuitDeCeMatin(relevé: unknown = RELEVÉ) {
  const serveur = doublureDuServeur(relevé, RELEVÉS_DES_DERNIÈRES_NUITS)
  serveur.nuits.set('2026-11-08', {
    jour: '2026-11-08',
    endormissement: 'calme',
    continuite: 'reveil_bref',
    coucher_ms: COUCHER_MS,
    lever_ms: LEVER_MS,
  })
  return serveur
}

/** Chaque ligne du relevé, écrite « ce qu'on mesure : valeur, objectif ». */
function lignesDuRelevé(page: VueWrapper): string[] {
  return page.findAll('dl > div').map((ligne) => {
    const [mesuré, ...valeurs] = ligne.findAll('dt, dd').map((partie) => partie.text())
    return `${mesuré} : ${valeurs.join(', ')}`
  })
}

/** Les jours de la tendance nommée, tels qu'ils se lisent : « jour : valeur, objectif ». */
function joursDeLaTendance(page: VueWrapper, nom: string): string[] {
  const tendance = page.find(`[aria-label="Tendance des dernières nuits : ${nom}"]`)
  return tendance.findAll('li').map((jour) => jour.attributes('aria-label') ?? '')
}

beforeEach(() => {
  vi.useFakeTimers({ toFake: ['Date'] })
  vi.setSystemTime(new Date(2026, 10, 8, 20, 0))
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.useRealTimers()
})

test("le relevé est celui de la nuit de ce matin, avec le coucher souhaité réglé", async () => {
  const serveur = serveurAvecLaNuitDeCeMatin()
  serveur.réglages.coucher_souhaite = '22:30'

  const page = mount(ReleveDuJour, { props: { jeton: JETON } })
  await flushPromises()
  await page.find('button[name="voir"]').trigger('click')
  await flushPromises()

  const coucherSouhaité = new Date(2026, 10, 8, 22, 30).getTime()
  const demandeDuRelevé = serveur.demandes.find((demande) => demande.adresse.startsWith('/api/releve'))
  expect(demandeDuRelevé?.adresse).toBe(
    `/api/releve?jour=2026-11-08&coucher_souhaite_ms=${coucherSouhaité}`,
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

test("un coucher souhaité vide n'est pas envoyé, et la page le dit", async () => {
  const serveur = doublureDuServeur(RELEVÉ)
  serveur.réglages.coucher_souhaite = '22:30'

  const page = mount(ReleveDuJour, { props: { jeton: JETON } })
  await flushPromises()
  await page.find('input[name="coucher_souhaite"]').setValue('')
  await page.find('button[name="regler"]').trigger('click')
  await flushPromises()

  expect(serveur.réglages.coucher_souhaite).toBe('22:30')
  expect(page.text()).toContain("Indique l'heure du coucher souhaité.")
})

test("la page affiche les quatre fenêtres du relevé, l'objectif sur la ligne qu'il juge", async () => {
  serveurAvecLaNuitDeCeMatin()

  const page = mount(ReleveDuJour, { props: { jeton: JETON } })
  await flushPromises()
  await page.find('button[name="voir"]').trigger('click')
  await flushPromises()

  const texte = page.text()
  expect(texte).toContain('Soir, 3 h avant le coucher')
  expect(texte).toContain('Nuit, du coucher au lever')
  expect(texte).toContain('Matin, 2 h après le lever')
  expect(texte).toContain('Journée, du lever à 3 h avant le coucher souhaité')
  const lignes = lignesDuRelevé(page)
  expect(lignes).toContain('Au-dessus de 10 lx : 10 min, objectif non tenu')
  expect(lignes).toContain('Valeur la plus haute : 50 lx')
  expect(lignes).toContain('Couverture : 6 %')
  expect(lignes).toContain('Au-dessus de 1 lx : 0 min, objectif tenu')
  expect(lignes).toContain('Valeur la plus haute : rien de mesuré')
  expect(lignes).toContain('À 10 000 lx ou plus : 40 min, objectif tenu')
  expect(lignes).toContain('Temps mesuré à 250 lx ou plus : 25 %, objectif non tenu')
})

test("le relevé montre les notes de l'agenda et la quantité de sommeil, l'objectif sur leur ligne", async () => {
  serveurAvecLaNuitDeCeMatin()

  const page = mount(ReleveDuJour, { props: { jeton: JETON } })
  await flushPromises()
  await page.find('button[name="voir"]').trigger('click')
  await flushPromises()

  const texte = page.text()
  expect(texte).toContain('Sommeil')
  expect(texte).toContain('Fatigue, le soir suivant')
  const lignes = lignesDuRelevé(page)
  expect(lignes).toContain('Endormissement : Calme, objectif tenu')
  expect(lignes).toContain('Continuité de la nuit : Réveil long, objectif non tenu')
  expect(lignes).toContain('Quantité de sommeil : 7 h 45 min, objectif de plus de 7 h tenu')
  expect(lignes).toContain('Fatigue de la journée : Gênante, objectif non tenu')
})

test("tant que la fatigue n'est pas notée, le relevé le dit", async () => {
  serveurAvecLaNuitDeCeMatin({ ...RELEVÉ, fatigue: null })

  const page = mount(ReleveDuJour, { props: { jeton: JETON } })
  await flushPromises()
  await page.find('button[name="voir"]').trigger('click')
  await flushPromises()

  expect(lignesDuRelevé(page)).toContain('Fatigue de la journée : pas encore notée')
})

test('chaque fenêtre de lumière montre sa courbe, avec son seuil et ses heures', async () => {
  serveurAvecLaNuitDeCeMatin()

  const page = mount(ReleveDuJour, { props: { jeton: JETON } })
  await flushPromises()
  await page.find('button[name="voir"]').trigger('click')
  await flushPromises()

  const courbes = page.findAll('[role="img"]').map((courbe) => courbe.attributes('aria-label'))
  expect(courbes).toEqual([
    'Courbe de la lumière, seuil 10 lx',
    'Courbe de la lumière, seuil 1 lx',
    'Courbe de la lumière, seuil 10 000 lx',
    'Courbe de la lumière, seuil 250 lx',
  ])
  const texte = page.text()
  expect(texte).toContain('20:15')
  expect(texte).toContain('19:30')
})

test('chaque fenêtre de lumière montre sa tendance des dernières nuits, avec son objectif', async () => {
  serveurAvecLaNuitDeCeMatin()

  const page = mount(ReleveDuJour, { props: { jeton: JETON } })
  await flushPromises()
  await page.find('button[name="voir"]').trigger('click')
  await flushPromises()

  const joursDuSoir = joursDeLaTendance(page, 'au-dessus de 10 lx')
  expect(joursDuSoir).toEqual(['sam. 7 : 0 min, objectif tenu', 'dim. 8 : 10 min, objectif non tenu'])
  expect(joursDeLaTendance(page, 'au-dessus de 1 lx')).toHaveLength(2)
  expect(joursDeLaTendance(page, 'à 10 000 lx ou plus')).toHaveLength(2)
  const joursDeLaJournée = joursDeLaTendance(page, 'temps mesuré à 250 lx ou plus')
  expect(joursDeLaJournée[1]).toBe('dim. 8 : 25 %, objectif non tenu')
})

test('le sommeil et la fatigue montrent leur tendance des dernières nuits, avec leur objectif', async () => {
  serveurAvecLaNuitDeCeMatin()

  const page = mount(ReleveDuJour, { props: { jeton: JETON } })
  await flushPromises()
  await page.find('button[name="voir"]').trigger('click')
  await flushPromises()

  expect(joursDeLaTendance(page, 'endormissement')).toEqual([
    'sam. 7 : Calme, objectif tenu',
    'dim. 8 : Calme, objectif tenu',
  ])
  expect(joursDeLaTendance(page, 'continuité de la nuit')[1]).toBe('dim. 8 : Réveil long, objectif non tenu')
  expect(joursDeLaTendance(page, 'quantité de sommeil')[1]).toBe('dim. 8 : 7 h 45 min, objectif tenu')
  expect(joursDeLaTendance(page, 'fatigue de la journée')).toEqual([
    'sam. 7 : pas encore notée',
    'dim. 8 : Gênante, objectif non tenu',
  ])
})
