<template>
  <n-modal
    v-model:show="isModalOpen"
    class="w-screen h-screen p-4"
    :style="{ top: '0px', left: '0px', margin: '0', padding: '0' }"
    :mask-closable="false"
    preset="card"
    :closable="true"
    v-on:update-show="closeModal"
    :title="modalTitle"
    size="huge"
  >
    <div class="w-screen h-screen bg-white overflow-auto p-4">
      <n-grid cols="1" responsive="screen" x-gap="16" y-gap="16">
        <!-- Dados do lote -->
        <n-grid-item>
          <n-card class="border border-gray-200">
            <n-descriptions label-placement="top" :column="11" size="small">
              <n-descriptions-item
                v-for="(indicator, index) in batchData"
                :key="index"
                :label="indicator.titulo"
              >
                {{ indicator.valor }} {{ indicator.unidade }}
              </n-descriptions-item>
            </n-descriptions>
          </n-card>
        </n-grid-item>

        <!-- Gráficos -->
        <n-grid-item>
          <n-grid cols="1 m:2" responsive="screen" x-gap="16" y-gap="16" class="mt-6">
            <n-grid-item>
              <n-card title="Saídas" style="height: 300px;">
                <div ref="outflowChart" style="height: 250px;" />
              </n-card>
            </n-grid-item>
            <n-grid-item>
              <n-card title="Saldo" style="height: 300px;">
                <div ref="balanceChart" style="height: 250px;" />
              </n-card>
            </n-grid-item>
          </n-grid>
        </n-grid-item>

        <!-- Tabela de saídas -->
        <n-grid-item>
          <n-card title="Saídas do lote">
            <n-data-table :columns="outflowColumns" :data="outflowData" :pagination="false" />
          </n-card>
        </n-grid-item>
      </n-grid>
    </div>
  </n-modal>
</template>

<script setup lang="ts">
import { NModal, NCard, NGrid, NGridItem, NDataTable, NDescriptions, NDescriptionsItem } from 'naive-ui'
import { nextTick, ref, watch } from 'vue'
import * as echarts from 'echarts'
import { useBatchesStore } from '@/stores/batchesStore';

type BatchOutflow = {
  outflowPP: number;
  outflowKg: number;
  outflowSack: number;
  usage: string;
};

type BatchBalance = {value: number; name: string};

const batchesStore = useBatchesStore();

const props = defineProps<{
  selectedBatch: any
}>()

const isModalOpen = defineModel('model', {
  type: Boolean,
  default: false
})

const selectedBatch = ref(props.selectedBatch)
const modalTitle = ref(`Detalhes do Lote ${props.selectedBatch}`)

const batchData = ref<{ titulo: string; valor: any; unidade: string }[]>([])
const outflowChart = ref<HTMLElement | null>(null)
const balanceChart = ref<HTMLElement | null>(null)
const outflowData = ref<BatchOutflow[]>([])
const batchBalance = ref<BatchBalance[]>([])

watch(isModalOpen, async () => {
  if (isModalOpen) {
    await nextTick()
    selectedBatch.value = props.selectedBatch
    modalTitle.value = `Detalhes do Lote ${selectedBatch.value.number}`
    batchData.value = getbatchData()
    outflowData.value = await batchesStore.getBatchOutflow(selectedBatch.value.number)
    batchBalance.value = await batchesStore.getBatchBalance(selectedBatch.value, outflowData.value)
    renderCharts()
  }
})

function renderCharts() {
  if (outflowChart.value && selectedBatch.value) {
    const chart = echarts.init(outflowChart.value)
    chart.setOption({
      xAxis: { type: 'category', data: ['Jan', 'Fev', 'Mar', 'Abr'] },
      yAxis: { type: 'value' },
      series: [
        { name: 'Entrada', type: 'bar', data: [300, 400, 500, 350] },
        { name: 'Saída', type: 'bar', data: [200, 250, 300, 270] }
      ]
    })
  }

  if (balanceChart.value && batchBalance.value) {
    const chart = echarts.init(balanceChart.value)
    chart.setOption({
        tooltip: {
        trigger: 'item',
        formatter: '{b}: {c} ({d}%)'
      },
      series: [{
        name: 'Saldo',
        type: 'pie',
        radius: '50%',
        data: batchBalance.value,
        label: {
          show: true,
          formatter: '{b}: {c}',
          fontSize: 14
        }
      }]
    })
  }
}

function closeModal(model: boolean) {
  if (!model) {
    selectedBatch.value = null
    modalTitle.value = ''
    batchData.value = []

    if (outflowChart.value) {
      const chart = echarts.getInstanceByDom(outflowChart.value)
      if (chart) {
        chart.dispose()
      }
    }

    if (balanceChart.value) {
      const chart = echarts.getInstanceByDom(balanceChart.value)
      if (chart) {
        chart.dispose()
      }
    }
  }
}

function getbatchData() {
  return [
    { titulo: 'Ano', valor: selectedBatch.value.year, unidade: '' },
    { titulo: 'Data de validade', valor: selectedBatch.value.expireDate, unidade: '' },
    { titulo: 'Cultivar', valor: selectedBatch.value.seed, unidade: '' },
    { titulo: 'Revestimento', valor: selectedBatch.value.coating, unidade: '' },
    { titulo: 'Marca da Sacaria', valor: selectedBatch.value.sackBrand, unidade: '' },
    { titulo: 'Sacos', valor: selectedBatch.value.sackQuantity, unidade: '' },
    { titulo: 'Peso da Sacaria', valor: selectedBatch.value.sackWeight, unidade: 'Kg' },
    { titulo: 'Quantidade (Kg)', valor: selectedBatch.value.availableQuantity, unidade: 'kg' },
    { titulo: 'Ponto de Pureza (PP)', valor: selectedBatch.value.purenessScore, unidade: '' },
    { titulo: 'Total de PP', valor: selectedBatch.value.totalPP, unidade: '' },
  ]
}

const outflowColumns = [
  { title: 'Ponto de Pureza (PP)', key: 'outflowPP' },
  { title: 'Quantidade (kg)', key: 'outflowKg' },
  { title: 'Sacos', key: 'outflowSack' },
  { title: 'Uso', key: 'usage' }
]

</script>
