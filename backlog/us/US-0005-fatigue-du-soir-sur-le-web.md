---
id: US-0005
type: us
titre: La fatigue de la journée se note le soir sur la page web
statut: termine
priorite: essentiel
cree_le: 2026-11-08
maj_le: 2026-11-11
source: REQ-0001
emetteur: Opérateur
composants: [serveur, web]
tickets_lies: [REQ-0001, US-0004]
objectif: aucun
niveau: 1
epique: EP-0001
valeur: Pour l'opérateur, savoir si la quantité de sommeil de la nuit précédente suffit.
---

## Provenance

Découpage de l'agenda du sommeil validé par l'opérateur le 2026-11-08 (voir [US-0004](US-0004-agenda-du-matin-sur-le-web.md)).
Contenu : CONCEPTION.md, section « Fatigue de la journée ». Le 2026-11-08, en la commençant, l'agent garde la
fatigue par jour, à part de la nuit : le soir, l'agenda du matin peut ne pas avoir été rempli, et la fatigue ne doit
pas en dépendre. Le jour est le même que celui de la nuit qui l'a précédée, ce qui garde leur rapprochement.

## Besoin

**En tant que** opérateur, **je veux** noter vers 21 h, en un clic, la fatigue de ma journée sur la page web, **afin
de** la comparer à la nuit qui l'a précédée.

## Comportement attendu

- La page présente la fatigue d'aujourd'hui : quatre niveaux (aucune, légère, gênante, somnolence), chacun avec ce
  qui s'est passé dans la journée, et un bouton qui la garde. Une fatigue déjà notée se retrouve et se corrige.

## Critères d'acceptation

- [x] La page note la fatigue du jour, sur les quatre niveaux de CONCEPTION.md, et la retrouve préremplie.
- [x] Le serveur la garde par jour, le même que celui de la nuit qui l'a précédée, même si cette nuit n'est pas notée.

## Validation

Le 2026-11-11, l'opérateur valide : « Tu peu les valider. »
