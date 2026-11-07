---
id: ANO-0001
type: anomalie
titre: Le jeton est refusé sans que l'accueil le dise
statut: a_valider
priorite: essentiel
cree_le: 2026-11-07
maj_le: 2026-11-07
source: interne
emetteur: Opérateur
composants: [android]
tickets_lies: [US-0001, US-0003]
objectif: aucun
niveau: 1
epique: EP-0001
---

## Provenance

Signalée par l'opérateur en conversation, le 2026-11-07 vers 20 h, avec une capture de l'accueil de l'APK
`2026-11-07-1604` (US-0003) : « si j'ajoute le token et clique sur garder le jeton, rien ne se passe (le champ se
vide). Synchro ne semble pas fonctionner non plus ». Qualifiée en anomalie par l'agent : l'accueil d'US-0001 devait
dire si les mesures sont envoyées, et celui d'US-0003 la dernière synchronisation réussie.

## Constat

D'après le journal nginx de mcProd :
- jusqu'à 17 h 36, le téléphone envoie ses mesures (`POST /api/mesures` en 204) ;
- de 17 h 50 à 18 h 14, nginx répond 502 : le serveur ferme la connexion sans répondre, ce qui arrive quand
  l'en-tête `Authorization` est illisible ;
- depuis 18 h 14, le serveur répond 401 : le jeton du téléphone n'est pas le sien.

Le jeton du serveur n'a pas changé depuis 11 h 29, et la page web l'utilise avec succès à 16 h 15. Le jeton gardé par
le téléphone a donc été remplacé par une saisie fausse. L'accueil n'en dit rien : « Garder le jeton » vide le champ
sans confirmer, accepte n'importe quel texte, et l'accueil affiche toujours « Jeton du serveur gardé : les mesures
sont envoyées. » alors que le serveur le refuse.

## Comportement attendu

- « Garder le jeton » refuse un jeton qui contient autre chose que des caractères ASCII visibles, et le dit.
- Un jeton gardé est confirmé, et une synchronisation part aussitôt.
- L'accueil affiche le dernier échec de synchronisation, avec son heure et sa raison : jeton refusé, ou serveur qui ne
  répond pas. Une synchronisation réussie l'efface.
- Un jeton refusé n'est pas retenté toutes les 15 min par une relance rapide : il attend la synchronisation suivante.

## Critères d'acceptation

- [x] Un jeton vide ou avec des caractères hors ASCII visible n'est pas gardé, et l'accueil le dit.
- [x] Un jeton gardé est confirmé, et une synchronisation part aussitôt.
- [x] L'accueil affiche le dernier échec (jeton refusé, ou serveur injoignable) jusqu'à la synchronisation réussie
  suivante.
