<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import CourbeDeLaLumiere from './CourbeDeLaLumiere.vue'
import RoueDeLHeure from './RoueDeLHeure.vue'
import SaisieEnregistree from './SaisieEnregistree.vue'
import TendanceDesNuits, { type JourDeLaTendance, type Repère } from './TendanceDesNuits.vue'
import type { FenêtreDuRelevé, PériodeMesurée } from './courbeDeLaLumiere'
import { CONTINUITÉ, ENDORMISSEMENT, FATIGUE, titreDuNiveau, type Niveau } from './echellesDuSommeil'
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

/** Le relevé d'une des dernières nuits, avec le jour de son lever. */
interface RelevéDaté extends RelevéDuJour {
  jour: string
}

/** `échelle` est la valeur d'une barre pleine ; sans `unité`, la valeur est le niveau d'une note. */
interface Tendance {
  libellé: string
  jours: JourDeLaTendance[]
  échelle: number
  unité?: string
  repère?: Repère
}

interface Fenêtre {
  titre: string
  lignes: Ligne[]
  /** Seulement pour les fenêtres de lumière. */
  courbe?: Courbe
  tendances: Tendance[]
}

interface NuitJson {
  coucher_ms: number
  lever_ms: number
}

const coucherSouhaité = ref('23:00')
const relevé = ref<RelevéDuJour | null>(null)
const relevésDesDernièresNuits = ref<RelevéDaté[]>([])
const erreur = ref<string | null>(null)
const messageDuRéglage = ref<string | null>(null)
/** Le coucher souhaité a toujours une valeur : la page l'affiche, jusqu'à ce qu'on veuille le modifier. */
const coucherSouhaitéEnModification = ref(false)

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
    ? 'Coucher souhaité enregistré.'
    : `Le coucher souhaité n'a pas pu être enregistré : le serveur a répondu ${réponse.status}.`
  coucherSouhaitéEnModification.value = !réponse.ok
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
  const réponseDesDernièresNuits = await fetch(`/api/releves?${paramètres}`, { headers: entêtes() })
  relevésDesDernièresNuits.value = réponseDesDernièresNuits.ok ? await réponseDesDernièresNuits.json() : []
}

function nombreDeMinutes(duréeMs: number): string {
  return `${Math.floor(duréeMs / 60_000)}`
}

