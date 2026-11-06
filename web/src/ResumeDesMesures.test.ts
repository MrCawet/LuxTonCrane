import { afterEach, beforeEach, expect, test, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import ResumeDesMesures from './ResumeDesMesures.vue'

const JETON = 'jeton-de-test'

/** Doublure du serveur : répond au jeton attendu avec le résumé donné, et refuse tout autre jeton. */
function serveurQuiRépond(résumé: unknown) {
  vi.stubGlobal('fetch', async (_adresse: string, options: RequestInit) => {
    const entêtes = new Headers(options.headers)
    if (entêtes.get('Authorization') !== `Bearer ${JETON}`) {
      return new Response('', { status: 401 })
    }
    return new Response(JSON.stringify(résumé), { status: 200 })
  })
}

beforeEach(() => {
  localStorage.clear()
})

afterEach(() => {
  vi.unstubAllGlobals()
})

test('sans jeton gardé, la page demande le jeton', async () => {
  serveurQuiRépond({ nombre: 0, derniere: null })

  const page = mount(ResumeDesMesures)
  await flushPromises()

  expect(page.find('input[type="password"]').exists()).toBe(true)
})

test('avec le jeton, la page affiche le nombre de mesures reçues et la dernière', async () => {
  serveurQuiRépond({ nombre: 2, derniere: { valeur_lux: 80, instant_ms: 1793000060000 } })

  const page = mount(ResumeDesMesures)
  await page.find('input[type="password"]').setValue(JETON)
  await page.find('form').trigger('submit')
  await flushPromises()

  expect(page.text()).toContain('2 mesures reçues')
  expect(page.text()).toContain('Dernière : 80 lx, le 26 octobre 2026 à 08:34')
})

test('le jeton gardé sert aux visites suivantes', async () => {
  serveurQuiRépond({ nombre: 2, derniere: { valeur_lux: 80, instant_ms: 1793000060000 } })
  const premièreVisite = mount(ResumeDesMesures)
  await premièreVisite.find('input[type="password"]').setValue(JETON)
  await premièreVisite.find('form').trigger('submit')
  await flushPromises()

  const visiteSuivante = mount(ResumeDesMesures)
  await flushPromises()

  expect(visiteSuivante.text()).toContain('2 mesures reçues')
})

test('un jeton refusé par le serveur est oublié, et la page le redemande', async () => {
  serveurQuiRépond({ nombre: 2, derniere: null })

  const page = mount(ResumeDesMesures)
  await page.find('input[type="password"]').setValue('faux')
  await page.find('form').trigger('submit')
  await flushPromises()

  expect(page.text()).toContain('Jeton refusé')
  expect(page.find('input[type="password"]').exists()).toBe(true)
})
