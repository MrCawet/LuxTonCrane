<script setup lang="ts">
import { computed } from 'vue'
import {
  hauteurDeLaLumière,
  tracésDeLaCourbe,
  type FenêtreDuRelevé,
  type Palier,
  type PériodeMesurée,
} from './courbeDeLaLumiere'
import { heureLocale } from './heuresDuReleve'

/** `seuil` est le seuil tel que le relevé l'écrit, `10 000 lx` par exemple. */
const props = defineProps<{
  fenêtre: FenêtreDuRelevé
  périodes: PériodeMesurée[]
  seuilLx: number
  seuil: string
}>()

// Le graphique se dessine dans un carré de 100 sur 100, étiré à la taille de la carte.
const SIZE = 100

function x(position: number): number {
  return position * SIZE
}

function y(hauteur: number): number {
  return SIZE - hauteur * SIZE
}

/** La ligne d'un tracé : chaque palier à sa hauteur, reliés par des marches verticales. */
function ligneDu(tracé: Palier[]): string {
  const premier = tracé[0]
  const marches = tracé.map((palier) => `V ${y(palier.hauteur)} H ${x(palier.fin)}`)
  return `M ${x(premier.début)} ${y(premier.hauteur)} ${marches.join(' ')}`
}

/** L'aire sous un tracé, fermée jusqu'au bas du graphique. */
function aireDu(tracé: Palier[]): string {
  const premier = tracé[0]
  return `${ligneDu(tracé)} V ${SIZE} H ${x(premier.début)} Z`
}

const tracés = computed(() => tracésDeLaCourbe(props.fenêtre, props.périodes))
const hauteurDuSeuil = computed(() => y(hauteurDeLaLumière(props.seuilLx)))
</script>

<template>
  <figure class="flex flex-col gap-1">
    <svg
      role="img"
      :aria-label="`Courbe de la lumière, seuil ${seuil}`"
      :viewBox="`0 0 ${SIZE} ${SIZE}`"
      preserveAspectRatio="none"
      class="h-28 w-full rounded-lg bg-dracula-background"
    >
      <template v-for="(tracé, position) in tracés" :key="position">
        <path :d="aireDu(tracé)" class="fill-dracula-yellow/20" />
        <path
          :d="ligneDu(tracé)"
          class="fill-none stroke-dracula-yellow stroke-2"
          vector-effect="non-scaling-stroke"
        />
      </template>
      <line
        x1="0"
        :x2="SIZE"
        :y1="hauteurDuSeuil"
        :y2="hauteurDuSeuil"
        class="stroke-dracula-pink stroke-2"
        stroke-dasharray="6 4"
        vector-effect="non-scaling-stroke"
      />
    </svg>
    <figcaption class="flex justify-between text-base text-dracula-foreground/80">
      <span>{{ heureLocale(fenêtre.debut_ms) }}</span>
      <span class="text-dracula-pink">- - seuil {{ seuil }}</span>
      <span>{{ heureLocale(fenêtre.fin_ms) }}</span>
    </figcaption>
  </figure>
</template>
