<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'

/** `nom` est celui que lit l'opérateur ; `pasEnMinutes` sépare deux minutes de la roue. */
const props = defineProps<{ nom: string; pasEnMinutes: number }>()

/** L'heure choisie, écrite « HH:MM ». */
const heureChoisie = defineModel<string>({ required: true })

const HEURES = Array.from({ length: 24 }, (_, heure) => deuxChiffres(heure))

const minutes = computed(() => {
  const nombreDeMinutes = 60 / props.pasEnMinutes
  return Array.from({ length: nombreDeMinutes }, (_, rang) => deuxChiffres(rang * props.pasEnMinutes))
})

// Tant que l'heure n'est pas chargée, la roue ne montre aucun choix.
const heure = computed(() => (heureChoisie.value ?? '').slice(0, 2))
const minute = computed(() => (heureChoisie.value ?? '').slice(3, 5))

function deuxChiffres(nombre: number): string {
  return String(nombre).padStart(2, '0')
}

function choisirLHeure(nouvelleHeure: string) {
  heureChoisie.value = `${nouvelleHeure}:${minute.value}`
}

function choisirLaMinute(nouvelleMinute: string) {
  heureChoisie.value = `${heure.value}:${nouvelleMinute}`
}

// La roue se fait tourner au doigt ou à la molette : quand elle s'arrête, la valeur au centre est choisie.
// Elle tourne sans fin : ses valeurs se répètent sur trois tours, et elle revient sur celui du milieu dès qu'elle
// s'arrête, pour en garder un de chaque côté.
const TURNS = [0, 1, 2]
const MIDDLE_TURN = 1
const hoursColumn = ref<HTMLElement | null>(null)
const minutesColumn = ref<HTMLElement | null>(null)
const SCROLL_SETTLE_MS = 120
let settleTimer: ReturnType<typeof setTimeout> | undefined

function itemHeightOf(column: HTMLElement): number {
  return (column.firstElementChild as HTMLElement).offsetHeight
}

function centeredValueBackOnMiddleTurn(column: HTMLElement, values: string[]): string {
  const itemHeight = itemHeightOf(column)
  const rank = Math.round(column.scrollTop / itemHeight)
  const rankInTurn = ((rank % values.length) + values.length) % values.length
  const rankOnMiddleTurn = MIDDLE_TURN * values.length + rankInTurn
  if (rank !== rankOnMiddleTurn) {
    column.scrollTo({ top: rankOnMiddleTurn * itemHeight, behavior: 'instant' })
  }
  return values[rankInTurn]
}

function onScroll() {
  clearTimeout(settleTimer)
  settleTimer = setTimeout(() => {
    if (hoursColumn.value === null || minutesColumn.value === null) {
      return
    }
    // Les deux d'un coup : le modèle ne prend sa nouvelle valeur qu'au rendu suivant.
    const heureAuCentre = centeredValueBackOnMiddleTurn(hoursColumn.value, HEURES)
    const minuteAuCentre = centeredValueBackOnMiddleTurn(minutesColumn.value, minutes.value)
    heureChoisie.value = `${heureAuCentre}:${minuteAuCentre}`
  }, SCROLL_SETTLE_MS)
}

function showOnMiddleTurn(column: HTMLElement | null, values: string[], value: string) {
  const rank = values.indexOf(value)
  if (column === null || rank === -1) {
    return
  }
  const rankOnMiddleTurn = MIDDLE_TURN * values.length + rank
  column.scrollTo({ top: rankOnMiddleTurn * itemHeightOf(column), behavior: 'instant' })
}

// Vers la copie la plus proche : de 00 à 23, la roue recule d'un cran au lieu de parcourir tout un tour.
function turnToNearest(column: HTMLElement | null, values: string[], value: string) {
  const rank = values.indexOf(value)
  if (column === null || rank === -1) {
    return
  }
  const itemHeight = itemHeightOf(column)
  const positions = TURNS.map((turn) => (turn * values.length + rank) * itemHeight)
  const distanceOf = (position: number) => Math.abs(position - column.scrollTop)
  const nearestPosition = positions.reduce((nearest, position) =>
    distanceOf(position) < distanceOf(nearest) ? position : nearest,
  )
  column.scrollTo({ top: nearestPosition, behavior: 'smooth' })
}

onMounted(() => {
  showOnMiddleTurn(hoursColumn.value, HEURES, heure.value)
  showOnMiddleTurn(minutesColumn.value, minutes.value, minute.value)
})
watch(
  heureChoisie,
  () => {
    turnToNearest(hoursColumn.value, HEURES, heure.value)
    turnToNearest(minutesColumn.value, minutes.value, minute.value)
  },
  { flush: 'post' },
)
</script>

<template>
  <fieldset class="flex flex-col gap-1">
    <legend class="mb-1">{{ nom }}</legend>
    <!-- La bande du milieu montre la valeur choisie ; une ligne au-dessus et une en dessous laissent voir la suite. -->
    <div class="relative grid grid-cols-2 gap-2 rounded-lg border border-dracula-comment bg-dracula-background">
      <div class="pointer-events-none absolute inset-x-1 top-12 h-12 rounded-md bg-dracula-purple/20" />
      <div
        ref="hoursColumn"
        role="group"
        :aria-label="`${nom}, heures`"
        class="roue h-36 snap-y snap-mandatory overflow-y-scroll py-12"
        @scroll="onScroll"
      >
        <template v-for="tour in TURNS" :key="tour">
          <button
            v-for="valeur in HEURES"
            :key="valeur"
            type="button"
            :value="valeur"
            :aria-pressed="tour === MIDDLE_TURN && valeur === heure"
            class="block h-12 w-full snap-center text-dracula-foreground/50 aria-pressed:font-semibold aria-pressed:text-dracula-foreground"
            @click="choisirLHeure(valeur)"
          >
            {{ valeur }}
          </button>
        </template>
      </div>
      <div
        ref="minutesColumn"
        role="group"
        :aria-label="`${nom}, minutes`"
        class="roue h-36 snap-y snap-mandatory overflow-y-scroll py-12"
        @scroll="onScroll"
      >
        <template v-for="tour in TURNS" :key="tour">
          <button
            v-for="valeur in minutes"
            :key="valeur"
            type="button"
            :value="valeur"
            :aria-pressed="tour === MIDDLE_TURN && valeur === minute"
            class="block h-12 w-full snap-center text-dracula-foreground/50 aria-pressed:font-semibold aria-pressed:text-dracula-foreground"
            @click="choisirLaMinute(valeur)"
          >
            {{ valeur }}
          </button>
        </template>
      </div>
    </div>
  </fieldset>
</template>

<style scoped>
/* Une roue n'a pas de barre de défilement : on la fait tourner. */
.roue {
  scrollbar-width: none;
}
.roue::-webkit-scrollbar {
  display: none;
}
</style>
