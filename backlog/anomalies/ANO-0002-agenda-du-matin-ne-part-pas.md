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

L'agenda se garde quelle que soit la minute du coucher et du lever. Une saisie refusée l'est avec sa raison, affichée
sur la page : la page ne bloque jamais en silence (demande de l'opérateur le 2026-11-09 : « Il faut aussi affiché
pourquoi c'est refusé, les erreurs de saisie »).

## Piste technique

Les champs d'heure du coucher et du lever portent `step="900"` : le navigateur juge invalide une heure comme 07:52 et
bloque l'envoi du formulaire, sans message visible sur Firefox Android, avant que la page ne s'exécute.

## Critères d'acceptation

- [x] Un coucher et un lever à la minute près (par exemple 23:07 et 07:52) se gardent depuis la page.
- [ ] L'agenda du matin refusé affiche toutes ses erreurs de saisie : note non choisie, heure vide, coucher et lever à
  la même heure ; rien n'est envoyé.
- [ ] Un champ refusé par le navigateur affiche sa raison sur la page.
- [ ] Un coucher souhaité vide n'est pas envoyé, et la page le dit.

## Validation

Sur https://lux-ton-crane.cawet.net, depuis le téléphone : remplir l'agenda du matin avec un lever qui n'est pas un
quart d'heure (par exemple 07:52), appuyer sur « Garder l'agenda » : la page affiche « Agenda gardé. », et le relevé du
jour reprend ces heures.
