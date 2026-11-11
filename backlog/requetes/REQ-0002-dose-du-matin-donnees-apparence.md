---
id: REQ-0002
type: requete
titre: Une dose du matin tenable en hiver, la croissance des mesures, l'apparence de la page
statut: qualifie
cree_le: 2026-11-09
maj_le: 2026-11-11
source: interne
emetteur: Opérateur
canal: conversation
recue_le: 2026-11-09
composants: [releve, serveur, web, android, journal_sqlite, pont]
tickets_lies: [US-0008, DT-0002, US-0009, EP-0002, US-0011, US-0012]
tickets_derives: [US-0008, DT-0002, US-0009, US-0011, US-0012]
resolution: qualifiee
---

## Provenance

Deux messages de l'opérateur en conversation avec l'agent, le 2026-11-09, recopiés ci-dessous. Qualifiée le
2026-11-09 en trois sujets : l'objectif alternatif du matin (US-0008, niveau 1), la croissance des mesures (DT-0002,
niveau 2) et l'apparence de la page (US-0009, niveau 3).

## Verbatim

> Pour l'objectif du matin a plus de 10000, a cette saison c'est casi impossible,le temps est souvent couvert. J'ai lu quune alternative a 2000 ou 2500 plus longue était possible ?

> Vérifie les source , update recherche, créer un ticket pour cette objectif alternatif.
> Rajoute en suites 2 autre ticket :
> On enregistre en permanence, une base de données sur le tel et sur le serveur. Il faut enticiper le moment où il faudra vider les vielles mesure et ne garder que les données consolider. Analyse la DB du serveur et fait des projections sur les deux machines.
>
> L'apparence est ostere, mais surtout peu pratique. Ouvre un ticket aussi a se sujet

## Analyse

1. **Dose du matin.** Les sources (RECHERCHE.md, « Une dose plus faible, plus longue ») confirment 2 500 lx pendant
   2 h comme dose d'origine de la luminothérapie, à peu près équivalente à 10 000 lx pendant 30 min. 2 000 lx n'a pas
   de dose validée. → [US-0008](../us/US-0008-objectif-du-matin-a-2500-lx.md).
2. **Croissance des mesures.** Analyse de la base du serveur et projections dans
   [DT-0002](../dette-technique/DT-0002-croissance-des-mesures.md).
3. **Apparence.** La gêne est dite (« austère », « peu pratique »), pas encore ce qui gêne ni où. →
   [US-0009](../us/US-0009-page-plus-pratique.md), niveau 3.

## Suite donnée

US-0008, DT-0002, US-0009.
