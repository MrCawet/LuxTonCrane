import { afterEach, beforeEach, expect, test, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import FatigueDuSoir from './FatigueDuSoir.vue'
import { JETON, doublureDuServeur } from './doublureDuServeur'

beforeEach(() => {
  vi.useFakeTimers({ toFake: ['Date'] })
  vi.setSystemTime(new Date(2026, 10, 8, 21, 0))
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.useRealTimers()
})

test("la page note la fatigue d'aujourd'hui", async () => {
  const serveur = doublureDuServeur()
  const page = mount(FatigueDuSoir, { props: { jeton: JETON } })
  await flushPromises()

  await page.find('input[value="genante"]').setValue(true)
  expect(page.find('button[type="submit"]').text()).toBe('Enregistrer')
  await page.find('form').trigger('submit')
  await flushPromises()

  expect(serveur.journées.get('2026-11-08')).toEqual({ jour: '2026-11-08', fatigue: 'genante' })
  expect(page.text()).toContain('Fatigue enregistrée.')
})

test("la fatigue enregistrée s'affiche en valeur, et « Modifier » rend les champs", async () => {
  doublureDuServeur()
  const page = mount(FatigueDuSoir, { props: { jeton: JETON } })
  await flushPromises()

  await page.find('input[value="genante"]').setValue(true)
  await page.find('form').trigger('submit')
  await flushPromises()

  expect(page.find('form').exists()).toBe(false)
  expect(page.text()).toContain('Fatigue de la journée : Gênante')

  await page.find('button[name="modifier"]').trigger('click')

  expect((page.find('input[value="genante"]').element as HTMLInputElement).checked).toBe(true)
})

test('une fatigue déjà notée se retrouve préremplie', async () => {
  const serveur = doublureDuServeur()
  serveur.journées.set('2026-11-08', { jour: '2026-11-08', fatigue: 'somnolence' })

  const page = mount(FatigueDuSoir, { props: { jeton: JETON } })
  await flushPromises()

  expect(page.text()).toContain('Fatigue de la journée : Somnolence')
  await page.find('button[name="modifier"]').trigger('click')

  expect((page.find('input[value="somnolence"]').element as HTMLInputElement).checked).toBe(true)
})

test("les niveaux se lisent par ce qui s'est passé dans la journée", async () => {
  doublureDuServeur()
  const page = mount(FatigueDuSoir, { props: { jeton: JETON } })
  await flushPromises()

  expect(page.text()).toContain("je n'ai pas pensé à ma fatigue")
  expect(page.text()).toContain("j'ai lutté pour ne pas m'endormir, ou je me suis endormi sans le vouloir")
})
