---
id: REQ-0001
type: requete
titre: Le rapport passe dans une webapp, l'application Android garde les capteurs
statut: qualifie
cree_le: 2026-11-06
maj_le: 2026-11-06
source: interne
emetteur: Opérateur
canal: conversation
recue_le: 2026-11-06
composants: [android, pont, releve, serveur, web]
tickets_lies: [EP-0001, US-0001]
tickets_derives: [EP-0001, US-0001]
resolution: qualifiee
---

## Provenance

Demande de l'opérateur en conversation avec l'agent, le 2026-11-06, recopiée ci-dessous. La dernière ligne est sa
correction, envoyée juste après. Qualifiée le 2026-11-06 en niveau 2 (direction) : une épique, et sa première tranche
en US.

## Verbatim

> Je veux passer le rapport sans une webapp plutôt quune app Android. On garde l'App juste pour les capteurs, enregistré les données. Elle sont synchronisée régulièrement sur le serveur. J'ai déjà créé le serv virtuelle sur mcProd, user lux-ton-crane.
> Front-end vuejs ts tailwind. Petit backend rust. DB MySQL.
>
> DB SQLite je veux dire

## Analyse

Le relevé du jour (et les rapports qui suivront) quitte l'application Android pour une application web. Le téléphone
ne fait plus que mesurer et enregistrer, puis synchronise régulièrement ses données vers un serveur. Le serveur
virtuel existe déjà sur mcProd (utilisateur `lux-ton-crane`). Pile : front Vue.js + TypeScript + Tailwind, petit
backend en Rust, base SQLite.

Le calcul du relevé est déjà en Rust (`releve/`) : le backend peut le reprendre tel quel.

## Questions ouvertes

- La webapp et l'API exposeront les mesures de lumière, et plus tard le sommeil, sur Internet. Comment protéger
  l'accès dans cette première tranche ?
  `Un bearer token? C'est nginx configurer pour le moment`
- Où vivent le backend et le front ?
  `Même dépôt` : une crate `serveur/` dans le workspace et un dossier `web/`.
- Découpage : une épique, et pour première tranche une US où les mesures arrivent sur le serveur, visibles sur une
  page minimale ?
  `Oui`

## Suite donnée

- [EP-0001](../epiques/EP-0001-le-rapport-sur-le-web.md) : la direction.
- [US-0001](../us/US-0001-mesures-envoyees-au-serveur.md) : la première tranche.
