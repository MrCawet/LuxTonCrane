import type { VueWrapper } from '@vue/test-utils'

/** Choisit `heure` (« HH:MM ») sur la roue nommée `nom`, en touchant son heure puis sa minute. */
export async function choisirSurLaRoue(page: VueWrapper, nom: string, heure: string) {
  const [heures, minutes] = heure.split(':')
  await page.find(`[aria-label="${nom}, heures"] button[value="${heures}"]`).trigger('click')
  await page.find(`[aria-label="${nom}, minutes"] button[value="${minutes}"]`).trigger('click')
}

/** L'heure (« HH:MM ») que montre la roue nommée `nom`. */
export function heureDeLaRoue(page: VueWrapper, nom: string): string {
  const heures = page.find(`[aria-label="${nom}, heures"] button[aria-pressed="true"]`).attributes('value')
  const minutes = page.find(`[aria-label="${nom}, minutes"] button[aria-pressed="true"]`).attributes('value')
  return `${heures}:${minutes}`
}

/** Les minutes que propose la roue nommée `nom`, sur un tour. */
export function minutesDeLaRoue(page: VueWrapper, nom: string): string[] {
  const boutons = page.findAll(`[aria-label="${nom}, minutes"] button`)
  const valeurs = boutons.map((bouton) => bouton.attributes('value') ?? '')
  return [...new Set(valeurs)]
}

/**
 * Ce que la colonne `colonne` (« heures » ou « minutes ») de la roue nommée `nom` montre autour de son choix :
 * la valeur d'avant, le choix, la valeur d'après.
 */
export function autourDuChoix(page: VueWrapper, nom: string, colonne: string): string[] {
  const boutons = page.findAll(`[aria-label="${nom}, ${colonne}"] button`)
  const rangDuChoix = boutons.findIndex((bouton) => bouton.attributes('aria-pressed') === 'true')
  const voisinage = boutons.slice(rangDuChoix - 1, rangDuChoix + 2)
  return voisinage.map((bouton) => bouton.attributes('value') ?? '')
}
