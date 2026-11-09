import { afterEach, beforeEach, expect, test, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import AgendaDuMatin from './AgendaDuMatin.vue'
import { JETON, doublureDuServeur } from './doublureDuServeur'

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
  await page.find('input[name="coucher"]').setValue('23:15')
  await page.find('input[name="lever"]').setValue('07:00')
  await page.find('form').trigger('submit')
  await flushPromises()

  expect(serveur.nuits.get('2026-11-08')).toEqual({
    jour: '2026-11-08',
    endormissement: 'calme',
    continuite: 'reveil_bref',
    coucher_ms: COUCHER_MS,
    lever_ms: LEVER_MS,
  })
  expect(page.text()).toContain('Agenda gardé.')
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

  expect((page.find('input[value="agite"]').element as HTMLInputElement).checked).toBe(true)
  expect((page.find('input[value="sans_reveil"]').element as HTMLInputElement).checked).toBe(true)
  expect((page.find('input[name="coucher"]').element as HTMLInputElement).value).toBe('23:15')
  expect((page.find('input[name="lever"]').element as HTMLInputElement).value).toBe('07:00')
})

test('les niveaux se lisent par ce qui s\'est passé', async () => {
  doublureDuServeur()
  const page = mount(AgendaDuMatin, { props: { jeton: JETON } })
  await flushPromises()

  expect(page.text()).toContain("je ne me souviens pas d'avoir attendu")
  expect(page.text()).toContain('je me suis levé, j\'ai pris le téléphone, ou je suis resté éveillé longtemps')
})

// Un pas plus grand que la minute fait refuser au navigateur, sans message visible sur Firefox Android, une heure
// comme 07:52 : le formulaire ne part pas (ANO-0002).
test("les heures du coucher et du lever se saisissent à la minute près", async () => {
  doublureDuServeur()
  const page = mount(AgendaDuMatin, { props: { jeton: JETON } })
  await flushPromises()

  for (const nom of ['coucher', 'lever']) {
    const pas = Number(page.find(`input[name="${nom}"]`).attributes('step') ?? '60')
    expect(pas).toBeLessThanOrEqual(60)
  }
})

test("la page dit pourquoi elle refuse l'agenda, et n'envoie rien", async () => {
  const serveur = doublureDuServeur()
  const page = mount(AgendaDuMatin, { props: { jeton: JETON } })
  await flushPromises()

  await page.find('input[value="calme"]').setValue(true)
  await page.find('input[name="lever"]').setValue('')
  await page.find('form').trigger('submit')
  await flushPromises()

  expect(serveur.nuits.size).toBe(0)
  expect(page.text()).toContain('Choisis un niveau pour la continuité de la nuit.')
  expect(page.text()).toContain("Indique l'heure du lever.")
})

test('la page montre la raison quand le navigateur refuse un champ', async () => {
  doublureDuServeur()
  const page = mount(AgendaDuMatin, { props: { jeton: JETON } })
  await flushPromises()

  await page.find('input[name="lever"]').trigger('invalid')

  expect(page.text()).toContain('Lever :')
})
