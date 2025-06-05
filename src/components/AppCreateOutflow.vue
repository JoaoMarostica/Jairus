<template>
  <n-modal
    v-model:show="createOutflowModal"
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
            label="Número do Pedido"
            path="usage"
          >
            <n-input v-model:value="form.usage" placeholder="Digite o número do pedido" clearable />
          </n-form-item>
          <n-form-item :span="12" label="Total PP" path="totalPurenessScore">
            <n-input-number
              v-model:value="form.totalPurenessScore"
              :parse="parseNumber"
              :format="formatNumber"
              placeholder="Digite o total de PP"
              style="width: 100%"
              clearable
              :disabled="inputMode === 'totalWeight'"
            />
          </n-form-item>
          <n-form-item :span="12" label="Quantidade (Kg)" path="totalWeight">
            <n-input-number
              v-model:value="form.totalWeight"
              :parse="parseNumber"
              :format="formatNumber"
              placeholder="Digite a quantidade"
              style="width: 100%"
              clearable
              :disabled="inputMode === 'totalPP'"
            >
              <template #suffix>
                Kg
              </template>
            </n-input-number>
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
          v-if="form.usage || 
            sackAmount ||
            totalWeight || 
            form.totalWeight ||
            totalPP ||
            form.totalPurenessScore"
        >
          <n-descriptions-item label="Número do Lote" v-if="form.usage">
            {{ form.usage }}
          </n-descriptions-item>
          <n-descriptions-item label="Quantidade de Sacos" v-if="sackAmount">
            {{ sackAmount }}
          </n-descriptions-item>
          <n-descriptions-item label="Quantidade (kg)" v-if="form.totalWeight || totalWeight">
            {{ totalWeight || form.totalWeight }}
          </n-descriptions-item>
          <n-descriptions-item label="Total PP" v-if="form.totalPurenessScore || totalPP">
            {{ totalPP || form.totalPurenessScore }}
          </n-descriptions-item>
        </n-descriptions>
      </n-gi>
    </n-grid>
    <template #footer>
      <div style="display: flex; justify-content: flex-end; margin-top: 16px">
        <n-button type="primary" @click="createOutflow" :disabled="batchTotalWeightInputStatus === 'error'">
          Adicionar Saída
        </n-button>
      </div>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { computed, ref, reactive, watch } from 'vue'
import type {
  FormInst,
  FormItemRule,
  FormRules
} from 'naive-ui'
import { NModal, NInput, NButton, NForm, NFormItem, NDescriptions, NDescriptionsItem, NGi, NGrid, NInputNumber } from 'naive-ui'
import { BatchOutflowDB } from '@/types/batches'
import { useBatchesStore } from '@/stores/batchesStore'
import { useGlobalStore } from '@/stores/globalStore'
import { FormValidationStatus } from 'naive-ui/es/form/src/interface'

const createOutflowModal = defineModel('modal', {
  type: Boolean,
  default: false
})

const globalStore = useGlobalStore()
const batchesStore = useBatchesStore()

const modalTitle = computed(() =>
  selectedBatch.value
    ? `Nova Saída para o lote ${selectedBatch.value.batch_number}/${String(selectedBatch.value.batch_year).slice(-2)}`
    : 'Nova Saída'
)

const batchTotalWeightInputStatus = ref<FormValidationStatus | undefined>(undefined)
// const batchTotalWeightInputFeedback = computed(() => {
//   return batchTotalWeightInputStatus.value === 'error'
//     ? 'Acima do limite de 10.000 Kg'
//     : undefined
// })

const props = defineProps<{
  selectedBatch: any
}>()

const selectedBatch = computed(() => props.selectedBatch)
const purenessScore = computed(() => selectedBatch.value.pureness_score || 0)

const inputMode = ref<'totalPP' | 'totalWeight' | null>(null)

const formRef = ref<FormInst | null>(null)
const size = ref<'small' | 'medium' | 'large'>('medium')
const form = reactive({
  usage: null as string | null,
  totalWeight: null as number | null,
  totalPurenessScore: null as number | null,
})

watch(createOutflowModal, () => {
  if (!createOutflowModal.value) {
    resetForm()
  }
})

watch(() => form.totalWeight, () => {
  if (form.totalWeight !== null) {
    inputMode.value = 'totalWeight'
  }
})

watch(() => form.totalPurenessScore, () => {
  if (form.totalPurenessScore !== null) {
    inputMode.value = 'totalPP'
  }
})

