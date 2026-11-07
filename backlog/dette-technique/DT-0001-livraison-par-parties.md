---
id: DT-0001
type: dette-technique
titre: La livraison déploie seule, et seulement les parties modifiées
statut: en_cours
priorite: confort
cree_le: 2026-11-07
maj_le: 2026-11-07
source: interne
emetteur: Opérateur
composants: [ci, serveur, web, android]
tickets_lies: [US-0001]
objectif: aucun
niveau: 1
risque: faible
impact: Le serveur et la page se déploient à la main, chaque écriture confirmée, et l'APK ne sort que sur demande.
---

## Provenance

Demande de l'opérateur en conversation, le 2026-11-07, après le premier déploiement manuel d'US-0001 : « met a jour
la cd pour déployer le serveur et le front quand il y a des modifications dessus. Vois si c'est possible de faire en
sorte que la ci travaille que sur les parties modifier. Idem pour la cd ». La CI le fait déjà (jobs filtrés par
chemins) : seule la CD change. Priorité proposée par l'agent, `confort` : le déploiement manuel fonctionne.

## Constat

`livraison.yml` ne se lance qu'à la main et ne livre que l'APK. Le serveur et la page se copient à la main sur mcProd.

## Risque

Un déploiement oublié laisse en production une version qui n'est plus celle de `main`.

## Cible

À chaque fusion dans `main`, la livraison ne livre que ce qui a changé : le serveur, la page, l'application (release
avec un nouvel APK). Lancée à la main, elle livre tout.

Décisions de l'opérateur, le 2026-11-07 :
- l'APK sort aussi tout seul quand l'application ou le pont change ;
- GitHub joint mcProd par une clé SSH dédiée, autorisée sur le compte `lux-ton-crane` avec `restrict`, rangée dans
  les secrets `DEPLOIEMENT_CLE` et `DEPLOIEMENT_HOTE_CONNU` ; le serveur redémarre par systemd, sans root.

## Étapes

1. Repérer les parties modifiées entre le commit précédent de `main` et le nouveau.
2. Serveur : compiler sur `ubuntu-24.04` (glibc 2.39, sous les 2.41 de Debian 13), remplacer le binaire, arrêter le
   processus : systemd le relance.
3. Page : compiler et synchroniser `web/dist/` dans `public_html/`.
4. Application : la compilation signée et la release d'aujourd'hui.

## Critères d'acceptation

- [ ] Une fusion qui ne touche que `web/` déploie la page, et rien d'autre.
- [x] Une fusion qui touche `serveur/` déploie le serveur, qui répond ensuite.
- [x] Une fusion qui touche `android/` ou le pont crée une release avec l'APK.
- [ ] Lancée à la main, la livraison livre les trois.

## Suivi

- 2026-11-07, fusion de la PR #11 (livraison, donc tout) : serveur, page et application livrés, release
  `2026-11-07-1238`.
- 2026-11-07, fusion de la PR #12 (`serveur/`, `android/`, pont, sans `web/`) : serveur et application livrés,
  release `2026-11-07-1335`, page non livrée. Reste à voir une fusion qui ne touche que `web/`, et un lancement à la
  main.
