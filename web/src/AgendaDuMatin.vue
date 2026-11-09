<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { CONTINUITÉ, ENDORMISSEMENT } from './echellesDuSommeil'
import { heureLocale, heuresDeLaNuit, jourLocal } from './heuresDuReleve'

const props = defineProps<{ jeton: string }>()

interface NuitJson {
  jour: string
  endormissement: string
  continuite: string
  coucher_ms: number
  lever_ms: number
}

const ÉCHELLES = [
  { champ: 'endormissement' as const, titre: 'Endormissement', niveaux: ENDORMISSEMENT },
  { champ: 'continuite' as const, titre: 'Continuité de la nuit', niveaux: CONTINUITÉ },
]

// La nuit qui finit ce matin se reconnaît à la date du lever, aujourd'hui.
const aujourdhui = new Date()
const jour = jourLocal(aujourdhui)

const choix = ref({ endormissement: '', continuite: '' })
const coucher = ref('23:00')
const lever = ref('07:00')
const message = ref<string | null>(null)

function entêtes(): HeadersInit {
  return { Authorization: `Bearer ${props.jeton}` }
}

async function chargerLaNuit() {
  const réponse = await fetch(`/api/nuits/${jour}`, { headers: entêtes() })
  if (!réponse.ok) {
    return
  }
  const nuit: NuitJson = await réponse.json()
  choix.value = { endormissement: nuit.endormissement, continuite: nuit.continuite }
  coucher.value = heureLocale(nuit.coucher_ms)
  lever.value = heureLocale(nuit.lever_ms)
}

async function garderLAgenda() {
  if (choix.value.endormissement === '' || choix.value.continuite === '') {
    message.value = 'Choisis un niveau pour l\'endormissement et pour la continuité.'
    return
  }
  const heures = heuresDeLaNuit(coucher.value, lever.value, aujourdhui)
  const réponse = await fetch(`/api/nuits/${jour}`, {
    method: 'POST',
    headers: entêtes(),
    body: JSON.stringify({
      endormissement: choix.value.endormissement,
      continuite: choix.value.continuite,
      coucher_ms: heures.coucherMs,
      lever_ms: heures.leverMs,
    }),
  })
  message.value = réponse.ok ? 'Agenda gardé.' : `L'agenda n'a pas pu être gardé : le serveur a répondu ${réponse.status}.`
}

onMounted(chargerLaNuit)
</script>

<template>
  <section class="flex flex-col gap-4">
    <h2 class="text-xl font-semibold">Agenda du matin</h2>
    <form class="flex flex-col gap-4" @submit.prevent="garderLAgenda">
      <fieldset v-for="échelle in ÉCHELLES" :key="échelle.champ" class="flex flex-col gap-2">
        <legend class="mb-1 font-semibold">{{ échelle.titre }}</legend>
        <label v-for="niveau in échelle.niveaux" :key="niveau.nom" class="flex items-start gap-2">
          <input
            v-model="choix[échelle.champ]"
            type="radio"
            :name="échelle.champ"
            :value="niveau.nom"
            class="mt-1"
          />
          <span><strong>{{ niveau.titre }}</strong> : {{ niveau.cequiSestPassé }}</span>
        </label>
      </fieldset>
      <div class="grid grid-cols-2 gap-3">
        <label class="flex flex-col gap-1">
          Coucher
          <input v-model="coucher" name="coucher" type="time" class="rounded border border-gray-400 p-2" />
        </label>
        <label class="flex flex-col gap-1">
          Lever
          <input v-model="lever" name="lever" type="time" class="rounded border border-gray-400 p-2" />
        </label>
      </div>
      <button type="submit" class="rounded bg-gray-900 p-2 text-white">Garder l'agenda</button>
      <p v-if="message !== null">{{ message }}</p>
    </form>
  </section>
</template>
