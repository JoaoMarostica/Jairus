<template>
  <n-modal
    v-model:show="createBatchModal"
    style="width: 1000px;"
    :mask-closable="false"
    preset="card"
    :closable="true"
    :title="modalTitle"
  >
    <n-grid :cols="3" x-gap="24px">
      <!-- Coluna do formulário -->
      <n-gi :span="2">
        <n-form
          ref="formRef"
          :model="form"
          :rules="rules"
          :size="size"
          label-placement="top"
        >
          <n-form-item
            :span="12"
            label="Número do Lote"
            path="batchNumber"
            :validation-status="batchNumberInputStatus"
            :feedback="batchNumberInputFeedback"
          >
            <n-input v-model:value="form.batchNumber" placeholder="Digite o número do lote" clearable />
          </n-form-item>
          <n-form-item
            :span="12"
            label="Data de Criação"
            path="timestamp"
          >
            <n-date-picker
              v-model:value="form.timestamp"
              type="date"
              format="dd/MM/yyyy"
              placeholder="Selecione a data de criação"
              @update:value="parseExpireDate"
              style="width: 100%"
              clearable
            />
          </n-form-item>
          <n-form-item :span="12" label="Cultivar" path="seed">
            <n-select
              v-model:value="form.seed"
              placeholder="Selecione a cultivar"
              :options="seedsOptions"
              clearable
            />
          </n-form-item>
          <n-form-item :span="12" label="Tratamento" path="coating">
            <n-select
              v-model:value="form.coating"
              placeholder="Selecione o tratamento"
              :options="coatingsOptions"
              clearable
            />
          </n-form-item>
          <n-form-item :span="12" label="PP/Kg" path="purenessScore">
            <n-input-number
              v-model:value="form.purenessScore"
              :parse="parseNumber"
              :format="formatNumber"
              placeholder="Digite o ponto de pureza por Kg"
              style="width: 100%"
              clearable 
            />
          </n-form-item>
          <n-form-item :span="12" label="Marca do Saco" path="sackBrand">
            <n-select
              v-model:value="form.sackBrand"
              placeholder="Selecione a marca do saco"
              :options="brandsOptions"
              clearable
            />
          </n-form-item>
          <n-form-item :span="12" label="Quantidade de Sacos" path="sackAmount">
            <n-input-number
              v-model:value="form.sackAmount"
              :step="1"
              :precision="0"
              placeholder="Digite a quantidade de sacos"
              style="width: 100%"
              clearable
            />
          </n-form-item>
          <n-form-item :span="12" label="Peso da Sacaria" path="sackWeight">
            <n-select
              v-model:value="form.sackWeight"
              placeholder="Selecione o peso da sacaria"
              :options="sackWeightsOptions"
              clearable
            />
          </n-form-item>
        </n-form>
      </n-gi>

      <!-- Coluna das descrições -->
      <n-gi :span="1">
        <n-descriptions
          label-placement="top"
          :column="1"
          title="Resumo do Lote"
          size="small"
          v-if="form.batchNumber || 
            year || 
            expireDate || 
            form.seed || 
            form.coating || 
            form.sackBrand || 
            form.sackWeight || 
            form.sackAmount || 
            totalWeight || 
            form.purenessScore || 
            totalPP"
        >
          <n-descriptions-item label="Número do Lote" v-if="form.batchNumber">
            {{ form.batchNumber }}
          </n-descriptions-item>
          <n-descriptions-item label="Ano" v-if="year">
            {{ year }}
          </n-descriptions-item>
          <n-descriptions-item label="Vencimento" v-if="expireDate">
            {{ expireDate }}
          </n-descriptions-item>
          <n-descriptions-item label="Cultivar" v-if="form.seed">
            {{ form.seed }}
          </n-descriptions-item>
          <n-descriptions-item label="Tratamento" v-if="form.coating">
            {{ form.coating }}
          </n-descriptions-item>
          <n-descriptions-item label="Marca do Saco" v-if="form.sackBrand">
            {{ form.sackBrand }}
          </n-descriptions-item>
          <n-descriptions-item label="Peso da Sacaria (Kg)" v-if="form.sackWeight">
            {{ form.sackWeight }}
          </n-descriptions-item>
          <n-descriptions-item label="Quantidade de Sacos" v-if="form.sackAmount">
            {{ form.sackAmount }}
          </n-descriptions-item>
          <n-descriptions-item label="Quantidade (kg)" v-if="totalWeight">
            {{ totalWeight }}
          </n-descriptions-item>
          <n-descriptions-item label="PP/Kg" v-if="form.purenessScore">
            {{ formatNumber(form.purenessScore) }}
          </n-descriptions-item>
          <n-descriptions-item label="Total PP" v-if="totalPP">
            {{ totalPP }}
          </n-descriptions-item>
        </n-descriptions>
      </n-gi>
    </n-grid>
    <template #footer>
      <div style="display: flex; justify-content: flex-end; margin-top: 16px">
        <n-button type="primary" @click="handleSubmit">
          Criar Lote
        </n-button>
      </div>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { computed, ref, reactive, watch } from 'vue'
