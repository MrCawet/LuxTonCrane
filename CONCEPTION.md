# Conception

Notes de conception provisoires, à confirmer par l'usage.

## Mesure de la lumière

- La mesure n'a lieu que lorsque l'écran est allumé.
- Elle tourne dans un service de premier plan, avec une notification permanente : sans lui, Android ne transmet aucune valeur du capteur à une application en arrière-plan.
- Le capteur de lumière est ouvert quand l'écran s'allume et fermé quand il s'éteint.
- Chaque valeur que le capteur donne, quand la lumière change, est enregistrée. Il n'y a pas de relevé à intervalle fixe.

### À vérifier sur un téléphone

- Le type de service de premier plan exigé depuis Android 14 (`specialUse` ?).
- Le comportement des surcouches des fabricants, qui peuvent arrêter le service.
