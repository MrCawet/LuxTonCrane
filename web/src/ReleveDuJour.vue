<script setup lang="ts">
import { computed, ref } from 'vue'
import { heuresDuRelevé } from './heuresDuReleve'

const props = defineProps<{ jeton: string }>()

/** La lumière du soir ou de la nuit, où l'objectif est de rester sous un seuil. */
interface LumièreÀÉviter {
  temps_au_dessus_du_seuil_ms: number
  valeur_la_plus_haute_lux: number | null
  objectif_tenu: boolean
  couverture: number
}

interface LumièreDuMatin {
  temps_a_10_000_lx_ou_plus_ms: number
  objectif_tenu: boolean
  couverture: number
}

interface LumièreDeLaJournée {
  part_du_temps_mesure_a_250_lx_ou_plus: number | null
  objectif_tenu: boolean
  couverture: number
}

interface RelevéDuJour {
  lumiere_du_soir: LumièreÀÉviter
  lumiere_de_la_nuit: LumièreÀÉviter
  lumiere_du_matin: LumièreDuMatin
  lumiere_de_la_journee: LumièreDeLaJournée
}

interface Fenêtre {
  titre: string
  lignes: string[]
}

// Les heures par défaut sont celles des roues de l'accueil Android.
const coucher = ref('23:00')
const lever = ref('07:00')
const coucherSouhaité = ref('23:00')
const relevé = ref<RelevéDuJour | null>(null)
const erreur = ref<string | null>(null)

async function voirLeRelevé() {
  const heures = heuresDuRelevé(coucher.value, lever.value, coucherSouhaité.value, new Date())
  const paramètres = `coucher_ms=${heures.coucherMs}&lever_ms=${heures.leverMs}&coucher_souhaite_ms=${heures.coucherSouhaitéMs}`
  const réponse = await fetch(`/api/releve?${paramètres}`, {
    headers: { Authorization: `Bearer ${props.jeton}` },
  })
  if (!réponse.ok) {
    erreur.value = `Le relevé n'a pas pu être lu : le serveur a répondu ${réponse.status}.`
    relevé.value = null
    return
  }
  erreur.value = null
  relevé.value = await réponse.json()
}

function minutes(duréeMs: number): string {
  return `${Math.floor(duréeMs / 60_000)} min`
}

function pourcentage(part: number): string {
  return `${Math.round(part * 100)} %`
}

function ouiOuNon(tenu: boolean): string {
  return tenu ? 'oui' : 'non'
}

function lignesDeLaLumièreÀÉviter(lumière: LumièreÀÉviter, seuil: string): string[] {
  const valeur = lumière.valeur_la_plus_haute_lux
  return [
    `Au-dessus de ${seuil} : ${minutes(lumière.temps_au_dessus_du_seuil_ms)}`,
    `Valeur la plus haute : ${valeur === null ? 'rien de mesuré' : `${valeur} lx`}`,
    `Objectif tenu : ${ouiOuNon(lumière.objectif_tenu)}`,
    `Couverture : ${pourcentage(lumière.couverture)}`,
  ]
}

// Les textes sont ceux de l'accueil Android.
const fenêtres = computed<Fenêtre[]>(() => {
  if (relevé.value === null) {
    return []
  }
  const matin = relevé.value.lumiere_du_matin
  const journée = relevé.value.lumiere_de_la_journee
  const partÀ250Lx = journée.part_du_temps_mesure_a_250_lx_ou_plus
  return [
    {
      titre: 'Soir, 3 h avant le coucher',
      lignes: lignesDeLaLumièreÀÉviter(relevé.value.lumiere_du_soir, '10 lx'),
    },
    {
      titre: 'Nuit, du coucher au lever',
      lignes: lignesDeLaLumièreÀÉviter(relevé.value.lumiere_de_la_nuit, '1 lx'),
    },
    {
      titre: 'Matin, 2 h après le lever',
      lignes: [
        `À 10 000 lx ou plus : ${minutes(matin.temps_a_10_000_lx_ou_plus_ms)}`,
        `Objectif tenu : ${ouiOuNon(matin.objectif_tenu)}`,
        `Couverture : ${pourcentage(matin.couverture)}`,
      ],
    },
    {
      titre: 'Journée, du lever à 3 h avant le coucher souhaité',
      lignes: [
        `Temps mesuré à 250 lx ou plus : ${partÀ250Lx === null ? 'rien de mesuré' : pourcentage(partÀ250Lx)}`,
        `Objectif tenu : ${ouiOuNon(journée.objectif_tenu)}`,
        `Couverture : ${pourcentage(journée.couverture)}`,
      ],
    },
  ]
})
</script>

<template>
  <section class="flex flex-col gap-4">
    <h2 class="text-xl font-semibold">Relevé du jour</h2>
    <div class="grid grid-cols-3 gap-3">
      <label class="flex flex-col gap-1">
        Coucher
        <input v-model="coucher" type="time" step="900" class="rounded border border-gray-400 p-2" />
      </label>
      <label class="flex flex-col gap-1">
        Lever
        <input v-model="lever" type="time" step="900" class="rounded border border-gray-400 p-2" />
      </label>
      <label class="flex flex-col gap-1">
        Coucher souhaité
        <input v-model="coucherSouhaité" type="time" step="900" class="rounded border border-gray-400 p-2" />
      </label>
    </div>
    <button type="button" class="rounded bg-gray-900 p-2 text-white" @click="voirLeRelevé">
      Voir le relevé du jour
    </button>

    <p v-if="erreur !== null" class="text-red-700">{{ erreur }}</p>

    <article v-for="fenêtre in fenêtres" :key="fenêtre.titre" class="flex flex-col gap-1">
      <h3 class="font-semibold">{{ fenêtre.titre }}</h3>
      <p v-for="ligne in fenêtre.lignes" :key="ligne">{{ ligne }}</p>
    </article>
  </section>
</template>
