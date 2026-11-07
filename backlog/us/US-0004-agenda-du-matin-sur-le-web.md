---
id: US-0004
type: us
titre: L'agenda du matin se remplit sur la page web, et le relevé prend ses heures
statut: en_cours
priorite: essentiel
cree_le: 2026-11-08
maj_le: 2026-11-08
source: REQ-0001
emetteur: Opérateur
composants: [serveur, web, ci]
tickets_lies: [REQ-0001, US-0002]
objectif: aucun
niveau: 1
epique: EP-0001
valeur: Pour l'opérateur, noter sa nuit une fois le matin, et que le relevé s'en serve sans ressaisir les heures.
---

## Provenance

Ligne « Où saisir l'agenda du sommeil » de [EP-0001](../epiques/EP-0001-le-rapport-sur-le-web.md). Décisions de
l'opérateur en conversation, le 2026-11-08 : l'agenda se saisit sur la page web ; le téléphone ne fait que le
rappeler (US-0006) ; l'heure du coucher souhaitée est un réglage unique, gardé sur le serveur ; découpage en US-0004
(matin), US-0005 (fatigue du soir), US-0006 (rappels), US-0007 (l'agenda dans le relevé). Le contenu de l'agenda
vient de CONCEPTION.md, section « Agenda du sommeil ».

## Besoin

**En tant que** opérateur, **je veux** noter chaque matin sur la page web mon endormissement, la continuité de ma
nuit et mes heures du coucher et du lever, **afin que** le relevé du jour se calcule avec elles.

## Comportement attendu

- La page présente l'agenda de la nuit qui finit ce matin : les deux échelles de CONCEPTION.md (quatre niveaux
  chacune, avec ce qui s'est passé), et les heures du coucher et du lever par pas de 15 min. Un bouton les garde.
- Une nuit déjà notée se retrouve préremplie, et se corrige : la nouvelle saisie remplace l'ancienne.
- Le serveur garde une nuit par matin, dans sa base SQLite.
- L'heure du coucher souhaitée est un réglage de la page, gardé sur le serveur, 23:00 par défaut.
- Le relevé du jour n'a plus de champs d'heure : il prend le coucher et le lever de l'agenda du matin, et le coucher
  souhaité du réglage. Sans agenda ce matin, il demande de le remplir d'abord.

Hypothèses :
- Une nuit se reconnaît à la date de son lever, celle du matin où on la note.
- Les heures se placent dans le temps comme sur le téléphone : le coucher est la veille s'il tombe après le lever ;
  le coucher souhaité est le soir du lever, ou le lendemain s'il tombe avant le lever.

## Hors périmètre

- Noter ou relire une autre nuit que celle de ce matin.
- La fatigue du soir : US-0005. Les rappels : US-0006. Les notes et la quantité de sommeil dans le relevé : US-0007.

## Piste technique

- Une crate `agenda/` : les deux échelles, la nuit, et leur stockage SQLite dans la base du serveur.
- `GET` et `POST /api/nuits/<jour>`, `GET` et `POST /api/reglages`.

## Critères d'acceptation

- [x] Le serveur garde une nuit par jour du lever, la rend, et la remplace ; il refuse un niveau inconnu.
- [x] Le serveur garde l'heure du coucher souhaitée, 23:00 tant qu'elle n'a pas été réglée.
- [ ] La page note l'agenda du matin, et le retrouve prérempli.
- [ ] Le relevé du jour prend les heures de l'agenda et le coucher souhaité du réglage, et demande l'agenda s'il manque.
- [x] `agenda/` est dans les chemins surveillés par l'intégration et la livraison du serveur.
