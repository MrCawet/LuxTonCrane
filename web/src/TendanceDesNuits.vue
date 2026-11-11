<script setup lang="ts">
import { computed } from 'vue'

/**
 * Un jour de la tendance : `jour` est celui du lever (`AAAA-MM-JJ`), `texte` la valeur telle qu'elle s'écrit
 * (`20 min`), `nombre` la même sans son unité (`20`), assez court pour tenir au-dessus de la barre.
 */
export interface JourDeLaTendance {
  jour: string
  valeur: number
  texte: string
  nombre: string
  objectifTenu: boolean
}

/** `échelle` est la valeur d'une barre pleine ; une valeur plus grande agrandit l'échelle. */
const props = defineProps<{
  libellé: string
  jours: JourDeLaTendance[]
  échelle: number
  unité: string
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
  const objectif = jour.objectifTenu ? 'objectif tenu' : 'objectif non tenu'
  return `${jourCourt(jour.jour)} : ${jour.texte}, ${objectif}`
}

const valeurDuHaut = computed(() => {
  const valeurs = props.jours.map((jour) => jour.valeur)
  return Math.max(props.échelle, ...valeurs)
})

function hauteurDe(jour: JourDeLaTendance): string {
  return `${(jour.valeur / valeurDuHaut.value) * 100}%`
}
</script>

<template>
  <figure :aria-label="`Tendance des dernières nuits : ${libellé}`" class="flex flex-col gap-2">
    <figcaption class="text-base text-dracula-foreground/80">Les dernières nuits, en {{ unité }}</figcaption>
    <ol class="flex h-36 items-stretch gap-1">
      <li
        v-for="jour in jours"
        :key="jour.jour"
        :aria-label="étiquetteDe(jour)"
        class="flex min-w-0 flex-1 flex-col items-center gap-1"
      >
        <span class="text-sm font-semibold">{{ jour.nombre }}</span>
        <div class="flex w-full flex-1 items-end rounded-md bg-dracula-background">
          <div
            class="min-h-1 w-full rounded-md"
            :class="jour.objectifTenu ? 'bg-dracula-green' : 'bg-dracula-red'"
            :style="{ height: hauteurDe(jour) }"
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