import { NModal, NInput, FormInst, NSelect, NButton, NForm, NFormItem, NDatePicker, NDescriptions, NDescriptionsItem, NGi, NGrid, NInputNumber } from 'naive-ui'
import { BatchDB } from '@/types/batches'
import { useBatchesStore } from '@/stores/batchesStore'
import { useSettingsStore } from '@/stores/settingsStore'
import { useGlobalStore } from '@/stores/globalStore'
import { useSeedsStore } from '@/stores/seedsStore';
import { storeToRefs } from 'pinia'
import { FormValidationStatus } from 'naive-ui/es/form/src/interface'

const createBatchModal = defineModel('modal', {
  type: Boolean,
  default: false
})

const globalStore = useGlobalStore()
const batchesStore = useBatchesStore()
const seedsStore = useSeedsStore()

const settingsStore = useSettingsStore()
const { coatings, brands } = storeToRefs(settingsStore)
const { seeds } = storeToRefs(seedsStore)

const modalTitle = computed(() =>
  form?.batchNumber
    ? `Novo Lote ${form.batchNumber}/${String(year.value).slice(-2)}`
    : 'Novo Lote'
)

const year = ref<number | null>(null)
const expireDate = ref<string | null>(null)

const batchNumberInputStatus = ref<FormValidationStatus | undefined>(undefined)
const batchNumberInputFeedback = computed(() => {
  return batchNumberInputStatus.value === 'error'
    ? 'Lote já existe'
    : undefined
})

const formRef = ref<FormInst | null>(null)
const size = ref<'small' | 'medium' | 'large'>('medium')
const form = reactive({
  batchNumber: null as string | null,
  timestamp: null as number | null,
  seed: null,
  coating: null,
  sackBrand: null,
  sackAmount: null,
  sackWeight: null,
  purenessScore: null,
})

const seedsOptions = computed(() => {
  return seeds.value.map(seed => {return ({ label: seed.popular_name, value: seed.popular_name })})
})

const coatingsOptions = computed(() => {
  return coatings.value.map(coating => {return ({ label: coating.name, value: coating.name })})
})

const brandsOptions = computed(() => {
  return brands.value.map(brand => {return ({ label: brand.name, value: brand.name })})
})

const sackWeightsOptions = computed(() => {
  const brand = brands.value.find(brand => brand.name === form.sackBrand)
  return brand?.sackWeights ?? []
})

const totalWeight = computed(() => {
  const totalWeight = Number(form.sackAmount) * Number(form.sackWeight)

  return totalWeight === 0 ? null : totalWeight.toLocaleString("pt-BR")
})

const totalPP = computed(() => {
  const weight = parsePtBrNumber(totalWeight.value)
  const purenessScore = form.purenessScore ?? 0
  const total = weight * purenessScore
  return total === 0 ? null : (Math.round(total * 100) / 100).toLocaleString("pt-BR")
})

