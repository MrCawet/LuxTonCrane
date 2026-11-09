/** Ce que l'opérateur a saisi dans l'agenda du matin, tel que le formulaire le tient. */
export interface SaisieDeLAgendaDuMatin {
  endormissement: string
  continuite: string
  coucher: string
  lever: string
}

/** Les raisons, dites à l'opérateur, pour lesquelles l'agenda du matin ne peut pas être gardé ; vide s'il peut l'être. */
export function erreursDeLAgendaDuMatin(saisie: SaisieDeLAgendaDuMatin): string[] {
  const erreurs: string[] = []
  if (saisie.endormissement === '') {
    erreurs.push("Choisis un niveau pour l'endormissement.")
  }
  if (saisie.continuite === '') {
    erreurs.push('Choisis un niveau pour la continuité de la nuit.')
  }
  if (saisie.coucher === '') {
    erreurs.push("Indique l'heure du coucher.")
  }
  if (saisie.lever === '') {
    erreurs.push("Indique l'heure du lever.")
  }
  if (saisie.coucher !== '' && saisie.coucher === saisie.lever) {
    erreurs.push('Le coucher et le lever ne peuvent pas être à la même heure.')
  }
  return erreurs
}

/** La raison pour laquelle le navigateur refuse un champ, précédée de son nom. */
export function raisonDuNavigateur(nom: string, champ: HTMLInputElement): string {
  const raison = champ.validationMessage === '' ? 'valeur refusée par le navigateur.' : champ.validationMessage
  return `${nom} : ${raison}`
}
