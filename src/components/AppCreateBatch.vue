<template>
  <n-modal
    v-model:show="createBatchModal"
    style="width: 600px;"
    :mask-closable="false"
    preset="card"
    :closable="true"
    :title="modalTitle"
    v-on:update-show="createBatchModal = false"
  >
    <div style="width: 100%; height: 100%; overflow: auto;" class="bg-white dark:bg-black p-4">
      <n-space vertical>
        <n-input-group>
            <n-input v-model:value="newBatch.number" :style="{ width: '33%' }" placeholder="Número do Lote" />
            <n-input :style="{ width: '33%' }" :placeholder="year.toString()" disabled />
            <n-input :style="{ width: '33%' }" :placeholder="parsedExpireDate" disabled />
        </n-input-group>

        <n-input-group>
            <n-input v-model:value="newBatch.seed" :style="{ width: '33%' }" placeholder="Cultivar" />
            <n-input v-model:value="newBatch.coating" :style="{ width: '33%' }" placeholder="Tratamento" />
        </n-input-group>

        <n-input-group>
            <n-input-number v-model:value="newBatch.purenessScore" :style="{ width: '33%' }" placeholder="PP/Kg" />
            <n-input-number v-model:value="totalPP" :style="{ width: '33%' }" placeholder="Total PP" />
            <n-input-number v-model:value="availableQuantity" :style="{ width: '33%' }" placeholder="Quantidade (Kg)" />
        </n-input-group>

        <n-input-group>
            <n-input v-model:value="newBatch.sackBrand" :style="{ width: '33%' }" placeholder="Marca do Saco" style="min-width: 45%"/>
            <n-input-number v-model:value="newBatch.sackQuantity" :style="{ width: '50%' }" placeholder="Qtd. de Sacos" />
            <n-select v-model:value="newBatch.sackWeight" :options="sackWeights" :style="{ width: '50%' }" placeholder="Peso por Saco (kg)" />
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
import { computed, ref, watch, reactive } from 'vue'
import { NModal, NSpace, NInput, NInputGroup, NInputNumber, NSelect, NButton } from 'naive-ui'

const createBatchModal = defineModel('modal', {
  type: Boolean,
  default: false
})

const modalTitle = ref('Lote')

const year = ref(new Date().getFullYear())
const expireDate = ref(new Date().getMonth())
const sackWeights = ref([
    {
        label: '10',
        value: 10,
    },
    {
        label: '15',
        value: 15,
    },
    {
        label: '25',
        value: 25,
    },
    {
        label: '30',
        value: 30,
    },
])

// Estado do formulário
const newBatch = reactive({
    number: null,
    seed: '',
    coating: '',
    sackBrand: '',
    sackQuantity: null,
    sackWeight: null,
    purenessScore: null,
    status: '',
    deletedAt: null
})

const availableQuantity = computed(() => {
    const quantity = newBatch.sackQuantity ?? 0
    const weight = newBatch.sackWeight ?? 0
    const availableQuantity = quantity * weight

    return availableQuantity === 0 ? null : availableQuantity
})

const totalPP = computed(() => {
    const toFloat2 = (value: any) => Math.round(parseFloat(value) * 100) / 100;
    const sackQuantity = newBatch.sackQuantity ?? 0;
    const sackWeight = newBatch.sackWeight ?? 0;
    const purenessScore = newBatch.purenessScore ?? 0;
    const totalPP = toFloat2(sackQuantity * sackWeight * purenessScore)

    return totalPP === 0 ? null : totalPP
})

const parsedExpireDate = computed(() => {
  const monthNames = ['jan', 'fev', 'mar', 'abr', 'mai', 'jun', 'jul', 'ago', 'set', 'out', 'nov', 'dez'];
  const month = monthNames[expireDate.value] || '--';
  return `${month}/${year.value + 1}`;
})

watch(newBatch, () => {
    if (newBatch.number !== null) {
        modalTitle.value = `Lote ${newBatch.number}`
    }
})

// Função para submeter
function createBatch() {
  console.log('Dados do lote:', newBatch)
  // Aqui você pode emitir para o backend, resetar o newBatch, fechar o modal, etc.
}

</script>
