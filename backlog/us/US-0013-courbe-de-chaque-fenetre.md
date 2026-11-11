---
id: US-0013
type: us
titre: Chaque fenêtre de lumière du relevé montre sa courbe
statut: a_valider
priorite: confort
cree_le: 2026-11-11
maj_le: 2026-11-11
source: interne
emetteur: Opérateur
composants: [serveur, web]
tickets_lies: [EP-0003, US-0012]
objectif: aucun
niveau: 1
epique: EP-0003
valeur: Pour l'opérateur, voir quand la lumière a dépassé le seuil, et pas seulement combien de temps.
---

## Provenance

Première tranche de [EP-0003](../epiques/EP-0003-graphiques-du-releve.md), demandée par l'opérateur en conversation
le 2026-11-11 (« La courbe de la fenêtre », en réponse à la question de l'agent). Priorité proposée par l'agent,
`confort` : le relevé se lit déjà sans elle.

## Besoin

**En tant que** opérateur, **je veux** voir dans chaque fenêtre de lumière du relevé (soir, nuit, matin, journée)
la courbe de la lumière mesurée, **afin de** voir à quels moments elle a franchi le seuil.

## Comportement attendu

Hypothèses de l'agent, à confirmer à la revue :

- un petit graphique sous les lignes de chaque fenêtre de lumière : la lumière mesurée au fil de la fenêtre, de son
  début à sa fin ;
- l'échelle des lux est logarithmique (de 0,1 lx à plus de 10 000 lx, sinon la nuit serait une ligne plate) ;
- le seuil de la fenêtre est une ligne en pointillés (10 lx le soir, 1 lx la nuit, 10 000 lx le matin, 250 lx la
  journée), avec sa valeur ;
- le temps non mesuré (plus de 10 min sans mesure, ou téléphone couvert) est un trou dans la courbe, comme la
  couverture le compte ;
- l'heure de début et de fin de la fenêtre sous le graphique ;
- les couleurs du thème Dracula.

## Hors périmètre

La tendance des derniers jours, les graphiques du sommeil et de la fatigue (à découper dans l'épique), le zoom, le
survol d'un point.

## Piste technique

Le serveur ne rend aujourd'hui que les chiffres calculés. Il rend en plus, pour chaque fenêtre de lumière, ses
mesures valables, dans la réponse de `GET /api/releve`. Le graphique est un SVG écrit à la main : aucune
bibliothèque.

## Critères d'acceptation

- [x] Chaque fenêtre de lumière du relevé montre la courbe de ses mesures valables, du début à la fin de la fenêtre.
- [x] Le seuil de la fenêtre se lit sur le graphique.
- [x] Le temps non mesuré se voit comme un trou dans la courbe.
- [ ] L'opérateur juge le graphique lisible, sur le téléphone et sur l'ordinateur.
