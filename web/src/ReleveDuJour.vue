<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import CourbeDeLaLumiere from './CourbeDeLaLumiere.vue'
import type { FenêtreDuRelevé, PériodeMesurée } from './courbeDeLaLumiere'
import { CONTINUITÉ, ENDORMISSEMENT, FATIGUE, type Niveau } from './echellesDuSommeil'
import { coucherSouhaitéMs, jourLocal } from './heuresDuReleve'

const props = defineProps<{ jeton: string }>()

/** La lumière du soir ou de la nuit, où l'objectif est de rester sous un seuil. */
interface LumièreÀÉviter {
  temps_au_dessus_du_seuil_ms: number
  valeur_la_plus_haute_lux: number | null
  objectif_tenu: boolean
  couverture: number
  fenetre: FenêtreDuRelevé
  periodes_mesurees: PériodeMesurée[]
}

interface LumièreDuMatin {
  temps_a_10_000_lx_ou_plus_ms: number
  objectif_tenu: boolean
  couverture: number
  fenetre: FenêtreDuRelevé
  periodes_mesurees: PériodeMesurée[]
}

interface LumièreDeLaJournée {
  part_du_temps_mesure_a_250_lx_ou_plus: number | null
  objectif_tenu: boolean
  couverture: number
  fenetre: FenêtreDuRelevé
  periodes_mesurees: PériodeMesurée[]
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

/** `nom` dit ce qui est visé : « objectif », ou « objectif de plus de 7 h ». */
interface Objectif {
  nom: string
  tenu: boolean
}

/** Une ligne du relevé : ce qu'on mesure, sa valeur, et l'objectif qui la juge s'il y en a un. */
interface Ligne {
  libellé: string
  valeur: string
  objectif?: Objectif
}

/** `seuil` est le seuil tel que le relevé l'écrit, `10 000 lx` par exemple. */
interface Courbe {
  fenêtre: FenêtreDuRelevé
  périodes: PériodeMesurée[]
  seuilLx: number
  seuil: string
}

interface Fenêtre {
  titre: string
  lignes: Ligne[]
  /** Seulement pour les fenêtres de lumière. */
  courbe?: Courbe
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
  if (coucherSouhaité.value === '') {
    messageDuRéglage.value = "Indique l'heure du coucher souhaité."
    return
  }
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

function étiquetteDe(objectif: Objectif): string {
  return objectif.tenu ? `${objectif.nom} tenu` : `${objectif.nom} non tenu`
}

/** Le titre du niveau noté, tel que l'échelle l'écrit. */
function titreDuNiveau(échelle: Niveau[], nom: string): string {
  const niveau = échelle.find((candidat) => candidat.nom === nom)
  return niveau === undefined ? nom : niveau.titre
}

function ligneDeLaNote(libellé: string, échelle: Niveau[], note: Note): Ligne {
  return {
    libellé,
    valeur: titreDuNiveau(échelle, note.niveau),
    objectif: { nom: 'objectif', tenu: note.objectif_tenu },
  }
}

function courbeDe(
  lumière: LumièreÀÉviter | LumièreDuMatin | LumièreDeLaJournée,
  seuilLx: number,
  seuil: string,
): Courbe {
  return { fenêtre: lumière.fenetre, périodes: lumière.periodes_mesurees, seuilLx, seuil }
}

function lignesDeLaLumièreÀÉviter(lumière: LumièreÀÉviter, seuil: string): Ligne[] {
  const valeur = lumière.valeur_la_plus_haute_lux
  return [
    {
      libellé: `Au-dessus de ${seuil}`,
      valeur: minutes(lumière.temps_au_dessus_du_seuil_ms),
      objectif: { nom: 'objectif', tenu: lumière.objectif_tenu },
    },
    { libellé: 'Valeur la plus haute', valeur: valeur === null ? 'rien de mesuré' : `${valeur} lx` },
    { libellé: 'Couverture', valeur: pourcentage(lumière.couverture) },
  ]
}

onMounted(chargerLeRéglage)

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
        {
          libellé: 'Quantité de sommeil',
          valeur: heuresEtMinutes(quantité.duree_ms),
          objectif: { nom: 'objectif de plus de 7 h', tenu: quantité.objectif_tenu },
        },
      ],
    },
    {
      titre: 'Soir, 3 h avant le coucher',
      lignes: lignesDeLaLumièreÀÉviter(relevé.value.lumiere_du_soir, '10 lx'),
      courbe: courbeDe(relevé.value.lumiere_du_soir, 10, '10 lx'),
    },
    {
      titre: 'Nuit, du coucher au lever',
      lignes: lignesDeLaLumièreÀÉviter(relevé.value.lumiere_de_la_nuit, '1 lx'),
      courbe: courbeDe(relevé.value.lumiere_de_la_nuit, 1, '1 lx'),
    },
    {
      titre: 'Fatigue, le soir suivant',
      lignes: [
        fatigue === null
          ? { libellé: 'Fatigue de la journée', valeur: 'pas encore notée' }
          : ligneDeLaNote('Fatigue de la journée', FATIGUE, fatigue),
      ],
    },
    {
      titre: 'Matin, 2 h après le lever',
      lignes: [
        {
          libellé: 'À 10 000 lx ou plus',
          valeur: minutes(matin.temps_a_10_000_lx_ou_plus_ms),
          objectif: { nom: 'objectif', tenu: matin.objectif_tenu },
        },
        { libellé: 'Couverture', valeur: pourcentage(matin.couverture) },
      ],
      courbe: courbeDe(matin, 10_000, '10 000 lx'),
    },
    {
      titre: 'Journée, du lever à 3 h avant le coucher souhaité',
      lignes: [
        {
          libellé: 'Temps mesuré à 250 lx ou plus',
          valeur: partÀ250Lx === null ? 'rien de mesuré' : pourcentage(partÀ250Lx),
          objectif: { nom: 'objectif', tenu: journée.objectif_tenu },
        },
        { libellé: 'Couverture', valeur: pourcentage(journée.couverture) },
      ],
      courbe: courbeDe(journée, 250, '250 lx'),
    },
  ]
})
</script>

