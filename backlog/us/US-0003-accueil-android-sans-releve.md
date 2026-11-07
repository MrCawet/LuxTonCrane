---
id: US-0003
type: us
titre: L'accueil Android montre la synchronisation à la place du relevé
statut: termine
priorite: essentiel
cree_le: 2026-11-07
maj_le: 2026-11-08
source: REQ-0001
emetteur: Opérateur
composants: [android, pont, ci]
tickets_lies: [REQ-0001, US-0002]
objectif: aucun
niveau: 1
epique: EP-0001
valeur: Pour l'opérateur, un téléphone qui ne fait plus que mesurer et envoyer, et qui dit quand il a envoyé.
---

## Provenance

Troisième tranche de [EP-0001](../epiques/EP-0001-le-rapport-sur-le-web.md), ligne « Retirer l'affichage du relevé
de l'accueil Android ». Demande de l'opérateur en conversation, le 2026-11-07, après la livraison d'US-0002 :

> Avant de poursuivre sur ces sujets, retire de l'appli Android ce qui est dans la version web. A la place, affiche
> la dernière synchro réussi, et un bouton pour synchroniser.

## Besoin

**En tant que** opérateur, **je veux** que l'accueil de l'application ne montre plus le relevé, mais la dernière
synchronisation réussie et un bouton pour synchroniser, **afin de** lire le relevé sur le web seulement, et de savoir
que mes mesures y sont.

## Comportement attendu

- L'accueil n'a plus les trois roues, ni le bouton « Voir le relevé du jour », ni le relevé.
- L'accueil affiche la date et l'heure de la dernière synchronisation réussie, ou qu'il n'y en a pas encore eu.
- Un bouton « Synchroniser maintenant » envoie tout de suite les mesures pas encore reçues, dès qu'il y a du réseau.
- L'affichage se met à jour dès qu'une synchronisation réussit, celle du bouton comme celle des 15 min.

Hypothèses :
- Une synchronisation réussie est une synchronisation où le serveur a tout reçu, même s'il n'y avait rien à envoyer.
  Sans jeton, rien n'est envoyé : ce n'est pas une synchronisation réussie.
- Le pont ne sert plus à calculer le relevé : ce calcul est retiré du pont, il reste dans `releve/` pour le serveur.

## Hors périmètre

- Afficher la dernière synchronisation sur la page web.
- Où saisir l'agenda du sommeil : ligne de l'épique.

## Critères d'acceptation

- [x] L'accueil n'affiche plus le relevé du jour, et le pont ne le calcule plus.
- [x] L'accueil affiche la dernière synchronisation réussie, ou « aucune pour l'instant ».
- [x] Le bouton « Synchroniser maintenant » envoie les mesures, et l'affichage suit.
- [x] La livraison redéploie le serveur quand `releve/` change, et ne sort plus d'APK pour `releve/` seul.

## Validation

Le 2026-11-08, l'opérateur valide l'accueil de l'APK `2026-11-07-1913` : « Sa me semble ok ».
