---
id: DT-0002
type: dette-technique
titre: Les mesures s'accumulent sans fin, sur le téléphone comme sur le serveur
statut: nouveau
priorite: essentiel
cree_le: 2026-11-09
maj_le: 2026-11-09
source: REQ-0002
emetteur: Opérateur
composants: [android, journal_sqlite, pont, serveur]
tickets_lies: [REQ-0002, US-0001]
objectif: aucun
niveau: 2
risque: eleve
impact: Chaque envoi, chaque relevé et chaque synchronisation relit toutes les mesures depuis le début ; en quelques semaines, le téléphone et le serveur ralentissent, puis manquent de mémoire.
---

## Provenance

Tirée de [REQ-0002](../requetes/REQ-0002-dose-du-matin-donnees-apparence.md), sujet 2, conversation du 2026-11-09 :
l'opérateur demande d'anticiper la purge des vieilles mesures, de ne garder que des données consolidées, et une
analyse de la base du serveur avec des projections sur les deux machines. Analyse faite par l'agent le 2026-11-09, sur
une copie en lecture de la base de production (chiffres agrégés seulement). Qualifiée en niveau 2 : le problème et la
direction sont clairs, le parcours se construit tranche par tranche. Priorité proposée `essentiel`, risque `eleve` :
rien ne casse aujourd'hui, mais le service se dégradera en quelques semaines.

## Constat

**La base du serveur, le 2026-11-09** (une seule table pèse : `mesure`)

| | Valeur |
|---|---|
| Mesures | 672 818, du 2026-11-05 au soir au 2026-11-09 au matin |
| Par jour complet | 183 000 à 236 000, environ **200 000** |
| Taille | 13,9 Mo, soit environ **21 octets par mesure** (aucun index) |
| Écart entre deux mesures | moins d'une seconde pour 99,9 % d'entre elles |
| Valeurs différentes | environ 38 000 sur toute la table |

L'écran allumé, le téléphone enregistre chaque évènement du capteur de lumière (`SENSOR_DELAY_NORMAL`), soit
plusieurs mesures par seconde, le plus souvent à la même valeur. Écran éteint, une mesure toutes les 9 min environ.

**Le téléphone** garde la même table, avec les mêmes mesures : rien n'est jamais effacé après l'envoi.

**Projections, au rythme actuel, sur chaque machine**

| Durée | Mesures | Taille de la table |
|---|---|---|
| 1 mois | 6 millions | 125 Mo |
| 6 mois | 36 millions | 760 Mo |
| 1 an | 73 millions | 1,5 Go |

**Ce qui coûte avant la place : tout est relu à chaque fois.** `historique()` charge toutes les mesures, triées, sans
index. Le serveur le fait pour chaque envoi du téléphone (pour écarter les doublons), pour chaque relevé et pour
`GET /api/mesures` ; le téléphone le fait à chaque synchronisation (`mesures_apres` lit tout, puis filtre). Une mesure
chargée occupe environ 24 octets en mémoire : environ 16 Mo par requête aujourd'hui, 150 Mo dans un mois, plusieurs
Go dans un an. Le téléphone manquera de mémoire bien avant de manquer de place.

**Le disque du serveur** est partagé entre tous les projets de mcProd et rempli à 87 % (27 Go libres). 1,5 Go par an
y pèse, sur un disque volontairement cher.

## Risque

- Dans quelques semaines : synchronisations et relevés lents, batterie du téléphone, puis échecs par manque de mémoire
  (synchronisation qui n'aboutit plus, serveur qui répond 502).
- Dans quelques mois : la place, sur le téléphone et sur le disque partagé du serveur.

## Cible

- On ne relit que ce dont on a besoin : la fenêtre du relevé, les mesures non encore envoyées.
- On n'enregistre pas plusieurs fois par seconde la même lumière.
- Les vieilles mesures sont remplacées par des données consolidées, qui donnent le même relevé ; le téléphone efface
  ce que le serveur a reçu.

## Étapes

1. **Première tranche : ne plus tout relire.** Un index sur `instant_ms` ; le relevé ne lit que sa fenêtre ; l'envoi
   écarte les doublons sans charger la table ; le téléphone ne lit que les mesures après la dernière reçue. Aucune
   donnée n'est effacée.
2. Ralentir l'enregistrement à la source, écran allumé. Hypothèse : au plus une mesure toutes les 10 s, et tout de
   suite quand la valeur ou la proximité change franchement. À trancher avec la règle « une mesure vaut 10 min au
   plus » de CONCEPTION.md.
3. Le téléphone efface les mesures que le serveur a reçues, au-delà de quelques jours.
4. Le serveur consolide les vieilles mesures (par exemple, par minute) et efface le détail ; le relevé d'un jour
   ancien doit rester le même.

Les étapes 2 à 4 s'écriront après la revue de la première. Les étapes 3 et 4 **effacent des données réelles** : elles
demandent l'accord de l'opérateur sur ce qui est gardé, et une sauvegarde avant la première purge.

## Critères d'acceptation

- [ ] (Tranche 1) Un relevé, un envoi et une synchronisation ne lisent plus toute la table : leur durée et leur
  mémoire ne croissent plus avec l'historique.
- [ ] (Tranche 1) Les relevés déjà affichés restent identiques.
