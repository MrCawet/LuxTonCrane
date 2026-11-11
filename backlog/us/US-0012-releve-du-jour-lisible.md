---
id: US-0012
type: us
titre: Le relevé du jour se lit d'un coup d'œil
statut: pret
priorite: confort
cree_le: 2026-11-11
maj_le: 2026-11-11
source: REQ-0002
emetteur: Opérateur
composants: [web]
tickets_lies: [REQ-0002, EP-0002, US-0009, US-0011]
objectif: aucun
niveau: 1
epique: EP-0002
valeur: Pour l'opérateur, voir le soir en quelques secondes ce que la nuit et la journée ont donné.
---

## Provenance

Troisième tranche de [EP-0002](../epiques/EP-0002-direction-artistique-et-experience-de-la-page.md), tirée de
`## À découper` (« Lisibilité du relevé du jour »). Demandée par l'opérateur en conversation le 2026-11-11 :
« Travaille maintenant le rapport », juste après avoir validé US-0009 et US-0011. Le « rapport » est le relevé du
jour du glossaire. Priorité proposée par l'agent, `confort` : le relevé se lit déjà.

## Besoin

**En tant que** opérateur, **je veux** lire le relevé du jour d'un coup d'œil, sur le téléphone comme sur
l'ordinateur, **afin de** voir vite quels objectifs sont tenus.

Aujourd'hui, chaque fenêtre est une suite de phrases (« Objectif tenu : non », « Couverture : 6 % ») collées les
unes aux autres, et rien ne distingue un objectif tenu d'un objectif non tenu.

## Comportement attendu

Hypothèses de l'agent, à confirmer à la revue :

- chaque fenêtre (et le sommeil, la fatigue) est une carte, comme les niveaux de l'agenda, avec son titre en gras ;
- chaque ligne se lit en deux temps : ce qu'on mesure, en petit et plus clair, puis sa valeur, en gras ;
- l'objectif se lit sur la ligne qu'il juge, par une étiquette « objectif tenu », en vert, ou « objectif non
  tenu », en rouge : la ligne « Objectif tenu : oui » disparaît ;
- aucune information du relevé n'est perdue (valeur la plus haute, couverture, objectif de plus de 7 h).

## Hors périmètre

Le réglage du coucher souhaité et le bouton « Voir le relevé du jour » ne changent pas. L'ordre des blocs selon le
moment de la journée reste à découper dans l'épique.

## Critères d'acceptation

- [ ] Chaque fenêtre du relevé est une carte, son titre se lit séparé de ses lignes.
- [ ] Chaque objectif du relevé se lit sur sa ligne, tenu en vert, non tenu en rouge.
- [ ] Le relevé dit toujours tout ce qu'il disait : notes de l'agenda, quantité de sommeil, temps au-dessus des
  seuils, valeur la plus haute, couverture.
- [ ] L'opérateur juge le relevé confortable à lire, sur le téléphone et sur l'ordinateur.
