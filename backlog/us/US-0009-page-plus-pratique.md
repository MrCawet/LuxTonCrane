---
id: US-0009
type: us
titre: La saisie de l'agenda se lit confortablement
statut: a_valider
priorite: confort
cree_le: 2026-11-09
maj_le: 2026-11-11
source: REQ-0002
emetteur: Opérateur
composants: [web]
tickets_lies: [REQ-0002, EP-0002]
objectif: aucun
niveau: 1
epique: EP-0002
valeur: Pour l'opérateur, une page qu'il a envie d'ouvrir et qui sert vite, chaque matin et chaque soir.
---

## Provenance

Tirée de [REQ-0002](../requetes/REQ-0002-dose-du-matin-donnees-apparence.md), sujet 3, conversation du 2026-11-09.
Qualifiée d'abord en niveau 3 (investigation) : la gêne était dite, « austère, mais surtout peu pratique », pas ce
qui gênait. Requalifiée en niveau 2 le 2026-11-10 sur la réponse de l'opérateur (fil ci-dessous) : la direction est
la lisibilité, pas à pas ; la première tranche est l'agenda. Le 2026-11-11, la direction passe à l'épique
[EP-0002](../epiques/EP-0002-direction-artistique-et-experience-de-la-page.md) et ce ticket devient sa première
tranche, de niveau 1, comme le veut la norme. Priorité proposée par l'agent, `confort` : la page sert déjà, on s'en
passe sans dommage.

## Besoin

**En tant que** opérateur, **je veux** lire la saisie de l'agenda sans effort, sur le téléphone comme sur
l'ordinateur, **afin de** la remplir vite, le matin et le soir.

Aujourd'hui, les questions (les niveaux de chaque échelle et ce qui s'est passé) sont longues, collées les unes aux
autres, avec un petit bouton rond pour choisir.

## Comportement attendu

Validé par l'opérateur le 2026-11-11 :

- chaque niveau d'une échelle est une carte qu'on touche en entier : son titre sur une ligne, en gras, et « ce qui
  s'est passé » dessous, en plus petit et plus clair ;
- la carte choisie se voit d'un coup d'œil (bordure et fond marqués) ;
- un texte plus grand et plus aéré : interligne, marges, espace entre les cartes et entre les blocs ;
- chaque bloc (agenda du matin, fatigue du soir, relevé) se détache des autres.

## Hors périmètre

Le relevé, l'ordre des blocs selon le moment de la journée, l'allure générale (couleurs, thème) : à découper dans
l'épique, après la revue de ce ticket.

## Critères d'acceptation

- [x] Sur le téléphone, chaque niveau de l'agenda du matin et de la fatigue du soir se choisit en touchant sa carte
  entière, et le choix se voit.
- [x] Le titre d'un niveau se lit séparé de sa description.
- [x] Sur l'ordinateur, la saisie de l'agenda est aussi confortable à lire que sur le téléphone.
- [ ] L'opérateur juge la saisie de l'agenda confortable à lire.

## Questions ouvertes

- Qu'est-ce qui est le plus peu pratique : la saisie du matin, la saisie du soir, la lecture du relevé, ou le
  passage de l'une à l'autre ?
- Sur quel appareil ouvres-tu surtout la page : le téléphone, ou aussi un ordinateur ?
  - Réponse de l'opérateur (2026-11-11) : `Téléphone et ordinateur.`
- « Austère » : as-tu un exemple d'application dont l'allure te plaît ?
  - Réponse de l'opérateur (2026-11-10) : « sur l'ux, je propose d'avancer pas a pas. Pour le moment, le point
    principal est la lisibilité. Ce n'es pas très confortable a lire, autant sur la saisie de l'agenda que les
    questions. »
- Comment ranger ce ticket de niveau 2, sans épique ?
  - Réponse de l'opérateur (2026-11-11) : `Ok pour l'épique mais le sujet est plutôt la DA et lux`
- Valides-tu les hypothèses du comportement attendu ?
  - Réponse de l'opérateur (2026-11-11) : `Oui, telles quelles.`
