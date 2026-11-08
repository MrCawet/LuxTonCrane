<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { CONTINUITÉ, ENDORMISSEMENT, FATIGUE, type Niveau } from './echellesDuSommeil'
import { coucherSouhaitéMs, jourLocal } from './heuresDuReleve'

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

/** Une note de l'agenda : `niveau` est le nom d'un niveau de son échelle. */
interface Note {
  niveau: string
  objectif_tenu: boolean
}

interface QuantitéDeSommeil {
  duree_ms: number
  objectif_tenu: boolean
}

interface RelevéDuJour {
  endormissement: Note
  continuite: Note
  quantite_de_sommeil: QuantitéDeSommeil
  lumiere_du_soir: LumièreÀÉviter
  lumiere_de_la_nuit: LumièreÀÉviter
  lumiere_du_matin: LumièreDuMatin
  lumiere_de_la_journee: LumièreDeLaJournée
  /** `null` tant qu'elle n'est pas notée, le soir. */
  fatigue: Note | null
}

interface Fenêtre {
  titre: string
  lignes: string[]
}

interface NuitJson {
  coucher_ms: number
  lever_ms: number
}

const coucherSouhaité = ref('23:00')
const relevé = ref<RelevéDuJour | null>(null)
const erreur = ref<string | null>(null)
const messageDuRéglage = ref<string | null>(null)

function entêtes(): HeadersInit {
  return { Authorization: `Bearer ${props.jeton}` }
}

async function chargerLeRéglage() {
  const réponse = await fetch('/api/reglages', { headers: entêtes() })
  if (réponse.ok) {
    const réglages: { coucher_souhaite: string } = await réponse.json()
    coucherSouhaité.value = réglages.coucher_souhaite
  }
}

async function réglerLeCoucherSouhaité() {
  const réponse = await fetch('/api/reglages', {
    method: 'POST',
    headers: entêtes(),
    body: JSON.stringify({ coucher_souhaite: coucherSouhaité.value }),
  })
  messageDuRéglage.value = réponse.ok
    ? 'Coucher souhaité gardé.'
    : `Le coucher souhaité n'a pas pu être gardé : le serveur a répondu ${réponse.status}.`
}

/** Le relevé de la nuit qui a fini ce matin, avec les heures de l'agenda et le coucher souhaité. */
async function voirLeRelevé() {
  const jour = jourLocal(new Date())
  const réponseDeLaNuit = await fetch(`/api/nuits/${jour}`, { headers: entêtes() })
  if (réponseDeLaNuit.status === 404) {
    erreur.value = "Remplis d'abord l'agenda du matin."
    relevé.value = null
    return
  }
  if (!réponseDeLaNuit.ok) {
    erreur.value = `L'agenda n'a pas pu être lu : le serveur a répondu ${réponseDeLaNuit.status}.`
    relevé.value = null
    return
  }
  const nuit: NuitJson = await réponseDeLaNuit.json()
  const finDeLaJournéeMs = coucherSouhaitéMs(coucherSouhaité.value, nuit.lever_ms)
  const paramètres = `jour=${jour}&coucher_souhaite_ms=${finDeLaJournéeMs}`
  const réponse = await fetch(`/api/releve?${paramètres}`, { headers: entêtes() })
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

function heuresEtMinutes(duréeMs: number): string {
  const minutesEnTout = Math.floor(duréeMs / 60_000)
  const heures = Math.floor(minutesEnTout / 60)
  return `${heures} h ${minutesEnTout % 60} min`
}

function pourcentage(part: number): string {
  return `${Math.round(part * 100)} %`
}

function ouiOuNon(tenu: boolean): string {
  return tenu ? 'oui' : 'non'
}

/** Le titre du niveau noté, tel que l'échelle l'écrit. */
function titreDuNiveau(échelle: Niveau[], nom: string): string {
  const niveau = échelle.find((candidat) => candidat.nom === nom)
  return niveau === undefined ? nom : niveau.titre
}

function ligneDeLaNote(titre: string, échelle: Niveau[], note: Note): string {
  return `${titre} : ${titreDuNiveau(échelle, note.niveau)}, objectif tenu : ${ouiOuNon(note.objectif_tenu)}`
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

onMounted(chargerLeRéglage)

// Les textes sont ceux qu'avait l'accueil Android.
const fenêtres = computed<Fenêtre[]>(() => {
  if (relevé.value === null) {
    return []
  }
  const matin = relevé.value.lumiere_du_matin
  const journée = relevé.value.lumiere_de_la_journee
  const partÀ250Lx = journée.part_du_temps_mesure_a_250_lx_ou_plus
  const quantité = relevé.value.quantite_de_sommeil
  const fatigue = relevé.value.fatigue
  // Les blocs suivent l'ordre de CONCEPTION.md, section « Relevé du jour ».
  return [
    {
      titre: 'Sommeil',
      lignes: [
        ligneDeLaNote('Endormissement', ENDORMISSEMENT, relevé.value.endormissement),
        ligneDeLaNote('Continuité de la nuit', CONTINUITÉ, relevé.value.continuite),
        `Quantité de sommeil : ${heuresEtMinutes(quantité.duree_ms)}, objectif de plus de 7 h tenu : ${ouiOuNon(quantité.objectif_tenu)}`,
      ],
    },
    {
      titre: 'Soir, 3 h avant le coucher',
      lignes: lignesDeLaLumièreÀÉviter(relevé.value.lumiere_du_soir, '10 lx'),
    },
    {
      titre: 'Nuit, du coucher au lever',
      lignes: lignesDeLaLumièreÀÉviter(relevé.value.lumiere_de_la_nuit, '1 lx'),
    },
    {
      titre: 'Fatigue, le soir suivant',
      lignes: [
        fatigue === null
          ? 'Fatigue de la journée : pas encore notée'
          : ligneDeLaNote('Fatigue de la journée', FATIGUE, fatigue),
      ],
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
    <div class="flex items-end gap-3">
      <label class="flex flex-col gap-1">
        Coucher souhaité
        <input
          v-model="coucherSouhaité"
          name="coucher_souhaite"
          type="time"
          step="900"
          class="rounded border border-gray-400 p-2"
        />
      </label>
      <button type="button" name="regler" class="rounded border border-gray-900 p-2" @click="réglerLeCoucherSouhaité">
        Garder
      </button>
    </div>
    <p v-if="messageDuRéglage !== null">{{ messageDuRéglage }}</p>
    <button type="button" name="voir" class="rounded bg-gray-900 p-2 text-white" @click="voirLeRelevé">
      Voir le relevé du jour
    </button>

    <p v-if="erreur !== null" class="text-red-700">{{ erreur }}</p>

    <article v-for="fenêtre in fenêtres" :key="fenêtre.titre" class="flex flex-col gap-1">
      <h3 class="font-semibold">{{ fenêtre.titre }}</h3>
      <p v-for="ligne in fenêtre.lignes" :key="ligne">{{ ligne }}</p>
    </article>
  </section>
</template>
