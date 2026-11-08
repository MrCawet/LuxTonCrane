---
id: EP-0001
type: epique
titre: Le rapport sur le web
statut: en_cours
cree_le: 2026-11-06
maj_le: 2026-11-08
source: REQ-0001
emetteur: Opérateur
composants: [android, pont, releve, serveur, web]
tickets_lies: [REQ-0001]
objectif: aucun
tickets: [US-0001, US-0002, US-0003, ANO-0001, US-0004, US-0005, US-0006, US-0007]
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
- [US-0002](../us/US-0002-releve-du-jour-sur-le-web.md) : le relevé du jour se lit sur la page web. Terminée.
- [US-0003](../us/US-0003-accueil-android-sans-releve.md) : l'accueil Android montre la synchronisation à la place du
  relevé. Terminée.
- [ANO-0001](../anomalies/ANO-0001-jeton-refuse-sans-que-l-accueil-le-dise.md) : le jeton est refusé sans que
  l'accueil le dise. Terminée.
- [US-0004](../us/US-0004-agenda-du-matin-sur-le-web.md) : l'agenda du matin se remplit sur la page web, et le
  relevé prend ses heures. À valider.
- [US-0005](../us/US-0005-fatigue-du-soir-sur-le-web.md) : la fatigue de la journée se note le soir sur la page web. À valider.
- [US-0006](../us/US-0006-rappels-de-l-agenda.md) : le téléphone rappelle l'agenda le matin et vers 21 h. À valider.
- [US-0007](../us/US-0007-agenda-dans-le-releve.md) : le relevé du jour montre les notes de l'agenda et la quantité
  de sommeil. À valider.

## À découper

Rien pour l'instant. Décision du 2026-11-08 : l'agenda du sommeil se saisit sur la page web, le téléphone le rappelle.
