# Conception

Notes de conception provisoires, à confirmer par l'usage.

## Agenda du sommeil

Chaque matin, l'application demande deux notes, en un clic chacune, puis les heures du coucher et du lever. Comme on ne peut pas mesurer le temps en s'endormant, chaque niveau est défini par ce qui s'est passé, dont on se souvient sans montre : c'est une échelle à ancrages comportementaux.

Les deux premiers niveaux correspondent à peu près aux objectifs de la National Sleep Foundation (Ohayon et al., 2017) : s'endormir en 30 min au plus, au plus 1 réveil court. C'est une correspondance approximative, pas une mesure.

### Endormissement

| Niveau | Ce qui s'est passé |
|---|---|
| Immédiat | je ne me souviens pas d'avoir attendu |
| Calme | j'ai attendu un peu, sans m'agiter ni me dire « je n'arrive pas à dormir » |
| Agité | je me suis retourné plusieurs fois, ou je me suis dit « je n'arrive pas à dormir » |
| Abandonné | j'ai renoncé un moment : je me suis levé, j'ai repris le téléphone ou j'ai rallumé la lumière |

Objectif : immédiat ou calme. Objectif de lumière lié : au plus 10 lx dans les 3 h avant le coucher.

### Continuité de la nuit

| Niveau | Ce qui s'est passé |
|---|---|
| Sans réveil | je ne me souviens d'aucun réveil |
| Réveil bref | réveillé, puis rendormi aussitôt, sans avoir le temps de penser |
| Réveil long | réveillé, et j'ai mis du temps à me rendormir : j'ai pensé, je me suis retourné |
| Nuit interrompue | je me suis levé, j'ai pris le téléphone, ou je suis resté éveillé longtemps |

Objectif : sans réveil ou réveil bref. Objectif de lumière lié : au plus 1 lx pendant la nuit.

Le niveau le plus bas de chaque échelle peut se recouper avec l'écran du téléphone, allumé après le coucher ou pendant la nuit.

### Heures du coucher et du lever

Deux questions demandent l'heure du coucher et l'heure du lever, chacune sur une roue qui se déroule par pas de 15 min. Le coucher place la fenêtre de lumière du soir (les 3 h avant) et le début de la nuit ; le lever marque la fin de la nuit.

### Quantité de sommeil

Calculée chaque nuit comme le temps au lit : l'heure du lever moins l'heure du coucher, données par l'agenda. Elle inclut l'endormissement et les réveils.

Objectif de départ : **plus de 7 h**, d'après le consensus AASM et SRS (2015) pour les adultes de 18 à 60 ans. Le besoin réel est inconnu : la fatigue de la journée dira s'il faut l'ajuster.

### Fatigue de la journée

Demandée chaque soir vers 21 h, en un clic, avec des niveaux définis par ce qui s'est passé. Les niveaux s'appuient sur des gestes plutôt que sur la concentration, que le TDAH brouille.

| Niveau | Ce qui s'est passé dans la journée |
|---|---|
| Aucune | je n'ai pas pensé à ma fatigue |
| Légère | je l'ai sentie, sans que ça change ce que je faisais |
| Gênante | elle m'a ralenti : j'ai dû faire une pause, prendre un café ou relire, ou je me suis forcé à continuer |
| Somnolence | j'ai lutté pour ne pas m'endormir, ou je me suis endormi sans le vouloir |

Objectif : aucune ou légère. Comparée à la quantité de sommeil de la nuit précédente, elle dit si cette quantité suffit.

## Relevé du jour

Le premier rapport. Il couvre une nuit et la journée qui suit.

Pourquoi dans cet ordre : l'agenda se remplit le matin, sur la nuit qui vient de passer, et la fatigue du soir se compare à la nuit d'avant. L'inverse (la journée, puis la nuit qui suit) garderait ensemble la lumière de la journée et la nuit qu'elle prépare, mais séparerait la nuit de la fatigue qu'elle cause. Aucun découpage ne garde tout ensemble. En contrepartie, le relevé n'est complet qu'au coucher suivant : c'est lui qui fixe la fin de la journée, 3 h avant.

| Bloc | Contenu |
|---|---|
| Sommeil | les notes d'endormissement et de continuité de la nuit, la quantité de sommeil, et pour chacune si l'objectif est tenu |
| Lumière du soir, de coucher − 3 h au coucher | le temps passé au-dessus de 10 lx, la valeur la plus haute, et si l'objectif est tenu |
| Lumière de la nuit, du coucher au lever | le temps passé au-dessus de 1 lx, la valeur la plus haute, et si l'objectif est tenu |
| Couverture | pour chaque fenêtre, la part du temps réellement mesurée |
| Fatigue, le soir suivant | la note, et si l'objectif est tenu |
| Lumière du matin, dans les 2 h après le lever | le temps passé à 10 000 lx ou plus, et si l'objectif est tenu |
| Lumière de la journée, du lever à coucher − 3 h | la part du temps mesuré à 250 lx ou plus, et si l'objectif est tenu |

### Le coucher suivant

La fin de la journée dépend du coucher suivant, qui n'a pas encore eu lieu quand on lit le relevé. On saisit donc l'**heure du coucher souhaitée** : le relevé se calcule avec elle. Le lendemain matin, l'heure du coucher réelle, saisie avec l'agenda, la remplace.

### Règles de calcul

