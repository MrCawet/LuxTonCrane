import { vi } from 'vitest'

export const JETON = 'jeton-de-test'

/** Une demande reçue par la doublure : la méthode, l'adresse et le corps. */
export interface Demande {
  méthode: string
  adresse: string
  corps: unknown
}

/**
 * Doublure du serveur, pour les tests : garde les nuits, les fatigues et le coucher souhaité en mémoire, rend le relevé et ceux des dernières nuits donnés,
 * et refuse tout autre jeton que `JETON`.
 */
export function doublureDuServeur(relevé: unknown = {}, relevésDesDernièresNuits: unknown[] = []) {
  const nuits = new Map<string, unknown>()
  const journées = new Map<string, unknown>()
  const réglages = { coucher_souhaite: '23:00' }
  const demandes: Demande[] = []

  vi.stubGlobal('fetch', async (adresse: string, options: RequestInit = {}) => {
    const méthode = options.method ?? 'GET'
    const corps = options.body === undefined ? undefined : JSON.parse(String(options.body))
    demandes.push({ méthode, adresse, corps })
    if (new Headers(options.headers).get('Authorization') !== `Bearer ${JETON}`) {
      return new Response('', { status: 401 })
    }
    const jour = adresse.match(/^\/api\/nuits\/(.+)$/)?.[1]
    if (jour !== undefined && méthode === 'GET') {
      const nuit = nuits.get(jour)
      return nuit === undefined ? new Response('', { status: 404 }) : Response.json(nuit)
    }
    if (jour !== undefined && méthode === 'POST') {
      nuits.set(jour, { jour, ...(corps as object) })
      return new Response(null, { status: 204 })
    }
    const jourDeLaJournée = adresse.match(/^\/api\/journees\/(.+)$/)?.[1]
    if (jourDeLaJournée !== undefined && méthode === 'GET') {
      const journée = journées.get(jourDeLaJournée)
      return journée === undefined ? new Response('', { status: 404 }) : Response.json(journée)
    }
    if (jourDeLaJournée !== undefined && méthode === 'POST') {
      journées.set(jourDeLaJournée, { jour: jourDeLaJournée, ...(corps as object) })
      return new Response(null, { status: 204 })
    }
    if (adresse === '/api/reglages' && méthode === 'GET') {
      return Response.json(réglages)
    }
    if (adresse === '/api/reglages' && méthode === 'POST') {
      Object.assign(réglages, corps)
      return new Response(null, { status: 204 })
    }
    if (adresse.startsWith('/api/releve?')) {
      return Response.json(relevé)
    }
    if (adresse.startsWith('/api/releves?')) {
      return Response.json(relevésDesDernièresNuits)
    }
    return new Response('', { status: 404 })
  })

  return { nuits, journées, réglages, demandes }
}
