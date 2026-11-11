<script setup lang="ts">
import { onMounted, ref } from 'vue'
import ChoixDuNiveau from './ChoixDuNiveau.vue'
import { FATIGUE } from './echellesDuSommeil'
import { jourLocal } from './heuresDuReleve'

const props = defineProps<{ jeton: string }>()

// La fatigue de la journée d'aujourd'hui, le même jour que la nuit qui l'a précédée.
const jour = jourLocal(new Date())

const fatigue = ref('')
const message = ref<string | null>(null)

function entêtes(): HeadersInit {
  return { Authorization: `Bearer ${props.jeton}` }
}

async function chargerLaFatigue() {
  const réponse = await fetch(`/api/journees/${jour}`, { headers: entêtes() })
  if (réponse.ok) {
    const journée: { fatigue: string } = await réponse.json()
    fatigue.value = journée.fatigue
  }
}

async function garderLaFatigue() {
  if (fatigue.value === '') {
    message.value = 'Choisis un niveau de fatigue.'
    return
  }
  const réponse = await fetch(`/api/journees/${jour}`, {
    method: 'POST',
    headers: entêtes(),
    body: JSON.stringify({ fatigue: fatigue.value }),
  })
  message.value = réponse.ok ? 'Fatigue gardée.' : `La fatigue n'a pas pu être gardée : le serveur a répondu ${réponse.status}.`
}

onMounted(chargerLaFatigue)
</script>

<template>
  <section class="flex flex-col gap-5 rounded-2xl bg-dracula-current-line p-5">
    <h2 class="text-2xl font-semibold">Fatigue du soir</h2>
    <form class="flex flex-col gap-6" @submit.prevent="garderLaFatigue">
      <ChoixDuNiveau v-model="fatigue" nom="fatigue" titre="Fatigue de la journée" :niveaux="FATIGUE" />
      <button type="submit" class="rounded-lg bg-dracula-purple p-3 font-semibold text-dracula-background">Garder la fatigue</button>
      <p v-if="message !== null">{{ message }}</p>
    </form>
  </section>
</template>
