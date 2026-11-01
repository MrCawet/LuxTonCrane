# Conception

Notes de conception provisoires, à confirmer par l'usage.

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
