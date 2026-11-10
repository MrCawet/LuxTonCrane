---
id: US-0008
type: us
titre: L'objectif du matin se tient aussi avec 2 h à 2 500 lx
statut: nouveau
priorite: essentiel
cree_le: 2026-11-09
maj_le: 2026-11-09
source: REQ-0002
emetteur: Opérateur
composants: [releve, serveur, web]
tickets_lies: [REQ-0002, US-0007]
objectif: aucun
niveau: 1
valeur: Pour l'opérateur, un objectif du matin qu'il peut tenir en hiver, par temps couvert, sans perdre l'appui des essais.
---

## Provenance

Tirée de [REQ-0002](../requetes/REQ-0002-dose-du-matin-donnees-apparence.md), sujet 1, conversation du 2026-11-09.
Sources vérifiées le même jour et ajoutées à RECHERCHE.md (« Une dose plus faible, plus longue »). L'opérateur
parlait de « 2000 ou 2500 » : le ticket retient 2 500 lx, seule dose trouvée dans les essais. Priorité proposée par
l'agent, `essentiel` : en hiver, l'objectif actuel n'est presque jamais tenu, et le relevé le dit faux chaque matin.

## Besoin

**En tant que** opérateur, **je veux** que l'objectif du matin soit tenu avec 30 min à 10 000 lx ou plus, **ou** avec
2 h à 2 500 lx ou plus, **afin de** tenir l'objectif par temps couvert avec une dose que les essais appuient.

## Comportement attendu

- Le relevé du matin montre le temps passé à 10 000 lx ou plus **et** le temps passé à 2 500 lx ou plus, dans la
  fenêtre du matin.
- L'objectif est tenu si l'une des deux doses est atteinte, et le relevé dit laquelle.
- Une mesure prise téléphone couvert ne compte pas, comme aujourd'hui.

Hypothèse : la fenêtre du matin reste « les 2 h après le lever ». La dose à 2 500 lx demande donc toute la fenêtre,
sans un instant de moins. Si l'usage montre que c'est intenable, la fenêtre ou la durée se rediscutent (les protocoles
donnent 2 500 lx pendant 1 à 2 h).

## Hors périmètre

- Une dose calculée en « lux × durée » : les sources ne la valident pas (RECHERCHE.md).
- Le seuil de 2 000 lx : pas de dose validée.

## Critères d'acceptation

- [ ] Le relevé montre le temps à 10 000 lx ou plus et le temps à 2 500 lx ou plus, dans les 2 h après le lever.
- [ ] L'objectif du matin est tenu avec 30 min à 10 000 lx ou plus, ou avec 2 h à 2 500 lx ou plus, et le relevé dit
  quelle dose l'a tenu.
- [ ] CONCEPTION.md (« Objectifs de lumière du matin et de la journée ») et GLOSSAIRE.md disent la nouvelle règle.
