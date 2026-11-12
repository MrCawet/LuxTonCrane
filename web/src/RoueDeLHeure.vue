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
const hoursColumn = ref<HTMLElement | null>(null)
const minutesColumn = ref<HTMLElement | null>(null)
const SCROLL_SETTLE_MS = 120
let settleTimer: ReturnType<typeof setTimeout> | undefined

function centeredValue(column: HTMLElement, values: string[]): string {
  const itemHeight = (column.firstElementChild as HTMLElement).offsetHeight
  const rank = Math.round(column.scrollTop / itemHeight)
  const clampedRank = Math.min(Math.max(rank, 0), values.length - 1)
  return values[clampedRank]
}

function onScroll() {
  clearTimeout(settleTimer)
  settleTimer = setTimeout(() => {
    if (hoursColumn.value === null || minutesColumn.value === null) {
      return
    }
    // Les deux d'un coup : le modèle ne prend sa nouvelle valeur qu'au rendu suivant.
    const heureAuCentre = centeredValue(hoursColumn.value, HEURES)
    const minuteAuCentre = centeredValue(minutesColumn.value, minutes.value)
    heureChoisie.value = `${heureAuCentre}:${minuteAuCentre}`
  }, SCROLL_SETTLE_MS)
}

function scrollToValue(column: HTMLElement | null, values: string[], value: string, behavior: ScrollBehavior) {
  const rank = values.indexOf(value)
  if (column === null || rank === -1) {
    return
  }
  const itemHeight = (column.firstElementChild as HTMLElement).offsetHeight
  column.scrollTo({ top: rank * itemHeight, behavior })
}

function scrollToChoice(behavior: ScrollBehavior) {
  scrollToValue(hoursColumn.value, HEURES, heure.value, behavior)
  scrollToValue(minutesColumn.value, minutes.value, minute.value, behavior)
}

onMounted(() => scrollToChoice('instant'))
watch(heureChoisie, () => scrollToChoice('smooth'), { flush: 'post' })
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
        <button
          v-for="valeur in HEURES"
          :key="valeur"
          type="button"
          :value="valeur"
          :aria-pressed="valeur === heure"
          class="block h-12 w-full snap-center text-dracula-foreground/50 aria-pressed:font-semibold aria-pressed:text-dracula-foreground"
          @click="choisirLHeure(valeur)"
        >
          {{ valeur }}
        </button>
      </div>
      <div
        ref="minutesColumn"
        role="group"
        :aria-label="`${nom}, minutes`"
        class="roue h-36 snap-y snap-mandatory overflow-y-scroll py-12"
        @scroll="onScroll"
      >
        <button
          v-for="valeur in minutes"
          :key="valeur"
          type="button"
          :value="valeur"
          :aria-pressed="valeur === minute"
          class="block h-12 w-full snap-center text-dracula-foreground/50 aria-pressed:font-semibold aria-pressed:text-dracula-foreground"
          @click="choisirLaMinute(valeur)"
        >
          {{ valeur }}
        </button>
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