<template>
  <section class="flex flex-col gap-5 rounded-2xl bg-dracula-current-line p-5">
    <h2 class="text-2xl font-semibold">Relevé du jour</h2>
    <div class="flex items-end gap-3">
      <label class="flex flex-col gap-1">
        Coucher souhaité
        <input
          v-model="coucherSouhaité"
          name="coucher_souhaite"
          type="time"
          step="900"
          class="rounded-lg border border-dracula-comment bg-dracula-background p-3"
        />
      </label>
      <button type="button" name="regler" class="rounded-lg border border-dracula-purple px-4 py-3 text-dracula-purple" @click="réglerLeCoucherSouhaité">
        Garder
      </button>
    </div>
    <p v-if="messageDuRéglage !== null">{{ messageDuRéglage }}</p>
    <button type="button" name="voir" class="rounded-lg bg-dracula-purple p-3 font-semibold text-dracula-background" @click="voirLeRelevé">
      Voir le relevé du jour
    </button>

    <p v-if="erreur !== null" class="text-dracula-red">{{ erreur }}</p>

    <article
      v-for="fenêtre in fenêtres"
      :key="fenêtre.titre"
      class="flex flex-col gap-3 rounded-xl border-2 border-dracula-comment p-4"
    >
      <h3 class="font-semibold text-dracula-purple">{{ fenêtre.titre }}</h3>
      <dl class="flex flex-col gap-3">
        <div v-for="ligne in fenêtre.lignes" :key="ligne.libellé" class="flex flex-wrap items-center gap-x-3 gap-y-1">
          <dt class="w-full text-base text-dracula-foreground/80">{{ ligne.libellé }}</dt>
          <dd class="font-semibold">{{ ligne.valeur }}</dd>
          <dd
            v-if="ligne.objectif !== undefined"
            class="rounded-full border px-3 text-base"
            :class="ligne.objectif.tenu ? 'border-dracula-green text-dracula-green' : 'border-dracula-red text-dracula-red'"
          >
            {{ étiquetteDe(ligne.objectif) }}
          </dd>
        </div>
      </dl>
      <CourbeDeLaLumiere
        v-if="fenêtre.courbe !== undefined"
        :fenêtre="fenêtre.courbe.fenêtre"
        :périodes="fenêtre.courbe.périodes"
        :seuil-lx="fenêtre.courbe.seuilLx"
        :seuil="fenêtre.courbe.seuil"
      />
    </article>
  </section>
</template>
