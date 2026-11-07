<script setup lang="ts">
import { onMounted, ref } from 'vue'

// Le jeton reste dans ce navigateur : on ne le saisit qu'une fois.
const TOKEN_STORAGE_KEY = 'jeton'

interface Mesure {
  valeur_lux: number
  instant_ms: number
}

interface Résumé {
  nombre: number
  derniere: Mesure | null
}

const formatDeLHeure = new Intl.DateTimeFormat('fr-FR', { dateStyle: 'long', timeStyle: 'short' })

const jeton = ref(localStorage.getItem(TOKEN_STORAGE_KEY))
const jetonSaisi = ref('')
const jetonRefusé = ref(false)
const résumé = ref<Résumé | null>(null)
const erreur = ref<string | null>(null)

async function chargerLeRésumé() {
  if (jeton.value === null) {
    return
  }
  const réponse = await fetch('/api/mesures', {
    headers: { Authorization: `Bearer ${jeton.value}` },
  })
  if (réponse.status === 401) {
    localStorage.removeItem(TOKEN_STORAGE_KEY)
    jeton.value = null
    jetonRefusé.value = true
    return
  }
  if (!réponse.ok) {
    erreur.value = `Le serveur a répondu ${réponse.status}.`
    return
  }
  résumé.value = await réponse.json()
}

async function garderLeJeton() {
  const saisi = jetonSaisi.value.trim()
  if (saisi === '') {
    return
  }
  localStorage.setItem(TOKEN_STORAGE_KEY, saisi)
  jeton.value = saisi
  jetonSaisi.value = ''
  jetonRefusé.value = false
  await chargerLeRésumé()
}

function heureDe(mesure: Mesure): string {
  const instant = new Date(mesure.instant_ms)
  return formatDeLHeure.format(instant)
}

onMounted(chargerLeRésumé)
</script>

<template>
  <main class="mx-auto max-w-xl p-6 font-sans text-gray-900">
    <h1 class="mb-6 text-2xl font-semibold">LuxTonCrane</h1>

    <form v-if="jeton === null" class="flex flex-col gap-3" @submit.prevent="garderLeJeton">
      <p v-if="jetonRefusé" class="text-red-700">Jeton refusé par le serveur.</p>
      <label for="jeton">Jeton du serveur</label>
      <input id="jeton" v-model="jetonSaisi" type="password" class="rounded border border-gray-400 p-2" />
      <button type="submit" class="rounded bg-gray-900 p-2 text-white">Garder le jeton</button>
    </form>

    <p v-else-if="erreur !== null" class="text-red-700">{{ erreur }}</p>

    <section v-else-if="résumé !== null" class="flex flex-col gap-2 text-lg">
      <p>{{ résumé.nombre }} mesures reçues</p>
      <p v-if="résumé.derniere !== null">
        Dernière : {{ résumé.derniere.valeur_lux }} lx, le {{ heureDe(résumé.derniere) }}
      </p>
    </section>
  </main>
</template>
