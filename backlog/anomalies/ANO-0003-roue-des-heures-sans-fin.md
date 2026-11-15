---
id: ANO-0003
type: anomalie
titre: La roue des heures s'arrête à 23 et à 00 au lieu de tourner en boucle
statut: a_valider
priorite: confort
cree_le: 2026-11-15
maj_le: 2026-11-15
source: interne
emetteur: Opérateur
composants: [web]
tickets_lies: [US-0016]
objectif: aucun
niveau: 0
epique: EP-0002
---

## Provenance

Signalée par l'opérateur en conversation, le 2026-11-15 : « la roue des heures et des minutes devrait être cycliques,
ce qui éviterai d'avoir a remonté tout en haut quand on se couche a 1h du matin par ex. ». Qualifiée en anomalie par
l'agent : la roue livrée par [US-0016](../us/US-0016-saisie-plus-pratique.md) doit tourner comme une roue. Priorité
proposée par l'agent, `confort` : la saisie marche, elle demande seulement de faire défiler toute la colonne.

## Critères d'acceptation

- [x] Sur la roue, après 23 vient 00 et avant 00 vient 23 ; de même pour les minutes, après la dernière vient 00.
- [ ] La roue se fait tourner sans fin dans les deux sens, au doigt comme à la molette.
