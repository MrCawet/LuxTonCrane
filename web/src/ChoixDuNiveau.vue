<script setup lang="ts">
import type { Niveau } from './echellesDuSommeil'

/** `nom` est celui du champ, `titre` celui que lit l'opérateur. */
defineProps<{ nom: string; titre: string; niveaux: Niveau[] }>()

const niveauChoisi = defineModel<string>({ required: true })
</script>

<template>
  <fieldset class="flex flex-col gap-3">
    <legend class="mb-3 text-lg font-semibold">{{ titre }}</legend>
    <!-- Le bouton rond est caché : toute la carte se touche, et c'est elle qui montre le choix. -->
    <label
      v-for="niveau in niveaux"
      :key="niveau.nom"
      class="flex cursor-pointer flex-col gap-1 rounded-xl border-2 border-dracula-comment p-4 has-checked:border-dracula-purple has-checked:bg-dracula-purple/20 has-focus-visible:outline-2 has-focus-visible:outline-dracula-purple has-focus-visible:outline-offset-2"
    >
      <input v-model="niveauChoisi" type="radio" :name="nom" :value="niveau.nom" class="sr-only" />
      <span class="font-semibold">{{ niveau.titre }}</span>
      <span class="text-base text-dracula-foreground/80">{{ niveau.cequiSestPassé }}</span>
    </label>
  </fieldset>
</template>
