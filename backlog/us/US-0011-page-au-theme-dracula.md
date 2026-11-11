---
id: US-0011
type: us
titre: La page web prend le thème Dracula
statut: termine
priorite: confort
cree_le: 2026-11-11
maj_le: 2026-11-11
source: REQ-0002
emetteur: Opérateur
composants: [web]
tickets_lies: [REQ-0002, EP-0002, US-0009]
objectif: aucun
niveau: 1
epique: EP-0002
valeur: Pour l'opérateur, une page à l'allure qui lui plaît, qu'il a envie d'ouvrir le matin et le soir.
---

## Provenance

Deuxième tranche de [EP-0002](../epiques/EP-0002-direction-artistique-et-experience-de-la-page.md), choisie par
l'opérateur en conversation le 2026-11-11 : « J'ai envie de reprendre le thème Dracula
(https://draculatheme.com/?platforms=linux). Ça ferait la 2e tranche de l'épique. » Le même jour, l'opérateur a demandé
d'enchaîner US-0009 et ce ticket sans attendre la revue de US-0009 : c'est l'écart à la norme (§6 Niveaux), décidé par
lui. Priorité proposée par l'agent, `confort` : la page sert déjà.

## Besoin

**En tant que** opérateur, **je veux** que la page web reprenne le thème Dracula, **afin d'**avoir une page à
l'allure qui me plaît.

## Comportement attendu

Hypothèses de l'agent, à confirmer à la revue :

- la page est toujours sombre, dans la palette Dracula (fond `#282A36`, texte `#F8F8F2`), quel que soit le réglage
  du téléphone ou de l'ordinateur ;
- les blocs et les cartes se détachent du fond par la couleur `Current Line` (`#44475A`) ;
- la carte choisie et les boutons principaux prennent le violet (`#BD93F9`) ;
- les erreurs sont en rouge (`#FF5555`) ;
- la lisibilité gagnée par US-0009 est gardée : le texte reste bien contrasté sur le fond.

## Hors périmètre

La variante claire (Alucard), l'application Android, la mise en couleur des objectifs tenus ou non dans le relevé.

## Critères d'acceptation

- [x] La page, sur le téléphone et sur l'ordinateur, est dans les couleurs de la palette Dracula.
- [x] La carte choisie se voit toujours d'un coup d'œil.
- [x] L'opérateur reconnaît le thème Dracula et le juge agréable à lire.
