---
id: US-0002
type: us
titre: Le relevé du jour se lit sur la page web
statut: en_cours
priorite: essentiel
cree_le: 2026-11-07
maj_le: 2026-11-07
source: REQ-0001
emetteur: Opérateur
composants: [releve, serveur, web]
tickets_lies: [REQ-0001, US-0001]
objectif: aucun
niveau: 1
epique: EP-0001
valeur: Pour l'opérateur, lire son relevé du jour sur le web plutôt que sur le téléphone.
---

## Provenance

Deuxième tranche de [EP-0001](../epiques/EP-0001-le-rapport-sur-le-web.md), tirée de
[REQ-0001](../requetes/REQ-0001-rapport-dans-une-webapp.md). Elle attendait
[US-0001](US-0001-mesures-envoyees-au-serveur.md), terminée le 2026-11-07. Le 2026-11-07, l'opérateur choisit que
la page demande les trois heures dans des champs, comme les roues de l'accueil Android, sans les garder sur le
serveur. Priorité proposée par l'agent, `essentiel` : c'est le but de l'épique.

## Besoin

**En tant que** opérateur, **je veux** voir sur la page web le relevé du jour calculé à partir des mesures reçues par
le serveur, **afin de** ne plus avoir à le lire sur le téléphone.

## Comportement attendu

- La page a trois champs d'heure : le coucher, le lever et le coucher souhaité (23:00, 07:00 et 23:00 par défaut,
  comme sur le téléphone), et un bouton « Voir le relevé du jour ».
- Le serveur calcule le relevé avec `releve/`, sur les mesures qu'il a reçues.
- La page affiche les quatre fenêtres du relevé, avec les mêmes textes que l'accueil Android : soir, nuit, matin,
  journée.

Hypothèses :
- Les heures se placent dans le temps comme sur le téléphone : le coucher est la veille s'il tombe après le lever, le
  coucher souhaité est le lendemain s'il tombe avant le lever. Le fuseau est celui du navigateur.
- Le relevé exige le jeton, comme le reste de l'API.

## Hors périmètre

- Garder les heures sur le serveur, l'agenda du sommeil : ligne « Où saisir l'agenda du sommeil » de l'épique.
- Retirer le relevé de l'accueil Android : ligne de l'épique.
- Choisir un autre jour qu'aujourd'hui.

## Piste technique

- `GET /api/releve?coucher_ms=…&lever_ms=…&coucher_souhaite_ms=…` : la page calcule les instants, le serveur
  appelle `relevé_du_jour` sur son historique.

## Critères d'acceptation

- [ ] Le serveur répond au relevé du jour pour trois instants donnés, et refuse une demande sans le bon jeton ou à
  laquelle il manque un instant.
- [ ] La page place les trois heures dans le temps comme le téléphone.
- [ ] La page affiche les quatre fenêtres du relevé, avec les textes de l'accueil Android.
