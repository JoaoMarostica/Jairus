<template>
  <n-modal
    v-model:show="editBatchModal"
    style="width: 600px;"
    :mask-closable="false"
    preset="card"
    :closable="true"
    :title="modalTitle"
  >
    <div style="width: 100%; height: 100%; overflow: auto;" class="bg-white dark:bg-black p-4">
      <n-space vertical>
        <n-input-group>
          <n-input v-model:value="newBatch.number" :style="{ width: '33%' }" :status="batchNumberInputStatus" placeholder="Número do Lote" />
          <n-input :style="{ width: '33%' }" :placeholder="year.toString()" disabled />
          <n-input :style="{ width: '33%' }" :placeholder="expireDate" disabled />
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
        <n-button type="primary" block @click="editBatch" :disabled="batchNumberInputStatus === 'error'">
          Editar Lote
        </n-button>
      </n-space>
    </div>
  </n-modal>
</template>

<script setup lang="ts">
import { computed, ref, reactive, watchEffect, onMounted } from 'vue'
import { NModal, NSpace, NInput, NInputGroup, NSelect, NButton } from 'naive-ui'
import { BatchDB } from '@/types/batches'
import { useBatchesStore } from '@/stores/batchesStore'
import { useSettingsStore } from '@/stores/settingsStore'
import { useGlobalStore } from '@/stores/globalStore'
import { storeToRefs } from 'pinia'
import { FormValidationStatus } from 'naive-ui/es/form/src/interface'

const editBatchModal = defineModel('modal', {
  type: Boolean,
  default: false
})

const globalStore = useGlobalStore()
const batchesStore = useBatchesStore()

const settingsStore = useSettingsStore()
const { seeds, coatings, brands } = storeToRefs(settingsStore)

const modalTitle = computed(() =>
  props.selectedBatch?.batch_number
    ? `Edição do Lote ${props.selectedBatch.batch_number}/${String(props.selectedBatch.batch_year).slice(-2)}`
    : 'Edição de Lote'
)

const year = ref(0)
const expireDate = ref('')
const batchNumberInputStatus = ref<FormValidationStatus | undefined>(undefined)

const props = defineProps<{
  selectedBatch: any
}>()

const selectedBatch = computed(() => props.selectedBatch)

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

  return totalWeight === 0 ? null : totalWeight.toLocaleString("pt-BR")
})

const totalPP = computed(() => {
  const totalPP = parsePtBrNumber(totalWeight.value) * parsePtBrNumber(newBatch.purenessScore)
  return totalPP === 0 ? null : (Math.round(totalPP * 100) / 100).toLocaleString("pt-BR")
})

watchEffect(() => {
  batchNumberInputStatus.value = undefined
  if (newBatch.number) {
    const batchKey = `${newBatch.number}${String(year.value)}`

    if (batchKey !== `${selectedBatch.value.batch_number}${selectedBatch.value.batch_year.toString()}` &&
      batchesStore.getBatchKeys.includes(batchKey)) {

      globalStore.showMessage({
        content: `Lote ${newBatch.number}/${String(year.value).slice(-2)} já existe!`,
        type: 'error',
      })
      batchNumberInputStatus.value = 'error'
    }
  }
})

onMounted(() => {
  if (selectedBatch.value) {
    prefillForm()
  }
})

function prefillForm() {
  newBatch.number = selectedBatch.value.batch_number?.toString() || ''
  newBatch.seed = selectedBatch.value.seed || null
  newBatch.coating = selectedBatch.value.coating || null
  newBatch.sackBrand = selectedBatch.value.brand || null
  newBatch.sackAmount = selectedBatch.value.sack_amount?.toString() || ''
  newBatch.sackWeight = selectedBatch.value.sack_weight?.toString() || null
  newBatch.purenessScore = selectedBatch.value.pureness_score || ''

  year.value = selectedBatch.value.batch_year
  expireDate.value = selectedBatch.value.expire_date
}

function parsePtBrNumber(value: string | null): number {
  if (!value) return 0
  return Number(value.replace(/\./g, '').replace(',', '.'))
}

async function editBatch() {
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
    batch_month: monthMap[expireDate.value.split('/')[0].toLowerCase()],
    seed: newBatch.seed,
    coating: newBatch.coating,
    brand: newBatch.sackBrand,
    sack_weight: Number(newBatch.sackWeight),
    sack_amount: Number(newBatch.sackAmount),
    total_weight: parsePtBrNumber(totalWeight.value),
    pureness_score: parsePtBrNumber(newBatch.purenessScore),
    total_pureness_score: parsePtBrNumber(totalPP.value),
    batch_status: 1,
    deleted_at: null,
    origin: null
  }

  try {
    await batchesStore.editBatch(batch)
    
    globalStore.showMessage({
      content: 'Lote editado com successo!',
      type: 'success',
    })
    editBatchModal.value = false
  } catch (error: any) {
    globalStore.showMessage({
      content: `Erro ao editar lote: ${error?.message || error}`,
      type: 'error',
      keepAliveOnHover: true,
    })
  }
}

const monthMap: Record<string, number> = {
  jan: 0,
  fev: 1,
  mar: 2,
  abr: 3,
  mai: 4,
  jun: 5,
  jul: 6,
  ago: 7,
  set: 8,
  out: 9,
  nov: 10,
  dez: 11
}

</script>
