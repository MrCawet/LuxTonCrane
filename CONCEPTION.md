# Conception

Notes de conception provisoires, à confirmer par l'usage.

## Agenda du sommeil

Chaque matin, l'application demande deux notes, en un clic chacune, et l'heure du coucher. Comme on ne peut pas mesurer le temps en s'endormant, chaque niveau est défini par ce qui s'est passé, dont on se souvient sans montre : c'est une échelle à ancrages comportementaux.

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

### Heure du coucher

Une troisième question demande l'heure du coucher, avec une roue qui se déroule par pas de 15 min. Elle place la fenêtre de lumière du soir (les 3 h avant) et le début de la nuit.

### À décider

- L'heure du lever, qui marque la fin de la nuit.

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
- **Plage maximale : 4 095,9 lx.** Au-delà, la valeur plafonne : la lumière du jour dehors et une lampe de luminothérapie à 10 000 lx ne sont pas mesurables telles quelles.
- **Résolution : 0,54 lx.** Le seuil de nuit de 1 lx se distingue à peine de 0.
- Consommation : 0,09 mA.
- Non-réveil : il ne réveille pas le processeur, comme prévu.

### À vérifier sur un téléphone

- Le type de service de premier plan exigé depuis Android 14 (`specialUse` ?).
- Le comportement des surcouches des fabricants, qui peuvent arrêter le service.
- Le coût réel des réveils en batterie.
- Si le capteur plafonne vraiment à 4 096 lx dehors, ou si sa plage réelle est plus large que celle déclarée.