- Une mesure vaut jusqu'à la suivante, pendant 10 min au plus. Au-delà, le temps compte comme non mesuré. 10 min couvrent l'écart entre deux réveils écran éteint (environ 9 min).
- La couverture d'une fenêtre est la part de sa durée couverte par une mesure valable. Une mesure prise téléphone couvert n'est pas valable.
- Pas de tolérance : une seule minute au-dessus du seuil suffit à ne pas tenir l'objectif du soir ou de la nuit, et une seule minute sous 250 lx suffit à ne pas tenir celui de la journée.

### Objectifs de lumière du matin et de la journée

- **Matin :** au moins 30 min à 10 000 lx ou plus, dans les 2 h après le lever. C'est la dose des essais et de la luminothérapie. Avec un retard de phase, une lumière vive reçue avant le point bas de l'horloge (environ 2 h avant le réveil naturel) peut retarder l'horloge : d'où la fenêtre après le lever.
- **Journée :** tout le temps mesuré, du lever jusqu'à 3 h avant le coucher, à 250 lx ou plus. Le consensus (Brown et al., 2022) dit « tout au long de la journée », sans part minimale.

### Téléphone couvert

Le capteur de proximité est lu avec chaque mesure de lumière. Une mesure prise téléphone couvert (dans une poche, face contre la table) compte comme non mesurée : elle baisse la couverture au lieu de faire manquer un objectif.

## Mesure de la lumière

- La mesure tourne dans un service de premier plan, avec une notification permanente : sans lui, Android ne transmet aucune valeur du capteur à une application en arrière-plan.
- Chaque relevé est noté avec son heure, quand on l'obtient. On ne cherche pas à avoir des relevés à intervalle régulier.

### Écran allumé

- Le capteur de lumière est ouvert quand l'écran s'allume.
- Chaque valeur que le capteur donne, quand la lumière change, est notée.

### Écran éteint

- Le capteur est fermé quand l'écran s'éteint.
- Une alarme réveille le téléphone environ toutes les 9 minutes : on ouvre le capteur, on note la première valeur, on le referme.
- 9 minutes est l'intervalle le plus court qu'Android accorde à une alarme pendant Doze. L'alarme n'a pas besoin d'être exacte : le système peut la décaler.
- L'alarme réveille le processeur, pas l'écran.
- Android ne garde le processeur éveillé que le temps de traiter l'alarme, alors que le capteur met un peu de temps à donner sa première valeur : un court wake lock est tenu jusqu'à cette valeur, puis relâché.
- À chaque réveil par l'alarme, on note aussi si le capteur de proximité est couvert ou non.
- À chaque réveil par l'alarme, on note aussi la position du téléphone, que donne l'accéléromètre : comparée à la précédente, elle dit si le téléphone a changé de position.
- Le capteur de mouvement significatif réveille le processeur, sans allumer l'écran, quand le téléphone bouge : on note l'heure de ce mouvement. Il se désactive après chaque mouvement signalé, on le réarme donc à chaque fois.

### Choix techniques

- Le journal est une base SQLite, écrite en Rust (`rusqlite`, avec SQLite compilé dedans).
- Kotlin appelle le métier en Rust à travers un pont généré par UniFFI.
- L'instant d'une mesure vient de l'horodatage du capteur, converti en heure réelle.
- Un échec d'enregistrement est écrit dans les journaux d'Android ; la mesure est perdue.

### Le capteur du Fairphone 6

Ce qu'Android déclare, relevé avec l'application « Sensor Test » :

- `stk_stk3bcx Ambient Light Sensor Non-wakeup`, fabricant Sensortek : le capteur standard de lumière (`TYPE_LIGHT`), sans canaux de couleur exposés.
- Plage maximale déclarée : 4 095,9 lx. Elle est fausse : sur ce téléphone, une autre application affiche plusieurs dizaines de milliers de lux dehors.
- **Résolution : 0,54 lx.** Le seuil de nuit de 1 lx se distingue à peine de 0.
- Consommation : 0,09 mA.
- Non-réveil : il ne réveille pas le processeur, comme prévu.

### À vérifier sur un téléphone

- Le type de service de premier plan exigé depuis Android 14 (`specialUse` ?).
- Le comportement des surcouches des fabricants, qui peuvent arrêter le service.
- Le coût réel des réveils en batterie.
- Que LuxTonCrane reçoit bien les valeurs au-delà de 4 096 lx, dehors.

## Prochaines étapes

Dans cet ordre, chaque étape en TDD et sur sa propre branche :

1. **Domaine Rust du relevé du jour** : fait pour la lumière (crate `releve/`) : les fenêtres tirées des heures du coucher et du lever, une mesure vaut 10 min au plus, les objectifs tenus ou non et la couverture. Restent les notes de l'agenda.
2. **Proximité avec chaque mesure** : faite. Chaque mesure est notée avec la dernière proximité connue ; écran éteint, la mesure attend la première valeur de proximité. Une mesure sans proximité connue (avant la première valeur, ou d'avant la proximité) reste valable.
3. **Agenda du sommeil dans l'application** : les deux notes et les deux roues le matin, la fatigue vers 21 h, avec leur enregistrement.
4. **Affichage du relevé du jour** : fait pour la lumière, sur la page web, avec trois champs d'heure (coucher, lever, coucher souhaité) dont les heures ne sont pas encore gardées. L'accueil Android ne l'affiche plus : il montre la dernière synchronisation réussie et permet de synchroniser tout de suite.
5. **Mesure écran éteint** : faite pour la lumière et la proximité, avec l'alarme environ toutes les 9 min et le court wake lock. Restent la position et le mouvement significatif décrits plus haut.

