---
id: US-0007
type: us
titre: Le relevé du jour montre les notes de l'agenda et la quantité de sommeil
statut: a_valider
priorite: essentiel
cree_le: 2026-11-08
maj_le: 2026-11-08
source: REQ-0001
emetteur: Opérateur
composants: [releve, serveur, web]
tickets_lies: [REQ-0001, US-0004, US-0005]
objectif: aucun
niveau: 1
epique: EP-0001
valeur: Pour l'opérateur, lire au même endroit la lumière et le sommeil qu'elle accompagne.
---

## Provenance

Découpage de l'agenda du sommeil validé par l'opérateur le 2026-11-08. Contenu : CONCEPTION.md, sections « Agenda
du sommeil » et « Relevé du jour » ; c'est le reste de l'étape 1 des prochaines étapes (« Restent les notes de
l'agenda »). Attend US-0004 et US-0005.

## Besoin

**En tant que** opérateur, **je veux** voir dans le relevé du jour l'endormissement, la continuité, la quantité de
sommeil et la fatigue, chacun avec son objectif tenu ou non, **afin de** les rapprocher de la lumière reçue.

## Comportement attendu

- Le relevé s'ouvre sur un bloc « Sommeil » : l'endormissement et la continuité notés ce matin, la quantité de
  sommeil (lever − coucher), chacun avec son objectif tenu ou non. Un bloc « Fatigue, le soir suivant », après la
  lumière de la nuit, donne la fatigue du même jour ; l'ordre des blocs est celui de CONCEPTION.md.

Hypothèses :
- Tant que la fatigue du jour n'est pas notée, le relevé l'écrit (« pas encore notée ») au lieu d'échouer.
- Exactement 7 h au lit ne tient pas l'objectif : il faut « plus de 7 h ».

## Critères d'acceptation

- [x] Le relevé montre les deux notes du matin et la fatigue, avec leur objectif tenu ou non.
- [x] Le relevé montre la quantité de sommeil (temps au lit), avec l'objectif de plus de 7 h.

## Validation

Sur https://lux-ton-crane.cawet.net, une fois la branche fusionnée et livrée :

1. Remplir l'agenda du matin (par exemple « Calme », « Réveil long », coucher 23:15, lever 07:00) et le garder.
2. Dans « Relevé du jour », cliquer sur « Voir le relevé du jour ». Le premier bloc, « Sommeil », montre
   « Endormissement : Calme, objectif tenu : oui », « Continuité de la nuit : Réveil long, objectif tenu : non » et
   « Quantité de sommeil : 7 h 45 min, objectif de plus de 7 h tenu : oui ».
3. Le bloc « Fatigue, le soir suivant » montre « Fatigue de la journée : pas encore notée » tant que la fatigue du soir
   n'est pas gardée. La noter (par exemple « Gênante »), puis revoir le relevé : « Fatigue de la journée : Gênante,
   objectif tenu : non ».
4. Les blocs de lumière restent les mêmes qu'avant.
