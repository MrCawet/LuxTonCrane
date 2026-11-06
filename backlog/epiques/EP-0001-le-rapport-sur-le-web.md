---
id: EP-0001
type: epique
titre: Le rapport sur le web
statut: en_cours
cree_le: 2026-11-06
maj_le: 2026-11-06
source: REQ-0001
emetteur: Opérateur
composants: [android, pont, releve, serveur, web]
tickets_lies: [REQ-0001]
objectif: aucun
tickets: [US-0001]
---

## Provenance

Tirée de [REQ-0001](../requetes/REQ-0001-rapport-dans-une-webapp.md), conversation du 2026-11-06. Qualifiée en
niveau 2 le 2026-11-06 : la direction est donnée, le parcours se construit tranche par tranche.

## Objectif

Le rapport se lit dans une application web plutôt que dans l'application Android. Le téléphone ne garde que les
capteurs et l'enregistrement des données, synchronisées régulièrement sur le serveur virtuel de mcProd (utilisateur
`lux-ton-crane`, derrière nginx). Pile : Vue.js + TypeScript + Tailwind, petit backend Rust, SQLite.

## Tickets

- [US-0001](../us/US-0001-mesures-envoyees-au-serveur.md) : les mesures arrivent sur le serveur.

## À découper

- **Le relevé du jour sur le web** : le backend le calcule avec `releve/`, la page l'affiche avec les heures du
  coucher, du lever et du coucher souhaité. Attend US-0001.
- **Où saisir l'agenda du sommeil** (téléphone ou web) : change l'étape 3 des prochaines étapes de CONCEPTION.md.
  Attend le relevé du jour sur le web.
- **Retirer l'affichage du relevé de l'accueil Android** : attend le relevé du jour sur le web.
