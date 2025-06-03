<template>
  <n-modal
    v-model:show="createBatchModal"
    style="width: 600px;"
    :mask-closable="false"
    preset="card"
    :closable="true"
    :title="modalTitle"
  >
    <div style="width: 100%; height: 100%; overflow: auto;" class="bg-white dark:bg-black p-4">
      <n-space vertical>
        <n-input-group>
          <n-input v-model:value="newBatch.number" :style="{ width: '33%' }" placeholder="Número do Lote" />
          <n-input :style="{ width: '33%' }" :placeholder="year.toString()" disabled />
          <n-input :style="{ width: '33%' }" :placeholder="parsedExpireDate" disabled />
        </n-input-group>

        <n-input-group>
          <n-select v-model:value="newBatch.seed" :options="seedsOptions" :style="{ width: '50%' }" placeholder="Cultivar" />
          <n-select v-model:value="newBatch.coating" :options="coatingsOptions" :style="{ width: '50%' }" placeholder="Tratamento" />
        </n-input-group>

        <n-input-group>
          <n-input v-model:value="newBatch.purenessScore" :style="{ width: '33%' }" placeholder="PP/Kg" />
          <n-input v-model:value="totalWeight" :style="{ width: '33%' }" placeholder="Quantidade (Kg)" disabled />
          <n-input v-model:value="totalPP" :style="{ width: '33%' }" placeholder="Total PP" disabled />
        </n-input-group>

        <n-input-group>
          <n-select v-model:value="newBatch.sackBrand" :options="brandsOptions" :style="{ width: '33%' }" placeholder="Marca do Saco" />
          <n-input v-model:value="newBatch.sackAmount" :style="{ width: '33%' }" placeholder="Qtd. de Sacos" />
          <n-select v-model:value="newBatch.sackWeight" :options="sackWeightsOptions" :style="{ width: '33%' }" placeholder="Peso por Saco (kg)" />
        </n-input-group>

        <!-- Botão de criar -->
        <n-button type="primary" block @click="createBatch">
          Criar Lote
        </n-button>
      </n-space>
    </div>
  </n-modal>
</template>

<script setup lang="ts">
import { computed, ref, reactive, watchEffect, watch } from 'vue'
import { NModal, NSpace, NInput, NInputGroup, NSelect, NButton } from 'naive-ui'
import { BatchDB } from '@/types/batches'
import { useBatchesStore } from '@/stores/batchesStore'
import { useSettingsStore } from '@/stores/settingsStore'
import { useGlobalStore } from '@/stores/globalStore'
import { storeToRefs } from 'pinia'

const createBatchModal = defineModel('modal', {
  type: Boolean,
  default: false
})

const globalStore = useGlobalStore()
const batchesStore = useBatchesStore()

const settingsStore = useSettingsStore()
const { seeds, coatings, brands } = storeToRefs(settingsStore)

const modalTitle = ref('Lote')

const year = ref(new Date().getFullYear())
const expireDate = ref(new Date().getMonth())

// Estado do formulário
const newBatch = reactive({
  number: '',
  seed: null,
  coating: null,
  sackBrand: null,
  sackAmount: '',
  sackWeight: null,
  purenessScore: '',
})

const seedsOptions = computed(() => {
  return seeds.value.map(seed => {return ({ label: seed.popularName, value: seed.popularName })})
})

const coatingsOptions = computed(() => {
  return coatings.value.map(coating => {return ({ label: coating.name, value: coating.name })})
})

const brandsOptions = computed(() => {
  return brands.value.map(brand => {return ({ label: brand.name, value: brand.name })})
})

const sackWeightsOptions = computed(() => {
  const brand = brands.value.find(brand => brand.name === newBatch.sackBrand)
  return brand?.sackWeights ?? []
})

const totalWeight = computed(() => {
  const amount = Number(newBatch.sackAmount)
  const weight = Number(newBatch.sackWeight)
  const totalWeight = amount * weight

  return totalWeight === 0 ? null : totalWeight.toString()
})

const totalPP = computed(() => {
  const totalPP = Number(totalWeight.value) * Number(newBatch.purenessScore)
  return totalPP === 0 ? null : (Math.round(totalPP * 100) / 100).toString()
})

const parsedExpireDate = computed(() => {
  const monthNames = ['jan', 'fev', 'mar', 'abr', 'mai', 'jun', 'jul', 'ago', 'set', 'out', 'nov', 'dez'];
  const month = monthNames[expireDate.value] || '--';
  return `${month}/${year.value + 1}`;
})

watch(createBatchModal, () => {
  if (createBatchModal.value) {
    newBatch.number = getNextBatchNumber()
  } else {
    resetForm()
  }
})

watchEffect(() => {
  if (newBatch.number) {
    modalTitle.value = `Lote ${newBatch.number}/${String(year.value).slice(-2)}`
  } else {
    modalTitle.value = 'Lote'
  }
})

function getNextBatchNumber() {
  const lastBatchNumber = batchesStore.getLastBatch()
  if (lastBatchNumber) {
    return (lastBatchNumber + 1).toString()
  }
  return '1'
}

// Função para submeter
async function createBatch() {
  if (
    !newBatch.number ||
    !newBatch.seed ||
    !newBatch.coating ||
    !newBatch.sackBrand ||
    !newBatch.sackAmount ||
    !newBatch.sackWeight ||
    !newBatch.purenessScore
  ) {
    globalStore.showMessage({
      content: 'Preencha todos os campos obrigatórios.',
      type: 'error',
    })
    return
  }
  
  const batch: BatchDB = {
    batch_number: Number(newBatch.number),
    batch_year: Number(year.value),
    batch_month: Number(expireDate.value),
    seed: newBatch.seed,
    coating: newBatch.coating,
    brand: newBatch.sackBrand,
    sack_weight: Number(newBatch.sackWeight),
    sack_amount: Number(newBatch.sackAmount),
    total_weight: Number(totalWeight.value),
    pureness_score: Number(newBatch.purenessScore),
    total_pureness_score: Number(totalPP.value),
    batch_status: 1,
    deleted_at: null,
    origin: null
  }

  try {
    await batchesStore.createBatch(batch)

    globalStore.showMessage({
      content: 'Lote criado com successo!',
      type: 'success',
    })
    resetForm()
    createBatchModal.value = false
  } catch (error: any) {
    globalStore.showMessage({
      content: `Erro ao criar lote: ${error?.message || error}`,
      type: 'error',
    })
  }
}

function resetForm() {
  newBatch.number = ''
  newBatch.seed = null
  newBatch.coating = null
  newBatch.sackBrand = null
  newBatch.sackAmount = ''
  newBatch.sackWeight = null
  newBatch.purenessScore = ''
}

</script>
