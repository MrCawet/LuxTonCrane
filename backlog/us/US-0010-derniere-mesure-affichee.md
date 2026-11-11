---
id: US-0010
type: us
titre: La dernière mesure dans la notification, un widget et l'accueil
statut: qualifie
priorite: confort
cree_le: 2026-11-11
maj_le: 2026-11-11
source: REQ-0003
emetteur: Opérateur
composants: [android]
tickets_lies: [REQ-0003]
objectif: aucun
niveau: 1
valeur: Pour l'opérateur, savoir d'un coup d'œil combien de lumière il reçoit en ce moment, sans ouvrir la page web.
---

## Provenance

Dérivée de [REQ-0003](../requetes/REQ-0003-mesure-courante-affichee.md), le 2026-11-11. Priorité proposée par
l'agent : `confort`, la mesure et le relevé fonctionnent sans elle.

## Besoin

**En tant que** opérateur, **je veux** voir la dernière mesure dans la notification de la mesure, dans un widget de
l'écran d'accueil et sur l'accueil de l'application, **afin de** savoir combien de lumière je reçois en ce moment.

## Comportement attendu

- La notification de la mesure affiche la dernière mesure, en lx.
- Un widget de la taille d'une icône, que l'opérateur pose sur l'écran d'accueil, affiche la dernière mesure, en lx.
- L'accueil de l'application affiche la dernière mesure, en lx, en énorme.
- Les trois suivent chaque nouvelle mesure.

Hypothèse : écran allumé, le capteur donne plusieurs valeurs par seconde, et Android ignore les mises à jour de
notification trop rapprochées : l'affichage suit au plus une mesure toutes les quelques secondes. Le journal, lui,
garde toujours chaque mesure.

## Hors périmètre

- La valeur dans l'icône de la barre du haut, ou dans la pastille d'Android 16.
- Toute autre information que la dernière mesure (heure, objectif, couleur selon le niveau…).
- Signaler que le service de mesure est arrêté.

## Piste technique

- Notification : le service reconstruit sa notification et la reposte sous le même numéro.
- Widget : un `AppWidgetProvider` et ses `RemoteViews`, mis à jour par le service ; sans nouvelle dépendance.
- Accueil : l'accueil écoute la dernière mesure, comme il écoute déjà la dernière synchronisation.
- Le SDK Android n'est pas joignable depuis le cloud : la vérification visuelle se fait sur le téléphone.

## Critères d'acceptation

- [ ] La notification de la mesure affiche la dernière mesure, en lx, et la suit.
- [ ] Un widget de la taille d'une icône affiche la dernière mesure, en lx, et la suit.
- [ ] L'accueil de l'application affiche la dernière mesure en énorme, en lx, et la suit.

## Questions ouvertes

- « En gros » : la dernière mesure s'écrit-elle en grand sur l'accueil, ou est-ce « pour résumer » ?
  ```
  Je veux la valeur dans la notif, dans un widget de la taille d'un icon sur l'écran d'accueil, et en énorme quand on ouvre l'appli luxtoncrane
  ```
- La dernière mesure s'affiche-t-elle avec son heure ? Écran éteint, elle a jusqu'à 9 min ; service arrêté, elle
  peut être bien plus vieille.
  ```
  Normalement écran allumé l'appli prend les mesures en temps réel non? A chaque changement annoncé par le capteurs ?
  ```
  - Écran allumé, la valeur affichée est celle du moment ; elle ne reste figée que si le service de mesure s'arrête.
    Que faire de l'heure de la mesure : rien, ou signaler l'arrêt du service ?
    ```
    Rien
    ```
- Le téléphone couvert : la dernière mesure s'affiche-t-elle quand même, ou un mot le dit-il à sa place ?
