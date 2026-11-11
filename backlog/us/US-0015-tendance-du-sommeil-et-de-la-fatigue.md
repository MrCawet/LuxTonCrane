---
id: US-0015
type: us
titre: Le sommeil et la fatigue montrent leur tendance des derniers jours
statut: pret
priorite: confort
cree_le: 2026-11-11
maj_le: 2026-11-11
source: interne
emetteur: Opérateur
composants: [web]
tickets_lies: [EP-0003, US-0014]
objectif: aucun
niveau: 1
epique: EP-0003
valeur: Pour l'opérateur, voir si ses nuits et sa fatigue s'améliorent d'un jour à l'autre.
---

## Provenance

Troisième tranche de [EP-0003](../epiques/EP-0003-graphiques-du-releve.md), tirée de `## À découper`. L'opérateur a
choisi « les derniers jours » pour le sommeil et la fatigue le 2026-11-11, puis a demandé d'enchaîner les tranches
sans revue entre elles. Priorité proposée par l'agent, `confort`.

## Besoin

**En tant que** opérateur, **je veux** voir l'endormissement, la continuité de la nuit, la quantité de sommeil et la
fatigue des derniers jours, **afin de** voir leur tendance.

## Comportement attendu

Hypothèses de l'agent, à confirmer à la revue :

- les mêmes jours que la tendance de la lumière (US-0014) : les 7 dernières nuits notées ;
- pour chaque note (endormissement, continuité de la nuit, fatigue de la journée), une petite barre par jour, plus
  haute quand la note est meilleure (4 hauteurs, une par niveau), verte si l'objectif est tenu, rouge sinon ; une
  fatigue pas encore notée n'a pas de barre ;
- pour la quantité de sommeil, une barre par jour, haute de la durée, avec la ligne des 7 h en pointillés ;
- le jour sous chaque barre, comme pour la lumière.

## Hors périmètre

Le choix du nombre de jours, une moyenne sur la période.

## Piste technique

Les relevés des derniers jours sont déjà rendus par `GET /api/releves` (US-0014) : rien à changer sur le serveur.

## Critères d'acceptation

- [ ] Le bloc du sommeil montre la tendance de l'endormissement, de la continuité de la nuit et de la quantité de
  sommeil, une barre par nuit notée.
- [ ] Le bloc de la fatigue montre la tendance de la fatigue de la journée.
- [ ] Un objectif tenu se distingue d'un objectif non tenu.
- [ ] L'opérateur juge ces tendances lisibles, sur le téléphone et sur l'ordinateur.
