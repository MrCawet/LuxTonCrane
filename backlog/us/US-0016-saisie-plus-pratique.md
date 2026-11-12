---
id: US-0016
type: us
titre: Les heures se saisissent à la roue, et une saisie enregistrée se lit
statut: en_cours
priorite: confort
cree_le: 2026-11-12
maj_le: 2026-11-12
source: REQ-0004
emetteur: Opérateur
composants: [web, android]
tickets_lies: [REQ-0004, EP-0002, ANO-0002]
objectif: aucun
niveau: 1
epique: EP-0002
valeur: Pour l'opérateur, une saisie plus rapide le matin et le soir, et ce qu'il a noté lisible d'un coup d'œil.
---

## Provenance

Tirée de [REQ-0004](../requetes/REQ-0004-saisie-des-heures-plus-pratique.md), conversation du 2026-11-12. Les trois
points de la requête sont réunis en un seul ticket : ils changent les mêmes formulaires. Rangée par l'agent dans
[EP-0002](../epiques/EP-0002-direction-artistique-et-experience-de-la-page.md), l'expérience de la page. Priorité
proposée par l'agent, `confort` : la page sert déjà.

## Besoin

**En tant que** opérateur, **je veux** choisir les heures sur une roue, et voir ce que j'ai enregistré à la place des
champs, **afin de** remplir la page plus vite et la relire d'un coup d'œil.

## Comportement attendu

- Le coucher, le lever et le coucher souhaité se choisissent sur une roue : une colonne pour les heures, une pour les
  minutes. Le coucher et le lever se choisissent à la minute près (ANO-0002), le coucher souhaité au quart d'heure,
  comme aujourd'hui.
- L'agenda du matin, la fatigue du soir et le coucher souhaité, une fois enregistrés (ou retrouvés sur le serveur),
  s'affichent en valeurs à la place des champs, avec un bouton « Modifier » qui rend les champs.
- Les boutons qui envoient une saisie disent « Enregistrer », sans complément, sur la page web comme sur l'accueil
  Android ; leur message de confirmation dit « enregistré ».

## Hors périmètre

Les roues de l'application Android (elle ne demande plus d'heure), le bouton « Modifier » sur le jeton.

## Critères d'acceptation

- [x] Les boutons qui envoient une saisie disent « Enregistrer », sur la page web et sur l'accueil Android.
- [ ] Le coucher et le lever se choisissent à la roue, à la minute près ; le coucher souhaité au quart d'heure.
- [ ] Une fois enregistrés ou retrouvés, l'agenda du matin, la fatigue du soir et le coucher souhaité s'affichent en
  valeurs, et « Modifier » rend les champs.
- [ ] L'opérateur juge la saisie plus pratique, sur le téléphone et sur l'ordinateur.

## Questions ouvertes

- Pourquoi « Garder » et pas « Enregistrer » ?
  - Réponse de l'agent (2026-11-12) : `Le bouton a repris le verbe des tickets (« garder les heures sur le serveur »), et « enregistrer » désignait déjà le téléphone qui enregistre la lumière. Personne n'avait comparé les deux mots.`
  - Quel verbe met-on sur ces boutons ?
    - Réponse de l'opérateur (2026-11-12) : `Enregistrer.`
  - Garde-t-on le complément (« Enregistrer l'agenda ») ?
    - Réponse de l'opérateur (2026-11-12) : `Enregistrer seul.`
  - Le bouton « Garder le jeton » de l'accueil Android change-t-il aussi ?
    - Réponse de l'opérateur (2026-11-12) : `Oui, aussi.`
- Quelles heures passent à la roue ?
  - Réponse de l'opérateur (2026-11-12) : `Les trois heures.`
- Quels formulaires s'affichent en valeurs, avec « Modifier » ?
  - Réponse de l'opérateur (2026-11-12) : `Les trois : l'agenda, la fatigue et le coucher souhaité.`
