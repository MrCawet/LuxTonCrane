---
id: US-0009
type: us
titre: La page web se lit confortablement, à commencer par l'agenda
statut: nouveau
priorite: confort
cree_le: 2026-11-09
maj_le: 2026-11-10
source: REQ-0002
emetteur: Opérateur
composants: [web]
tickets_lies: [REQ-0002]
objectif: aucun
niveau: 2
valeur: Pour l'opérateur, une page qu'il a envie d'ouvrir et qui sert vite, chaque matin et chaque soir.
---

## Provenance

Tirée de [REQ-0002](../requetes/REQ-0002-dose-du-matin-donnees-apparence.md), sujet 3, conversation du 2026-11-09.
Qualifiée d'abord en niveau 3 (investigation) : la gêne était dite, « austère, mais surtout peu pratique », pas ce
qui gênait. Requalifiée en niveau 2 le 2026-11-10 sur la réponse de l'opérateur (fil ci-dessous) : la direction est
la lisibilité, pas à pas ; la première tranche est l'agenda. Priorité proposée par l'agent, `confort` : la page sert déjà, on s'en passe sans
dommage.

## Besoin

**En tant que** opérateur, **je veux** lire la page sans effort, sur le téléphone, **afin de** remplir l'agenda et lire
le relevé vite, le matin et le soir.

Première tranche : **la saisie de l'agenda**, du matin et du soir. Les questions (les niveaux de chaque échelle et ce
qui s'est passé) sont longues, collées les unes aux autres, avec un petit bouton rond pour choisir.

## Comportement attendu

Hypothèses de l'agent, à confirmer sur le téléphone :

- chaque niveau d'une échelle est une carte qu'on touche en entier : son titre sur une ligne, en gras, et « ce qui
  s'est passé » dessous, en plus petit et plus clair ;
- la carte choisie se voit d'un coup d'œil (bordure et fond marqués) ;
- un texte plus grand et plus aéré : interligne, marges, espace entre les cartes et entre les blocs ;
- chaque bloc (agenda du matin, fatigue du soir, relevé) se détache des autres.

## Hors périmètre

Le relevé, l'ordre des blocs selon le moment de la journée, l'allure générale (couleurs, thème) : tranches suivantes,
après la revue de celle-ci.

## Critères d'acceptation

- [ ] Sur le téléphone, chaque niveau de l'agenda du matin et de la fatigue du soir se choisit en touchant sa carte
  entière, et le choix se voit.
- [ ] Le titre d'un niveau se lit séparé de sa description.
- [ ] L'opérateur juge la saisie de l'agenda confortable à lire.

## À découper

- Lisibilité du relevé du jour : attend la revue de la première tranche.

## Questions ouvertes

- Qu'est-ce qui est le plus peu pratique : la saisie du matin, la saisie du soir, la lecture du relevé, ou le
  passage de l'une à l'autre ?
- Sur quel appareil ouvres-tu surtout la page : le téléphone, ou aussi un ordinateur ?
- « Austère » : as-tu un exemple d'application dont l'allure te plaît ?
  - Réponse de l'opérateur (2026-11-10) : « sur l'ux, je propose d'avancer pas a pas. Pour le moment, le point
    principal est la lisibilité. Ce n'es pas très confortable a lire, autant sur la saisie de l'agenda que les
    questions. »
