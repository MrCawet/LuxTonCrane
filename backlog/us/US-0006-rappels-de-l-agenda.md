---
id: US-0006
type: us
titre: Le téléphone rappelle l'agenda le matin et vers 21 h
statut: nouveau
priorite: confort
cree_le: 2026-11-08
maj_le: 2026-11-08
source: REQ-0001
emetteur: Opérateur
composants: [android]
tickets_lies: [REQ-0001, US-0004, US-0005]
objectif: aucun
niveau: 1
epique: EP-0001
valeur: Pour l'opérateur, ne pas oublier de noter sa nuit et sa fatigue.
---

## Provenance

Décision de l'opérateur le 2026-11-08 : une page web ne peut pas rappeler seule, le téléphone notifie et ouvre la
page ; la saisie reste sur le web. Priorité proposée par l'agent, `confort` : l'agenda se remplit sans rappel. Attend
US-0004 et US-0005.

## Besoin

**En tant que** opérateur, **je veux** une notification le matin et vers 21 h, qui ouvre la page web, **afin de**
penser à remplir l'agenda.

## Questions ouvertes

- À quelle heure le matin : une heure fixe, ou au premier déverrouillage après le lever ?

## Critères d'acceptation

- [ ] Une notification le matin et une vers 21 h ouvrent la page web.
