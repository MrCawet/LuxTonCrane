---
id: US-0001
type: us
titre: Les mesures arrivent sur le serveur
statut: termine
priorite: essentiel
cree_le: 2026-11-06
maj_le: 2026-11-07
source: REQ-0001
emetteur: Opérateur
composants: [android, pont, serveur, web, ci]
tickets_lies: [REQ-0001]
objectif: aucun
niveau: 1
epique: EP-0001
valeur: Pour l'opérateur, poser le socle sur lequel le rapport quittera l'application Android.
---

## Provenance

Première tranche de [EP-0001](../epiques/EP-0001-le-rapport-sur-le-web.md), tirée de
[REQ-0001](../requetes/REQ-0001-rapport-dans-une-webapp.md) (conversation du 2026-11-06). Découpage validé par
l'opérateur le 2026-11-06. Priorité proposée par l'agent, `essentiel` : tout le reste de l'épique en dépend.

## Besoin

**En tant que** opérateur, **je veux** que les mesures de mon téléphone arrivent régulièrement sur mon serveur et
s'y voient sur une page web, **afin de** pouvoir ensuite y lire le rapport plutôt que dans l'application.

## Comportement attendu

- Le téléphone envoie régulièrement au serveur les mesures qu'il ne lui a pas encore envoyées. Il garde son journal.
- Le serveur garde les mesures reçues dans sa base SQLite.
- Une page web affiche le nombre de mesures reçues et la dernière.
- L'envoi comme la lecture exigent un jeton secret (bearer token).

Hypothèses :
- L'envoi a lieu environ toutes les 15 min, l'intervalle minimal d'Android pour une tâche périodique, quand le
  téléphone a du réseau.
- Une mesure se reconnaît à son instant : reçue deux fois, elle n'est gardée qu'une fois.
- Le jeton se saisit une fois sur l'accueil de l'application, qui le garde dans ses données privées : l'APK est
  publié dans les releases d'un dépôt public, un jeton compilé dedans serait public. L'adresse du serveur, qui n'est
  pas secrète, est écrite dans le code. Le serveur lit le jeton dans son environnement, hors du dépôt.
- La page demande le jeton une fois et le garde dans le navigateur.
- L'opérateur déploie le serveur et la page sur mcProd, derrière nginx ; l'agent ne déploie pas.

## Hors périmètre

- Le relevé du jour sur le web, l'agenda du sommeil, le retrait de l'accueil Android : lignes de l'épique.
- Comptes, plusieurs utilisateurs, plusieurs téléphones.
- Déploiement automatique sur mcProd.

## Piste technique

- Crate `serveur/` dans le workspace : elle peut reprendre `journal_sqlite/` pour garder les mesures.
- Dossier `web/` : Vue.js + TypeScript + Tailwind.
- Côté téléphone : une tâche périodique Android (WorkManager), et le pont expose les mesures pas encore envoyées.

## Critères d'acceptation

- [x] Le téléphone envoie au serveur les mesures pas encore envoyées, environ toutes les 15 min, quand il a du réseau.
- [x] Le serveur garde chaque mesure reçue dans sa base SQLite ; une mesure déjà reçue n'est pas gardée deux fois.
- [x] Une requête sans le bon jeton est refusée, à l'envoi comme à la lecture.
- [x] La page web affiche le nombre de mesures reçues et la dernière (son heure locale et sa valeur en lx).
- [x] `serveur/` et `web/` sont dans les chemins surveillés par `integration.yml`.
- [x] CLAUDE.md dit comment compiler et lancer le serveur et la page.

## Validation

Le 2026-11-07, l'opérateur constate que les mesures sont arrivées. Le journal nginx de mcProd montre 341 envois
`POST /api/mesures` du téléphone, tous acceptés (204), le dernier à 14 h 32, et la base du serveur pèse 6,3 Mo.
