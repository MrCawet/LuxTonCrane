<script setup lang="ts">
import { onMounted, ref } from 'vue'
import ChoixDuNiveau from './ChoixDuNiveau.vue'
import { CONTINUITÉ, ENDORMISSEMENT } from './echellesDuSommeil'
import { erreursDeLAgendaDuMatin, raisonDuNavigateur } from './erreursDeSaisie'
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

const NOMS_DES_CHAMPS: Record<string, string> = { coucher: 'Coucher', lever: 'Lever' }

/** Le navigateur refuse un champ : la page en dit la raison au lieu de bloquer en silence. */
function champRefusé(évènement: Event) {
  const champ = évènement.target as HTMLInputElement
  message.value = null
  erreurs.value = [raisonDuNavigateur(NOMS_DES_CHAMPS[champ.name] ?? champ.name, champ)]
}

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
  message.value = réponse.ok ? 'Agenda gardé.' : `L'agenda n'a pas pu être gardé : le serveur a répondu ${réponse.status}.`
}

onMounted(chargerLaNuit)
</script>

<template>
  <section class="flex flex-col gap-5 rounded-2xl bg-white p-5 shadow-sm">
    <h2 class="text-2xl font-semibold">Agenda du matin</h2>
    <form class="flex flex-col gap-6" @submit.prevent="garderLAgenda" @invalid.capture="champRefusé">
      <ChoixDuNiveau
        v-for="échelle in ÉCHELLES"
        :key="échelle.champ"
        v-model="choix[échelle.champ]"
        :nom="échelle.champ"
        :titre="échelle.titre"
        :niveaux="échelle.niveaux"
      />
      <div class="grid grid-cols-2 gap-3">
        <label class="flex flex-col gap-1">
          Coucher
          <input v-model="coucher" name="coucher" type="time" class="rounded-lg border border-gray-400 p-3" />
        </label>
        <label class="flex flex-col gap-1">
          Lever
          <input v-model="lever" name="lever" type="time" class="rounded-lg border border-gray-400 p-3" />
        </label>
      </div>
      <button type="submit" class="rounded-lg bg-gray-900 p-3 font-semibold text-white">Garder l'agenda</button>
      <ul v-if="erreurs.length > 0" class="list-disc pl-5 text-red-700">
        <li v-for="erreur in erreurs" :key="erreur">{{ erreur }}</li>
      </ul>
      <p v-if="message !== null">{{ message }}</p>
    </form>
  </section>
</template>
