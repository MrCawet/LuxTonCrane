<script setup lang="ts">
/** Une valeur enregistrée, avec ce qu'elle mesure. */
export interface LigneEnregistrée {
  libellé: string
  valeur: string
}

defineProps<{ lignes: LigneEnregistrée[] }>()

const émettre = defineEmits<{ modifier: [] }>()
</script>

<template>
  <div class="flex flex-col gap-4">
    <ul class="flex flex-col gap-2">
      <li v-for="ligne in lignes" :key="ligne.libellé">{{ ligne.libellé }} : <strong>{{ ligne.valeur }}</strong></li>
    </ul>
    <button type="button" name="modifier" class="rounded-lg border border-dracula-purple p-3 text-dracula-purple" @click="émettre('modifier')">
      Modifier
    </button>
  </div>
</template>
