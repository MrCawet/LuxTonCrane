<script setup lang="ts">
import { computed } from 'vue'

/**
 * Un jour de la tendance : `jour` est celui du lever (`AAAA-MM-JJ`), `texte` la valeur telle qu'elle s'écrit
 * (`20 min`), `nombre` la même sans son unité (`20`), assez court pour tenir au-dessus de la barre.
 * Sans valeur notée, `objectifTenu` est `null` et le jour n'a pas de barre.
 */
export interface JourDeLaTendance {
  jour: string
  valeur: number
  texte: string
  nombre: string
  objectifTenu: boolean | null
}

/** Une valeur de référence, tracée en pointillés à travers les barres : l'objectif de 7 h de sommeil. */
export interface Repère {
  valeur: number
  texte: string
}

/**
 * `échelle` est la valeur d'une barre pleine ; une valeur plus grande agrandit l'échelle.
 * Sans `unité`, la valeur n'a pas d'unité : le niveau d'une note.
 */
const props = defineProps<{
  libellé: string
  jours: JourDeLaTendance[]
  échelle: number
  unité?: string
  repère?: Repère
}>()

const formatDuJourDeLaSemaine = new Intl.DateTimeFormat('fr-FR', { weekday: 'short' })

function dateDu(jour: string): Date {
  const [année, mois, quantième] = jour.split('-').map(Number)
  return new Date(année, mois - 1, quantième)
}

function jourDeLaSemaine(jour: string): string {
  return formatDuJourDeLaSemaine.format(dateDu(jour))
}

function quantièmeDu(jour: string): number {
  return dateDu(jour).getDate()
}

/** Le jour court, `sam. 7`, dans le fuseau du navigateur. */
function jourCourt(jour: string): string {
  return `${jourDeLaSemaine(jour)} ${quantièmeDu(jour)}`
}

function étiquetteDe(jour: JourDeLaTendance): string {
  if (jour.objectifTenu === null) {
    return `${jourCourt(jour.jour)} : ${jour.texte}`
  }
  const objectif = jour.objectifTenu ? 'objectif tenu' : 'objectif non tenu'
  return `${jourCourt(jour.jour)} : ${jour.texte}, ${objectif}`
}

/** `Endormissement, les dernières nuits`, avec l'unité s'il y en a une. */
const légende = computed(() => {
  const nom = props.libellé.charAt(0).toUpperCase() + props.libellé.slice(1)
  const unité = props.unité === undefined ? '' : `, en ${props.unité}`
  return `${nom}, les dernières nuits${unité}`
})

const valeurDuHaut = computed(() => {
  const valeurs = props.jours.map((jour) => jour.valeur)
  return Math.max(props.échelle, ...valeurs)
})

function hauteurDe(valeur: number): string {
  return `${(valeur / valeurDuHaut.value) * 100}%`
}
</script>

<template>
  <figure :aria-label="`Tendance des dernières nuits : ${libellé}`" class="flex flex-col gap-2">
    <figcaption class="flex justify-between gap-3 text-base text-dracula-foreground/80">
      <span>{{ légende }}</span>
      <span v-if="repère !== undefined" class="whitespace-nowrap text-dracula-pink">- - {{ repère.texte }}</span>
    </figcaption>
    <ol class="flex h-36 items-stretch gap-1">
      <li
        v-for="jour in jours"
        :key="jour.jour"
        :aria-label="étiquetteDe(jour)"
        class="flex min-w-0 flex-1 flex-col items-center gap-1"
      >
        <span class="text-sm font-semibold">{{ jour.nombre }}</span>
        <div class="relative flex w-full flex-1 items-end rounded-md bg-dracula-background">
          <div
            v-if="jour.objectifTenu !== null"
            class="min-h-1 w-full rounded-md"
            :class="jour.objectifTenu ? 'bg-dracula-green' : 'bg-dracula-red'"
            :style="{ height: hauteurDe(jour.valeur) }"
          />
          <div
            v-if="repère !== undefined"
            class="absolute inset-x-0 border-t-2 border-dashed border-dracula-pink"
            :style="{ bottom: hauteurDe(repère.valeur) }"
          />
        </div>
        <span class="flex flex-col items-center text-sm leading-tight text-dracula-foreground/80">
          <span>{{ jourDeLaSemaine(jour.jour) }}</span>
          <span>{{ quantièmeDu(jour.jour) }}</span>
        </span>
      </li>
    </ol>
  </figure>
</template>
