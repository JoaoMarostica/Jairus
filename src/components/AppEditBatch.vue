<template>
  <n-modal
    v-model:show="editBatchModal"
    style="width: 1000px;"
    :mask-closable="false"
    preset="card"
    :closable="true"
    :title="modalTitle"
  >
    <n-grid :cols="3" x-gap="24px">
      <!-- Coluna do formulário -->
      <n-gi :span="1">
        <n-form
          ref="formRef"
          :model="newBatch"
          :rules="rules"
          :size="size"
          label-placement="top"
        >
        <n-form-item
          :span="12"
          label="Número do Lote"
          path="batch_number"
          :validation-status="batchNumberInputStatus"
          :feedback="batchNumberInputFeedback"
        >
          <n-input v-model:value="newBatch.batch_number" placeholder="Digite o número do lote" />
        </n-form-item>
        <n-form-item :span="12" label="Cultivar" path="seed">
          <n-select
            v-model:value="newBatch.seed"
            placeholder="Selecione a cultivar"
            :options="seedsOptions"
          />
        </n-form-item>
        <n-form-item :span="12" label="Tratamento" path="coating">
          <n-select
            v-model:value="newBatch.coating"
            placeholder="Selecione o tratamento"
            :options="coatingsOptions"
          />
        </n-form-item>
        <n-form-item :span="12" label="PP/Kg" path="purenessScore">
          <n-input v-model:value="newBatch.purenessScore" placeholder="Digite o ponto de pureza por Kg" />
        </n-form-item>
        <n-form-item :span="12" label="Marca do Saco" path="sackBrand">
          <n-select
            v-model:value="newBatch.sackBrand"
            placeholder="Selecione a marca do saco"
            :options="brandsOptions"
          />
        </n-form-item>
        <n-form-item :span="12" label="Quantidade de Sacos" path="sackAmount">
          <n-input v-model:value="newBatch.sackAmount" placeholder="Digite a quantidade de sacos" />
        </n-form-item>
        <n-form-item :span="12" label="Peso da Sacaria" path="sackWeight">
          <n-select
            v-model:value="newBatch.sackWeight"
            placeholder="Selecione o peso da sacaria"
            :options="sackWeightsOptions"
          />
        </n-form-item>
        

        <div style="display: flex; justify-content: flex-end; margin-top: 16px">
            <n-button round type="primary" @click="editBatch">
              Editar Lote
            </n-button>
          </div>
        </n-form>
      </n-gi>

      <!-- Divider vertical (coluna 2) -->
      <n-gi :span="1" style="display: flex; justify-content: center;">
        <n-divider vertical style="height: 100%; margin: 0;" />
      </n-gi>

      <!-- Coluna das descrições -->
      <n-gi :span="1">
        <n-descriptions
          bordered
          label-placement="top"
          :column="1"
          title="Resumo do Lote"
          size="small"
        >
          <n-descriptions-item label="Número do Lote">
            {{ newBatch.batch_number }}
          </n-descriptions-item>
          <n-descriptions-item label="Cultivar">
            {{ newBatch.seed }}
          </n-descriptions-item>
          <n-descriptions-item label="Tratamento">
            {{ newBatch.coating }}
          </n-descriptions-item>
          <n-descriptions-item label="Marca do Saco">
            {{ newBatch.sackBrand }}
          </n-descriptions-item>
          <n-descriptions-item label="Peso por Saco (Kg)">
            {{ newBatch.sackWeight }}
          </n-descriptions-item>
          <n-descriptions-item label="Quantidade de Sacos">
            {{ newBatch.sackAmount }}
          </n-descriptions-item>
          <n-descriptions-item label="Peso Total (Kg)">
            {{ totalWeight }}
          </n-descriptions-item>
          <n-descriptions-item label="PP/Kg">
            {{ newBatch.purenessScore }}
          </n-descriptions-item>
          <n-descriptions-item label="PP Total">
            {{ totalPP }}
          </n-descriptions-item>
        </n-descriptions>
      </n-gi>
    </n-grid>
  </n-modal>
</template>

<script setup lang="ts">
import { computed, ref, reactive, watchEffect, onMounted } from 'vue'
import { NModal, NInput, FormInst, NSelect, NButton, NForm, NFormItem, NDivider, NDescriptions, NDescriptionsItem, NGi, NGrid } from 'naive-ui'
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
const batchNumberInputFeedback = computed(() => {
  return batchNumberInputStatus.value === 'error'
    ? 'Lote já existe'
    : undefined
})

const batchTotalWeightInputStatus = ref<FormValidationStatus | undefined>(undefined)
const batchTotalWeightInputFeedback = computed(() => {
  return batchTotalWeightInputStatus.value === 'error'
    ? 'Acima do limite de 10.000 Kg'
    : undefined
})

const props = defineProps<{
  selectedBatch: any
}>()

const selectedBatch = computed(() => props.selectedBatch)

const formRef = ref<FormInst | null>(null)
const size = ref<'small' | 'medium' | 'large'>('medium')
const newBatch = reactive({
  batch_number: null,
  seed: null,
  coating: null,
  sackBrand: null,
  sackAmount: null,
  sackWeight: null,
  purenessScore: null,
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
  batchTotalWeightInputStatus.value = undefined

  if (newBatch.batch_number) {
    const batchKey = `${newBatch.batch_number}${String(year.value)}`

    if (batchKey !== `${selectedBatch.value.batch_number}${selectedBatch.value.batch_year.toString()}` &&
      batchesStore.getBatchKeys.includes(batchKey)) {
      batchNumberInputStatus.value = 'error'
    }
  }
  if (totalWeight.value !== null && parsePtBrNumber(totalWeight.value) > 10000) {
    batchTotalWeightInputStatus.value = 'error'
  }
})

onMounted(() => {
  if (selectedBatch.value) {
    prefillForm()
  }
})

function prefillForm() {
  newBatch.batch_number = selectedBatch.value.batch_number?.toString() || ''
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

const rules = {
  batch_number: {
    required: true,
    trigger: ['blur', 'input'],
    message: 'Campo obrigatório',
  },
  seed: {
    required: true,
    trigger: ['blur', 'change'],
    message: 'Campo obrigatório'
  },
  coating: {
    required: true,
    trigger: ['blur', 'change'],
    message: 'Campo obrigatório'
  },
  purenessScore: {
    required: true,
    trigger: ['blur', 'change'],
    message: 'Campo obrigatório'
  },
  sackBrand: {
    required: true,
    trigger: ['blur', 'change'],
    message: 'Campo obrigatório'
  },
  sackAmount: {
    required: true,
    trigger: ['blur', 'change'],
    message: 'Campo obrigatório'
  },
  sackWeight: {
    required: true,
    trigger: ['blur', 'change'],
    message: 'Campo obrigatório'
  },
}

function editBatch(e: MouseEvent) {
  e.preventDefault()
  formRef.value?.validate(async (errors) => {
    if (!errors) {
      const batch: BatchDB = {
        batch_number: Number(newBatch.batch_number),
        batch_year: Number(year.value),
        batch_month: monthMap[expireDate.value.split('/')[0].toLowerCase()],
        seed: newBatch.seed || '',
        coating: newBatch.coating || '',
        brand: newBatch.sackBrand || '',
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
    } else {
      globalStore.showMessage({
        content: 'Preencha todos os campos obrigatórios.',
        type: 'error',
      })
      return
    }
  })
}

</script>
