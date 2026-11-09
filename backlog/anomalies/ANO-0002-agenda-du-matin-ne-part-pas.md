---
id: ANO-0002
type: anomalie
titre: L'agenda du matin ne part pas quand une heure n'est pas un quart d'heure
statut: en_cours
priorite: essentiel
cree_le: 2026-11-09
maj_le: 2026-11-09
source: interne
emetteur: Opérateur
composants: [web]
tickets_lies: [US-0004]
objectif: aucun
niveau: 1
epique: EP-0001
---

## Provenance

Signalée par l'opérateur en conversation, le 2026-11-09 au matin : « Je n'arrive pas a enregistré l'agenda du matin.
Après avoir rempli, je clique sur le bouton, rien ne se passe ». Qualifiée en anomalie par l'agent : US-0004 promet
que l'agenda du matin se garde.

## Comportement observé

Le journal nginx de mcProd montre, le 2026-11-09 entre 7 h 57 et 7 h 59, trois lectures `GET /api/nuits/2026-11-09`
(404, rien de noté) et aucun `POST` : la page n'envoie rien. Le 2026-11-08, avec des heures au quart d'heure, l'envoi
avait marché (`POST` en 204).

## Comportement attendu

L'agenda se garde quelle que soit la minute du coucher et du lever.

## Piste technique

Les champs d'heure du coucher et du lever portent `step="900"` : le navigateur juge invalide une heure comme 07:52 et
bloque l'envoi du formulaire, sans message visible sur Firefox Android, avant que la page ne s'exécute.

## Critères d'acceptation

- [ ] Un coucher et un lever à la minute près (par exemple 23:07 et 07:52) se gardent depuis la page.
