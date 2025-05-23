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
        <!-- Indicadores -->
        <n-grid-item>
          <n-grid cols="1 m:4" responsive="screen" x-gap="16" y-gap="16">
            <n-grid-item v-for="(indicator, index) in keyPointIndicators" :key="index">
              <n-card :title="indicator.titulo" class="text-center shadow-lg border border-gray-200">
                <div class="text-2xl font-extrabold text-primary">{{ indicator.valor }} {{ indicator.unidade }}</div>
              </n-card>
            </n-grid-item>
          </n-grid>
        </n-grid-item>

        <!-- Gráfico de Pureza -->
        <n-grid-item>
          <n-card class="mt-6" title="Evolução mensal da pureza" style="height: 300px;">
            <div ref="chartPurezaRef" style="height: 250px;" />
          </n-card>
        </n-grid-item>

        <!-- Gráficos: Kg movimentados e Distribuição -->
        <n-grid-item>
          <n-grid cols="1 m:2" responsive="screen" x-gap="16" y-gap="16" class="mt-6">
            <n-grid-item>
              <n-card title="Kg movimentados" style="height: 300px;">
                <div ref="chartMovimentacaoRef" style="height: 250px;" />
              </n-card>
            </n-grid-item>
            <n-grid-item>
              <n-card title="Distribuição por tratamento" style="height: 300px;">
                <div ref="chartTratamentoRef" style="height: 250px;" />
              </n-card>
            </n-grid-item>
          </n-grid>
        </n-grid-item>

        <!-- Tabela de saídas -->
        <n-grid-item>
          <n-card title="Saídas do lote">
            <n-data-table :columns="saidaColumns" :data="saidaData" :pagination="false" />
          </n-card>
        </n-grid-item>
      </n-grid>
    </div>
  </n-modal>
</template>

<script setup lang="ts">
import { NModal, NCard, NGrid, NGridItem, NDataTable } from 'naive-ui'
import { nextTick, ref, watch } from 'vue'
import * as echarts from 'echarts'

const props = defineProps<{
  selectedBatch: any
}>()

const isModalOpen = defineModel('model', {
  type: Boolean,
  default: false
})

const selectedBatch = ref(props.selectedBatch)
const modalTitle = ref(`Detalhes do Lote ${props.selectedBatch}`)

const keyPointIndicators = ref<{ titulo: string; valor: any; unidade: string }[]>([])
const chartPurezaRef = ref(null)
const chartMovimentacaoRef = ref(null)
const chartTratamentoRef = ref(null)

watch(isModalOpen, async () => {
  if (isModalOpen) {
    await nextTick()
    selectedBatch.value = props.selectedBatch
    modalTitle.value = `Detalhes do Lote ${selectedBatch.value.batchNumber}`
    keyPointIndicators.value = getKeyPointIndicators()
    renderCharts()
  }
})

function renderCharts() {
  if (chartPurezaRef.value) {
    const chart = echarts.init(chartPurezaRef.value)
    chart.setOption({
      xAxis: { type: 'category', data: ['Jan', 'Fev', 'Mar', 'Abr'] },
      yAxis: { type: 'value', min: 0, max: 1 },
      series: [{
        name: 'Pureza',
        type: 'line',
        data: [0.8, 0.82, 0.85, props.selectedBatch.purenessScore],
        smooth: true
      }]
    })
  }

  if (chartMovimentacaoRef.value) {
    const chart = echarts.init(chartMovimentacaoRef.value)
    chart.setOption({
      xAxis: { type: 'category', data: ['Jan', 'Fev', 'Mar', 'Abr'] },
      yAxis: { type: 'value' },
      series: [
        { name: 'Entrada', type: 'bar', data: [300, 400, 500, 350] },
        { name: 'Saída', type: 'bar', data: [200, 250, 300, 270] }
      ]
    })
  }

  if (chartTratamentoRef.value) {
    const chart = echarts.init(chartTratamentoRef.value)
    chart.setOption({
      series: [{
        name: 'Tratamento',
        type: 'pie',
        radius: '50%',
        data: [
          { value: 400, name: 'Golden' },
          { value: 300, name: 'Podium' },
          { value: 280, name: 'Convencional' }
        ]
      }]
    })
  }
}

function closeModal(model: boolean) {
  if (!model) {
    selectedBatch.value = null
    modalTitle.value = ''
    keyPointIndicators.value = []

    if (chartPurezaRef.value) {
      const chart = echarts.getInstanceByDom(chartPurezaRef.value)
      if (chart) {
        chart.dispose()
      }
    }

    if (chartMovimentacaoRef.value) {
      const chart = echarts.getInstanceByDom(chartMovimentacaoRef.value)
      if (chart) {
        chart.dispose()
      }
    }

    if (chartTratamentoRef.value) {
      const chart = echarts.getInstanceByDom(chartTratamentoRef.value)
      if (chart) {
        chart.dispose()
      }
    }
  }
}

function getKeyPointIndicators() {
  // Precisa formata melhor isso, falta ainda alguns campos para a sacaria...
  return [
    { titulo: 'Data de validade', valor: selectedBatch.value.expireDate, unidade: '' },
    { titulo: 'Cultivar', valor: selectedBatch.value.seed, unidade: '' },
    { titulo: 'Revestimento', valor: selectedBatch.value.coating, unidade: '' },
    { titulo: 'Sacaria', valor: selectedBatch.value.sackQuantity, unidade: '' },
    { titulo: 'Quantidade Disponível', valor: selectedBatch.value.availableQuantity, unidade: 'kg' },
    { titulo: 'PP', valor: selectedBatch.value.purenessScore, unidade: '' },
    { titulo: 'Total de PP', valor: selectedBatch.value.totalPP, unidade: '' },
  ]
}

const saidaColumns = [
  { title: 'Data', key: 'data' },
  { title: 'Destino', key: 'destino' },
  { title: 'Quantidade (kg)', key: 'quantidade' },
  { title: 'Responsável', key: 'responsavel' }
]

const saidaData = [
  { data: '2025-05-01', destino: 'Cliente A', quantidade: 200, responsavel: 'João' },
  { data: '2025-05-10', destino: 'Cliente B', quantidade: 100, responsavel: 'Maria' },
  { data: '2025-05-15', destino: 'Cliente C', quantidade: 50, responsavel: 'Pedro' }
]
</script>
