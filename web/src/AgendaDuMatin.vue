<script setup lang="ts">
import { onMounted, ref } from 'vue'
import ChoixDuNiveau from './ChoixDuNiveau.vue'
import RoueDeLHeure from './RoueDeLHeure.vue'
import { CONTINUITÉ, ENDORMISSEMENT } from './echellesDuSommeil'
import { erreursDeLAgendaDuMatin } from './erreursDeSaisie'
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
const erreurs = ref<string[]>([])

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

async function enregistrerLAgenda() {
  erreurs.value = erreursDeLAgendaDuMatin({ ...choix.value, coucher: coucher.value, lever: lever.value })
  message.value = null
  if (erreurs.value.length > 0) {
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
  message.value = réponse.ok ? 'Agenda enregistré.' : `L'agenda n'a pas pu être enregistré : le serveur a répondu ${réponse.status}.`
}

onMounted(chargerLaNuit)
</script>

<template>
  <section class="flex flex-col gap-5 rounded-2xl bg-dracula-current-line p-5">
    <h2 class="text-2xl font-semibold">Agenda du matin</h2>
    <form class="flex flex-col gap-6" @submit.prevent="enregistrerLAgenda">
      <ChoixDuNiveau
        v-for="échelle in ÉCHELLES"
        :key="échelle.champ"
        v-model="choix[échelle.champ]"
        :nom="échelle.champ"
        :titre="échelle.titre"
        :niveaux="échelle.niveaux"
      />
      <div class="grid grid-cols-2 gap-3">
        <RoueDeLHeure v-model="coucher" nom="Coucher" :pas-en-minutes="1" />
        <RoueDeLHeure v-model="lever" nom="Lever" :pas-en-minutes="1" />
      </div>
      <button type="submit" class="rounded-lg bg-dracula-purple p-3 font-semibold text-dracula-background">Enregistrer</button>
      <ul v-if="erreurs.length > 0" class="list-disc pl-5 text-dracula-red">
        <li v-for="erreur in erreurs" :key="erreur">{{ erreur }}</li>
      </ul>
      <p v-if="message !== null">{{ message }}</p>
    </form>
  </section>
</template>
