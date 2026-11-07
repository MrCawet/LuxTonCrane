---
id: EP-0001
type: epique
titre: Le rapport sur le web
statut: en_cours
cree_le: 2026-11-06
maj_le: 2026-11-07
source: REQ-0001
emetteur: Opérateur
composants: [android, pont, releve, serveur, web]
tickets_lies: [REQ-0001]
objectif: aucun
tickets: [US-0001, US-0002, US-0003]
---

## Provenance

Tirée de [REQ-0001](../requetes/REQ-0001-rapport-dans-une-webapp.md), conversation du 2026-11-06. Qualifiée en
niveau 2 le 2026-11-06 : la direction est donnée, le parcours se construit tranche par tranche.

## Objectif

Le rapport se lit dans une application web plutôt que dans l'application Android. Le téléphone ne garde que les
capteurs et l'enregistrement des données, synchronisées régulièrement sur le serveur virtuel de mcProd (utilisateur
`lux-ton-crane`, derrière nginx). Pile : Vue.js + TypeScript + Tailwind, petit backend Rust, SQLite.

## Tickets

- [US-0001](../us/US-0001-mesures-envoyees-au-serveur.md) : les mesures arrivent sur le serveur. Terminée.
- [US-0002](../us/US-0002-releve-du-jour-sur-le-web.md) : le relevé du jour se lit sur la page web.
- [US-0003](../us/US-0003-accueil-android-sans-releve.md) : l'accueil Android montre la synchronisation à la place du
  relevé.

## À découper

- **Où saisir l'agenda du sommeil** (téléphone ou web) : change l'étape 3 des prochaines étapes de CONCEPTION.md.
  Attend US-0002.