function minutes(duréeMs: number): string {
  return `${nombreDeMinutes(duréeMs)} min`
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

const UNE_HEURE_MS = 60 * 60_000

/** La tendance d'une valeur du relevé : `jourDe` la tire du relevé de chaque nuit. */
function tendanceDe(
  libellé: string,
  échelle: number,
  unité: string | undefined,
  jourDe: (relevé: RelevéDaté) => Omit<JourDeLaTendance, 'jour'>,
): Tendance {
  const jours = relevésDesDernièresNuits.value.map((relevéDeLaNuit) => ({
    jour: relevéDeLaNuit.jour,
    ...jourDe(relevéDeLaNuit),
  }))
  return { libellé, jours, échelle, unité }
}

/** La tendance d'une note : plus la note est bonne, plus la barre est haute. Seule la fatigue peut manquer. */
function tendanceDeLaNote(
  libellé: string,
  échelle: Niveau[],
  noteDe: (relevé: RelevéDaté) => Note | null,
): Tendance {
  return tendanceDe(libellé, échelle.length, undefined, (relevéDeLaNuit) => {
    const note = noteDe(relevéDeLaNuit)
    if (note === null) {
      return { valeur: 0, texte: 'pas encore notée', nombre: '', objectifTenu: null }
    }
    const rang = échelle.findIndex((niveau) => niveau.nom === note.niveau)
    return {
      valeur: échelle.length - rang,
      texte: titreDuNiveau(échelle, note.niveau),
      nombre: '',
      objectifTenu: note.objectif_tenu,
    }
  })
}

const OBJECTIF_DE_QUANTITÉ_DE_SOMMEIL_MS = 7 * UNE_HEURE_MS

function heuresAvecUneDécimale(duréeMs: number): string {
  const heures = duréeMs / UNE_HEURE_MS
  return heures.toLocaleString('fr-FR', { maximumFractionDigits: 1 })
}

function tendanceDeLaQuantitéDeSommeil(): Tendance {
  const tendance = tendanceDe('quantité de sommeil', 9 * UNE_HEURE_MS, 'heures', (relevéDeLaNuit) => {
    const quantité = relevéDeLaNuit.quantite_de_sommeil
    return {
      valeur: quantité.duree_ms,
      texte: heuresEtMinutes(quantité.duree_ms),
      nombre: heuresAvecUneDécimale(quantité.duree_ms),
      objectifTenu: quantité.objectif_tenu,
    }
  })
  return { ...tendance, repère: { valeur: OBJECTIF_DE_QUANTITÉ_DE_SOMMEIL_MS, texte: '7 h' } }
}

function tendanceDuMatin(): Tendance {
  return tendanceDe('à 10 000 lx ou plus', UNE_HEURE_MS, 'min', (relevéDeLaNuit) => {
    const lumière = relevéDeLaNuit.lumiere_du_matin
    return {
      valeur: lumière.temps_a_10_000_lx_ou_plus_ms,
      texte: minutes(lumière.temps_a_10_000_lx_ou_plus_ms),
      nombre: nombreDeMinutes(lumière.temps_a_10_000_lx_ou_plus_ms),
      objectifTenu: lumière.objectif_tenu,
    }
  })
}

function tendanceDeLaJournée(): Tendance {
  return tendanceDe('temps mesuré à 250 lx ou plus', 1, '%', (relevéDeLaNuit) => {
    const lumière = relevéDeLaNuit.lumiere_de_la_journee
    const part = lumière.part_du_temps_mesure_a_250_lx_ou_plus
    return {
      valeur: part ?? 0,
      texte: part === null ? 'rien de mesuré' : pourcentage(part),
      nombre: part === null ? '–' : `${Math.round(part * 100)}`,
      objectifTenu: lumière.objectif_tenu,
    }
  })
}

function tendanceDeLaLumièreÀÉviter(
  seuil: string,
  lumièreDe: (relevé: RelevéDaté) => LumièreÀÉviter,
): Tendance {
  return tendanceDe(`au-dessus de ${seuil}`, UNE_HEURE_MS, 'min', (relevéDeLaNuit) => {
    const lumière = lumièreDe(relevéDeLaNuit)
    return {
      valeur: lumière.temps_au_dessus_du_seuil_ms,
      texte: minutes(lumière.temps_au_dessus_du_seuil_ms),
      nombre: nombreDeMinutes(lumière.temps_au_dessus_du_seuil_ms),
      objectifTenu: lumière.objectif_tenu,
    }
  })
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
      tendances: [
        tendanceDeLaNote('endormissement', ENDORMISSEMENT, (relevéDeLaNuit) => relevéDeLaNuit.endormissement),
        tendanceDeLaNote('continuité de la nuit', CONTINUITÉ, (relevéDeLaNuit) => relevéDeLaNuit.continuite),
        tendanceDeLaQuantitéDeSommeil(),
      ],
    },
    {
      titre: 'Soir, 3 h avant le coucher',
      lignes: lignesDeLaLumièreÀÉviter(relevé.value.lumiere_du_soir, '10 lx'),
      courbe: courbeDe(relevé.value.lumiere_du_soir, 10, '10 lx'),
      tendances: [tendanceDeLaLumièreÀÉviter('10 lx', (relevéDeLaNuit) => relevéDeLaNuit.lumiere_du_soir)],
    },
    {
      titre: 'Nuit, du coucher au lever',
      lignes: lignesDeLaLumièreÀÉviter(relevé.value.lumiere_de_la_nuit, '1 lx'),
      courbe: courbeDe(relevé.value.lumiere_de_la_nuit, 1, '1 lx'),
      tendances: [tendanceDeLaLumièreÀÉviter('1 lx', (relevéDeLaNuit) => relevéDeLaNuit.lumiere_de_la_nuit)],
    },
    {
      titre: 'Fatigue, le soir suivant',
      lignes: [
        fatigue === null
          ? { libellé: 'Fatigue de la journée', valeur: 'pas encore notée' }
          : ligneDeLaNote('Fatigue de la journée', FATIGUE, fatigue),
      ],
      tendances: [
        tendanceDeLaNote('fatigue de la journée', FATIGUE, (relevéDeLaNuit) => relevéDeLaNuit.fatigue),
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
      tendances: [tendanceDuMatin()],
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
      tendances: [tendanceDeLaJournée()],
    },
  ]
})
</script>

<template>
  <section class="flex flex-col gap-5 rounded-2xl bg-dracula-current-line p-5">
    <h2 class="text-2xl font-semibold">Relevé du jour</h2>
    <SaisieEnregistree
      v-if="!coucherSouhaitéEnModification"
      :lignes="[{ libellé: 'Coucher souhaité', valeur: coucherSouhaité }]"
      @modifier="coucherSouhaitéEnModification = true"
    />
    <div v-else class="flex items-end gap-3">
      <RoueDeLHeure v-model="coucherSouhaité" nom="Coucher souhaité" :pas-en-minutes="15" />
      <button type="button" name="regler" class="rounded-lg border border-dracula-purple px-4 py-3 text-dracula-purple" @click="réglerLeCoucherSouhaité">
        Enregistrer
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
      <template v-if="relevésDesDernièresNuits.length > 0">
        <TendanceDesNuits
          v-for="tendance in fenêtre.tendances"
          :key="tendance.libellé"
          :libellé="tendance.libellé"
          :jours="tendance.jours"
          :échelle="tendance.échelle"
          :unité="tendance.unité"
          :repère="tendance.repère"
        />
      </template>
    </article>
  </section>
</template>
