---
id: US-0006
type: us
titre: Le téléphone rappelle l'agenda le matin et vers 21 h
statut: a_valider
priorite: confort
cree_le: 2026-11-08
maj_le: 2026-11-08
source: REQ-0001
emetteur: Opérateur
composants: [android]
tickets_lies: [REQ-0001, US-0004, US-0005]
objectif: aucun
niveau: 1
epique: EP-0001
valeur: Pour l'opérateur, ne pas oublier de noter sa nuit et sa fatigue.
---

## Provenance

Décision de l'opérateur le 2026-11-08 : une page web ne peut pas rappeler seule, le téléphone notifie et ouvre la
page ; la saisie reste sur le web. Priorité proposée par l'agent, `confort` : l'agenda se remplit sans rappel. Attend
US-0004 et US-0005.

## Besoin

**En tant que** opérateur, **je veux** une notification le matin et vers 21 h, qui ouvre la page web, **afin de**
penser à remplir l'agenda.

Hypothèse : le rappel du matin sonne à 8 h, celui du soir à 21 h, à heure fixe, à l'heure locale du téléphone.

## Critères d'acceptation

- [x] Une notification le matin et une vers 21 h ouvrent la page web.

## Validation

À faire par l'opérateur, avec l'APK livré après la fusion :

1. Installer l'APK, puis ouvrir l'application une fois : c'est l'ouverture de l'accueil qui planifie les rappels.
2. Accepter les notifications quand Android le demande. Si elles ont été refusées un jour : Paramètres > Applications >
   LuxTonCrane > Notifications, et autoriser « Rappels de l'agenda du sommeil ».
3. À 8 h (à quelques minutes près), une notification « Note ta nuit dans l'agenda du sommeil. » ; un appui ouvre
   https://lux-ton-crane.cawet.net dans le navigateur.
4. À 21 h (à quelques minutes près), une notification « Note la fatigue de ta journée dans l'agenda du sommeil. »,
   qui ouvre la même page.
5. Le lendemain, les deux rappels reviennent sans rouvrir l'application, redémarrage du téléphone compris.

Dire aussi si l'heure fixe de 8 h convient (hypothèse de l'agent), ou s'il vaut mieux une autre heure.

## Questions ouvertes

- À quelle heure le matin : une heure fixe, ou au premier déverrouillage après le lever ?
  - Réponse provisoire de l'agent (2026-11-08), à faire valider par l'opérateur : une heure fixe, 8 h. Depuis
    Android 8, le déverrouillage (`ACTION_USER_PRESENT`) n'est plus annoncé à une application qui ne tourne pas ; il
    faudrait le guetter depuis un service de premier plan permanent.