watch(createBatchModal, () => {
  if (createBatchModal.value) {
    form.batchNumber = getNextBatchNumber()
    parseExpireDate(Date.now())
  } else {
    resetForm()
  }
})

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
  batchNumber: {
    required: true,
    validator: (rule: any, value: string) => {
      const batchKey = `${value}${String(year.value)}`
      if (batchesStore.getBatchKeys.includes(batchKey)) {
        return Promise.reject(`Lote ${value}/${String(year.value).slice(-2)} já existe!`)
      }
      return Promise.resolve()
    },
    trigger: ['input', 'blur'],
  },
  timestamp: {
    required: true,
    type: 'number' as const,
    trigger: ['blur', 'change'],
    message: 'Campo obrigatório'
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
  sackBrand: {
    required: true,
    trigger: ['blur', 'change'],
    message: 'Campo obrigatório'
  },
  sackAmount: [
    {
      required: true,
      type: 'number' as const,
      trigger: ['blur', 'change'],
      message: 'Campo obrigatório'
    },
    {
      validator: positiveNumberValidator,
      trigger: ['blur', 'change']
    }
  ],
  sackWeight: [
    {
      required: true,
      trigger: ['blur', 'change'],
      message: 'Campo obrigatório'
    },
    {
      validator: positiveNumberValidator,
      trigger: ['blur', 'change']
    }
  ],
  purenessScore: [
    {
      required: true,
      type: 'number' as const,
      trigger: ['blur', 'change'],
      message: 'Campo obrigatório'
    },
    {
      validator: positiveNumberValidator,
      trigger: ['blur', 'change']
    }
  ],
}

function parseExpireDate(value: number | null) {
  if (value) {
    const date = new Date(value)
    year.value = date.getFullYear()
    expireDate.value = `${date.toLocaleString('pt-BR', { month: 'short' }).replace('.', '')}/${year.value + 1}`
    form.timestamp = value
  } else {
    form.timestamp = null
  }
}

function getNextBatchNumber() {
  const lastBatchNumber = batchesStore.getLastBatch()
  if (lastBatchNumber) {
    return (lastBatchNumber + 1).toString()
  }
  return '1'
}

function handleSubmit(e: MouseEvent) {
  e.preventDefault()

  const parsedWeight = parsePtBrNumber(totalWeight.value)

  if (parsedWeight > 10000) {
    globalStore.showMessage({
      content: 'O quantidade (Kg) não pode ultrapassar 10.000 kg.',
      type: 'error',
    })
    return
  }

  formRef.value?.validate(async (errors) => {
    if (!errors) {
      const batch: BatchDB = {
        batch_number: Number(form.batchNumber),
        batch_year: Number(year.value),
        batch_month: expireDate.value ? monthMap[expireDate.value.split('/')[0].toLowerCase()] : 0,
        seed: form.seed || '',
        coating: form.coating || '',
        brand: form.sackBrand || '',
        sack_weight: Number(form.sackWeight),
        sack_amount: Number(form.sackAmount),
        total_weight: parsedWeight,
        pureness_score: Number(form.purenessScore),
        total_pureness_score: parsePtBrNumber(totalPP.value),
        batch_status: 1,
        deleted_at: null,
        origin: null,
      }

      try {
        await batchesStore.createBatch(batch)

        globalStore.showMessage({
          content: 'Lote criado com sucesso!',
          type: 'success',
        })
        createBatchModal.value = false
        resetForm()
      } catch (error: any) {
        globalStore.showMessage({
          content: `Erro ao criar lote: ${error?.message || error}`,
          type: 'error',
        })
      }
    } else {
      globalStore.showMessage({
        content: 'Preencha todos os campos obrigatórios.',
        type: 'error',
      })
    }
  })
}

function resetForm() {
  form.batchNumber = null
  form.seed = null
  form.coating = null
  form.sackBrand = null
  form.sackAmount = null
  form.sackWeight = null
  form.purenessScore = null
}

function parsePtBrNumber(value: string | null): number {
  if (!value) return 0
  return Number(value.replace(/\./g, '').replace(',', '.'))
}

const parseNumber = (input: string): number | null => {
  const cleaned = input.trim()
    .replace(/\./g, '')
    .replace(',', '.')

  if (cleaned === '') return null

  const num = Number(cleaned)
  return isNaN(num) ? Number.NaN : num
}

const formatNumber = (value: number | null): string => {
  if (value === null)
    return ''
  return value.toLocaleString('pt-BR', {
    minimumFractionDigits: 1,
    maximumFractionDigits: 2
  })
}

function positiveNumberValidator(_: any, value: number | string | null) {
  if (Number(value) <= 0) {
    return new Error('Deve ser maior que zero')
  }
  return true
}

</script>