const sackAmount = computed(() => {
  const sackWeight = selectedBatch.value.sack_weight
  const sackAmount = (parsePtBrNumber(totalWeight.value) || form.totalWeight || 0) / sackWeight

  return sackAmount === 0 ? null : sackAmount.toLocaleString("pt-BR")
})

const totalWeight = computed(() => {
  if (inputMode.value !== 'totalPP') return null

  const totalPP = form.totalPurenessScore
  const pp = parsePtBrNumber(purenessScore.value)

  if (!totalPP || totalPP === 0 || !pp || pp === 0) return null

  const totalWeight = totalPP / pp
  form.totalWeight = totalWeight

  return totalWeight.toLocaleString('pt-BR')
})

const totalPP = computed(() => {
  if (inputMode.value !== 'totalWeight') return null

  const totalWeight = form.totalWeight
  const pp = parsePtBrNumber(purenessScore.value)

  if (!totalWeight || totalWeight === 0 || !pp || pp === 0) return null

  const totalPP = totalWeight * pp
  form.totalPurenessScore = totalPP

  return totalPP <= 0 ? null : (Math.round(totalPP * 100) / 100).toLocaleString("pt-BR")
})

const rules: FormRules = {
  usage: {
    required: true,
    trigger: ['blur', 'input'],
    message: 'Campo obrigatório',
  },
  totalPurenessScore: [
    {
      required: true,
      type: 'number' as const,
      trigger: ['blur', 'change'],
      message: 'Campo obrigatório'
    }, 
    // {
    //   validator: validatePositiveNumber,
    //   trigger: ['blur', 'change'],
    //   message: 'O total de PP não pode ser negativo.'
    // }, 
    // {
    //   validator: validateTotalPPMaximumValue,
    //   trigger: ['blur', 'change'],
    //   message: 'Saldo indisponível no lote.'
    // }
  ],
  totalWeight: [
    {
      required: true,
      type: 'number' as const,
      trigger: ['blur', 'change'],
      message: 'Campo obrigatório'
    }, 
    // {
    //   validator: validatePositiveNumber,
    //   trigger: ['blur', 'change'],
    //   message: 'A Quantidade (Kg) não pode ser negativa.'
    // }, 
    // {
    //   validator: validateTotalWeightMaximumValue,
    //   trigger: ['blur', 'change'],
    //   message: 'Saldo indisponível no lote.'
    // }
  ],
}

function createOutflow(e: MouseEvent) {
  e.preventDefault()
  formRef.value?.validate(async (errors) => {
    if (!errors) {
      const outflow: BatchOutflowDB = {
        batch_number: Number(selectedBatch.value.batch_number),
        batch_year: Number(selectedBatch.value.batch_year),
        sack_amount: sackAmount.value ? parsePtBrNumber(sackAmount.value) : 0,
        total_weight: parsePtBrNumber(totalWeight.value) || form.totalWeight || 0,
        total_pureness_score: parsePtBrNumber(totalPP.value) || form.totalPurenessScore || 0,
        usage: form.usage || '',
      }

      try {
        await batchesStore.createOutflow(outflow)
        
        globalStore.showMessage({
          content: 'Saída criada com successo!',
          type: 'success',
        })
        createOutflowModal.value = false
        resetForm()
      } catch (error: any) {
        globalStore.showMessage({
          content: `Erro ao criar saída: ${error?.message || error}`,
          type: 'error',
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

function resetForm() {
  form.usage = null
  form.totalWeight = null
  form.totalPurenessScore = null
  inputMode.value = null
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

function validatePositiveNumber(rule: FormItemRule, value: number | null): boolean {
  if (value === null || value >= 0) return true
  return false
}

// function validateTotalPPMaximumValue(rule: FormItemRule, value: number | null): boolean {
//   const batchOutflows = batchesStore.getBatchOutflows(selectedBatch.value.key)
//   const batchBalance = batchesStore.getBatchBalance(selectedBatch.value, batchOutflows)
//   if (value === null || value <= batchBalance[0].value) return true
//   return false
// }

// function validateTotalWeightMaximumValue(rule: FormItemRule, value: number | null): boolean {
//   const batchOutflows = batchesStore.getBatchOutflows(selectedBatch.value.key)
//   const batchBalance = batchesStore.getBatchBalance(selectedBatch.value, batchOutflows)
//   if (value === null || value <= batchBalance[1].value) return true
//   return false
// }

</script>
