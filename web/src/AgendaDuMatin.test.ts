import { afterEach, beforeEach, expect, test, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import AgendaDuMatin from './AgendaDuMatin.vue'
import { JETON, doublureDuServeur } from './doublureDuServeur'
import { choisirSurLaRoue, heureDeLaRoue, minutesDeLaRoue } from './roueDeLHeureDansLesTests'

const COUCHER_MS = new Date(2026, 10, 7, 23, 15).getTime()
const LEVER_MS = new Date(2026, 10, 8, 7, 0).getTime()

beforeEach(() => {
  vi.useFakeTimers({ toFake: ['Date'] })
  vi.setSystemTime(new Date(2026, 10, 8, 9, 0))
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.useRealTimers()
})

test("la page note l'agenda de la nuit qui finit ce matin", async () => {
  const serveur = doublureDuServeur()
  const page = mount(AgendaDuMatin, { props: { jeton: JETON } })
  await flushPromises()

  await page.find('input[value="calme"]').setValue(true)
  await page.find('input[value="reveil_bref"]').setValue(true)
  await choisirSurLaRoue(page, 'Coucher', '23:15')
  await choisirSurLaRoue(page, 'Lever', '07:00')
  expect(page.find('button[type="submit"]').text()).toBe('Enregistrer')
  await page.find('form').trigger('submit')
  await flushPromises()

  expect(serveur.nuits.get('2026-11-08')).toEqual({
    jour: '2026-11-08',
    endormissement: 'calme',
    continuite: 'reveil_bref',
    coucher_ms: COUCHER_MS,
    lever_ms: LEVER_MS,
  })
  expect(page.text()).toContain('Agenda enregistré.')
})

test("l'agenda enregistré s'affiche en valeurs, et « Modifier » rend les champs", async () => {
  doublureDuServeur()
  const page = mount(AgendaDuMatin, { props: { jeton: JETON } })
  await flushPromises()

  await page.find('input[value="calme"]').setValue(true)
  await page.find('input[value="reveil_bref"]').setValue(true)
  await choisirSurLaRoue(page, 'Coucher', '23:15')
  await choisirSurLaRoue(page, 'Lever', '07:00')
  await page.find('form').trigger('submit')
  await flushPromises()

  expect(page.find('form').exists()).toBe(false)
  expect(page.text()).toContain('Endormissement : Calme')
  expect(page.text()).toContain('Continuité de la nuit : Réveil bref')
  expect(page.text()).toContain('Coucher : 23:15')
  expect(page.text()).toContain('Lever : 07:00')

  await page.find('button[name="modifier"]').trigger('click')

  expect(heureDeLaRoue(page, 'Coucher')).toBe('23:15')
})

test('une nuit déjà notée se retrouve préremplie', async () => {
  const serveur = doublureDuServeur()
  serveur.nuits.set('2026-11-08', {
    jour: '2026-11-08',
    endormissement: 'agite',
    continuite: 'sans_reveil',
    coucher_ms: COUCHER_MS,
    lever_ms: LEVER_MS,
  })

  const page = mount(AgendaDuMatin, { props: { jeton: JETON } })
  await flushPromises()

  expect(page.text()).toContain('Endormissement : Agité')
  await page.find('button[name="modifier"]').trigger('click')

  expect((page.find('input[value="agite"]').element as HTMLInputElement).checked).toBe(true)
  expect((page.find('input[value="sans_reveil"]').element as HTMLInputElement).checked).toBe(true)
  expect(heureDeLaRoue(page, 'Coucher')).toBe('23:15')
  expect(heureDeLaRoue(page, 'Lever')).toBe('07:00')
})

test('les niveaux se lisent par ce qui s\'est passé', async () => {
  doublureDuServeur()
  const page = mount(AgendaDuMatin, { props: { jeton: JETON } })
  await flushPromises()

  expect(page.text()).toContain("je ne me souviens pas d'avoir attendu")
  expect(page.text()).toContain('je me suis levé, j\'ai pris le téléphone, ou je suis resté éveillé longtemps')
})

// Une heure comme 07:52 doit pouvoir se noter telle quelle (ANO-0002).
test("les heures du coucher et du lever se choisissent à la minute près", async () => {
  doublureDuServeur()
  const page = mount(AgendaDuMatin, { props: { jeton: JETON } })
  await flushPromises()

  for (const nom of ['Coucher', 'Lever']) {
    expect(minutesDeLaRoue(page, nom)).toHaveLength(60)
  }
})

test("la page dit pourquoi elle refuse l'agenda, et n'envoie rien", async () => {
  const serveur = doublureDuServeur()
  const page = mount(AgendaDuMatin, { props: { jeton: JETON } })
  await flushPromises()

  await page.find('input[value="calme"]').setValue(true)
  await page.find('form').trigger('submit')
  await flushPromises()

  expect(serveur.nuits.size).toBe(0)
  expect(page.text()).toContain('Choisis un niveau pour la continuité de la nuit.')
})
