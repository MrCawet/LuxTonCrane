---
id: US-0014
type: us
titre: Chaque fenêtre de lumière montre sa tendance des derniers jours
statut: a_valider
priorite: confort
cree_le: 2026-11-11
maj_le: 2026-11-11
source: interne
emetteur: Opérateur
composants: [serveur, web]
tickets_lies: [EP-0003, US-0013]
objectif: aucun
niveau: 1
epique: EP-0003
valeur: Pour l'opérateur, voir si la lumière de chaque fenêtre s'améliore d'un jour à l'autre.
---

## Provenance

Deuxième tranche de [EP-0003](../epiques/EP-0003-graphiques-du-releve.md), tirée de `## À découper`. L'opérateur a
choisi « les deux » (courbe et tendance) le 2026-11-11, puis a demandé d'enchaîner les tranches de l'épique sans
revue entre elles. Priorité proposée par l'agent, `confort`.

## Besoin

**En tant que** opérateur, **je veux** voir pour chaque fenêtre de lumière le chiffre principal des derniers jours,
**afin de** voir la tendance et pas seulement la journée.

## Comportement attendu

Hypothèses de l'agent, à confirmer à la revue :

- les derniers jours sont les 7 dernières nuits notées, jusqu'à aujourd'hui : un jour sans agenda du matin n'a pas de
  relevé, il n'apparaît pas ;
- sous la courbe de chaque fenêtre, une petite barre par jour, du plus ancien au plus récent, avec le jour sous la
  barre ; la barre d'aujourd'hui est celle du relevé affiché ;
- la hauteur de la barre est le chiffre principal de la fenêtre : le temps au-dessus de 10 lx le soir, au-dessus de
  1 lx la nuit, le temps à 10 000 lx ou plus le matin, la part du temps mesuré à 250 lx ou plus la journée ;
- la barre est verte si l'objectif est tenu, rouge sinon ;
- le coucher suivant d'un jour passé est le coucher de la nuit notée le lendemain ; s'il n'y en a pas, l'heure du
  coucher souhaitée, le même soir.

## Hors périmètre

La tendance du sommeil et de la fatigue (US-0015), le choix du nombre de jours.

## Piste technique

Le serveur rend les relevés des 7 dernières nuits notées par une nouvelle route,
`GET /api/releves?jour=<AAAA-MM-JJ>&coucher_souhaite_ms=…` ; l'agenda donne ces nuits et le coucher qui suit chacune.

## Critères d'acceptation

- [x] Chaque fenêtre de lumière montre une barre par nuit notée des 7 derniers jours, avec son jour.
- [x] La barre d'un jour dont l'objectif est tenu se distingue de celle d'un jour dont il ne l'est pas.
- [x] Le coucher suivant d'un jour passé est celui noté le lendemain.
- [ ] L'opérateur juge la tendance lisible, sur le téléphone et sur l'ordinateur.
