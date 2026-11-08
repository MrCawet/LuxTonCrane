<script setup lang="ts">
import { onMounted, ref } from 'vue'
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
  <section class="flex flex-col gap-4">
    <h2 class="text-xl font-semibold">Fatigue du soir</h2>
    <form class="flex flex-col gap-4" @submit.prevent="garderLaFatigue">
      <fieldset class="flex flex-col gap-2">
        <legend class="mb-1 font-semibold">Fatigue de la journée</legend>
        <label v-for="niveau in FATIGUE" :key="niveau.nom" class="flex items-start gap-2">
          <input v-model="fatigue" type="radio" name="fatigue" :value="niveau.nom" class="mt-1" />
          <span><strong>{{ niveau.titre }}</strong> : {{ niveau.cequiSestPassé }}</span>
        </label>
      </fieldset>
      <button type="submit" class="rounded bg-gray-900 p-2 text-white">Garder la fatigue</button>
      <p v-if="message !== null">{{ message }}</p>
    </form>
  </section>
</template>
