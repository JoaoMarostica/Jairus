<template>
  <n-modal
    v-model:show="createOutflowModal"
    style="width: 1000px;"
    :mask-closable="false"
    preset="card"
    draggable
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
          <!-- <n-form-item :span="12" label="Total PP" path="totalPurenessScore">
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
          </n-form-item> -->
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
            parsePtBrNumber(totalWeight) > 0 || 
            totalPP"
        >
          <n-descriptions-item label="Número do Lote" v-if="form.usage">
            {{ form.usage }}
          </n-descriptions-item>
          <n-descriptions-item label="Quantidade de Sacos" v-if="form.sackAmount">
            {{ form.sackAmount }}
          </n-descriptions-item>
          <n-descriptions-item label="Quantidade (kg)" v-if="parsePtBrNumber(totalWeight) > 0">
            {{ totalWeight }}
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
          Adicionar Saída
        </n-button>
      </div>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { computed, ref, reactive, watch } from 'vue'
import type { FormInst, FormRules } from 'naive-ui'
import { NModal, NInput, NButton, NForm, NFormItem, NDescriptions, NDescriptionsItem, NGi, NGrid, NInputNumber } from 'naive-ui'
import { BatchOutflowDB } from '@/types/batches'
import { useBatchesStore } from '@/stores/batchesStore'
import { useGlobalStore } from '@/stores/globalStore'

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

const props = defineProps<{
  selectedBatch: any
}>()

const selectedBatch = computed(() => props.selectedBatch)
const purenessScore = computed(() => selectedBatch.value.pureness_score || 0)

// const inputMode = ref<'totalPP' | 'totalWeight' | null>(null)

const formRef = ref<FormInst | null>(null)
const size = ref<'small' | 'medium' | 'large'>('medium')
const form = reactive({
  usage: null as string | null,
  sackAmount: null as number | null,
  // totalWeight: null as number | null,
  // totalPurenessScore: null as number | null,
})

watch(createOutflowModal, () => {
  if (!createOutflowModal.value) {
    resetForm()
  }
})

// watch(() => form.totalWeight, () => {
//   if (form.totalWeight !== null) {
//     inputMode.value = 'totalWeight'
//   }
// })

// watch(() => form.totalPurenessScore, () => {
//   if (form.totalPurenessScore !== null) {
//     inputMode.value = 'totalPP'
//   }
// })

// const sackAmount = computed(() => {
//   const sackWeight = selectedBatch.value.sack_weight
//   const sackAmount = (parsePtBrNumber(totalWeight.value) || form.totalWeight || 0) / sackWeight

//   return sackAmount === 0 ? null : sackAmount.toLocaleString("pt-BR")
// })

const totalWeight = computed(() => {
  const totalWeight = (form.sackAmount ?? 0) * selectedBatch.value.sack_weight

  return totalWeight.toLocaleString('pt-BR')
})

const totalPP = computed(() => {
  const pp = parsePtBrNumber(purenessScore.value)

  const totalPP = parsePtBrNumber(totalWeight.value) * pp

  return totalPP <= 0 ? null : (Math.round(totalPP * 100) / 100).toLocaleString("pt-BR")
})

async function handleSubmit(e: MouseEvent) {
  e.preventDefault()

  try {
    const parsedWeight = parsePtBrNumber(totalWeight.value)
    const batchBalance = await batchesStore.getBatchBalance(selectedBatch.value.batch_number, selectedBatch.value.batch_year)

    if (parsedWeight > parsePtBrNumber(batchBalance.total_weight)) {
      globalStore.showMessage({
        content: 'O quantidade (Kg) não disponível no lote.',
        type: 'error',
      })
      return
    }
  } catch (error) {
    globalStore.showMessage({
      content: 'Erro ao calcular saldo.',
      type: 'error',
    })
    return
  }

  formRef.value?.validate(async (errors) => {
    if (!errors) {
      const outflow = {
        batch_number: Number(selectedBatch.value.batch_number),
        batch_year: Number(selectedBatch.value.batch_year),
        sack_amount: form.sackAmount || 0,
        total_weight: parsePtBrNumber(totalWeight.value) || 0,
        total_pureness_score: parsePtBrNumber(totalPP.value) || 0,
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
  form.sackAmount = null
  // form.totalWeight = null
  // form.totalPurenessScore = null
  // inputMode.value = null
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

const rules: FormRules = {
  usage: {
    required: true,
    trigger: ['blur', 'input'],
    message: 'Campo obrigatório',
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
      trigger: ['blur', 'change'],
      message: 'A quantidade de sacos não pode ser negativa.'
    }
  ],
  // totalPurenessScore: [
  //   {
  //     required: true,
  //     type: 'number' as const,
  //     trigger: ['blur', 'change'],
  //     message: 'Campo obrigatório'
  //   }, 
  //   {
  //     validator: validatePositiveNumber,
  //     trigger: ['blur', 'change'],
  //     message: 'O total de PP não pode ser negativo.'
  //   }, 
  //   {
  //     validator: validateTotalPPMaximumValue,
  //     trigger: ['blur', 'change'],
  //     message: 'Saldo indisponível no lote.'
  //   }
  // ],
  // totalWeight: [
  //   {
  //     required: true,
  //     type: 'number' as const,
  //     trigger: ['blur', 'change'],
  //     message: 'Campo obrigatório'
  //   }, 
  //   {
  //     validator: validatePositiveNumber,
  //     trigger: ['blur', 'change'],
  //     message: 'A Quantidade (Kg) não pode ser negativa.'
  //   }, 
  //   {
  //     validator: validateTotalWeightMaximumValue,
  //     trigger: ['blur', 'change'],
  //     message: 'Saldo indisponível no lote.'
  //   }
  // ],
}

</script>
