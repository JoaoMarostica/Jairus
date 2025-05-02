<template>
  <n-grid cols="1" responsive="screen" x-gap="16" y-gap="16">
    <n-grid-item>
      <!-- Top cards -->
      <n-grid cols="1 m:4" responsive="screen" x-gap="16" y-gap="16">
        <n-grid-item v-for="(indicator, index) in keyPointIndicators" :key="index">
          <n-card :title="indicator.titulo" class="text-center">
            <div class="text-xl font-bold">{{ indicator.valor }} {{ indicator.unidade }}</div>
          </n-card>
        </n-grid-item>
      </n-grid>
    </n-grid-item>

    <!-- Chart Section -->
    <n-grid-item>
      <n-card class="mt-6" title="Evolução mensal da pureza" style="height: 300px;">
        <div ref="chartPurezaRef" style="height: 250px;" />
      </n-card>
    </n-grid-item>

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

    <!-- Filter + Table section -->
    <n-grid-item>
      <n-card class="mt-6" title="Transações recentes">
        <n-grid cols="1 m:6" responsive="screen" x-gap="16" y-gap="16">
          <n-grid-item span="m:3">
            <n-input placeholder="Buscar..." />
          </n-grid-item>
          <n-grid-item span="m:1.5">
            <n-select :options="['Categoria 1', 'Categoria 2'].map(i => ({ label: i, value: i }))" placeholder="Filtro" />
          </n-grid-item>
          <n-grid-item span="m:1.5">
            <n-select :options="['Mais recentes', 'Mais antigas'].map(i => ({ label: i, value: i }))" placeholder="Ordenar" />
          </n-grid-item>
        </n-grid>
  
        <n-data-table :columns="columns" :data="data" class="mt-4" />
      </n-card>
    </n-grid-item>
  </n-grid>
</template>

<script setup lang="ts">
import { NCard, NGrid, NGridItem, NInput, NSelect, NDataTable } from 'naive-ui'
import { ref, onMounted } from 'vue'
import * as echarts from 'echarts'

const chartPurezaRef = ref<HTMLElement | null>(null)
const chartMovimentacaoRef = ref<HTMLElement | null>(null)
const chartTratamentoRef = ref<HTMLElement | null>(null)


onMounted(() => {
  if (chartPurezaRef.value) {
    const chart = echarts.init(chartPurezaRef.value)
    chart.setOption({
      xAxis: { type: 'category', data: evolucaoPureza.meses },
      yAxis: { type: 'value', min: 0, max: 100 },
      series: [{
        name: 'Pureza (%)',
        type: 'line',
        data: evolucaoPureza.pureza,
        smooth: true
      }]
    })
  }

  if (chartMovimentacaoRef.value) {
    const chart = echarts.init(chartMovimentacaoRef.value)
    chart.setOption({
      tooltip: { trigger: 'axis' },
      legend: { data: ['Entrada', 'Saída'] },
      xAxis: { type: 'category', data: movimentacaoKg.meses },
      yAxis: { type: 'value' },
      series: [
        { name: 'Entrada', type: 'bar', data: movimentacaoKg.entrada },
        { name: 'Saída', type: 'bar', data: movimentacaoKg.saida }
      ]
    })
  }

  if (chartTratamentoRef.value) {
    const chart = echarts.init(chartTratamentoRef.value)
    chart.setOption({
      tooltip: { trigger: 'item' },
      legend: { orient: 'vertical', left: 'left' },
      series: [
        {
          name: 'PP por tratamento',
          type: 'pie',
          radius: '60%',
          data: tratamentoPP.tipos.map((tipo, i) => ({
            name: tipo,
            value: tratamentoPP.pontos[i]
          }))
        }
      ]
    })
  }
})

const keyPointIndicators = [
  { titulo: 'PP Entrados', valor: '12.300', unidade: 'pts' },
  { titulo: 'PP Vendidos', valor: '9.500', unidade: 'pts' },
  { titulo: 'Estoque atual', valor: '2.800', unidade: 'pts' },
  { titulo: 'Quebra média', valor: '18%', unidade: '' }
]

const columns = [
  { title: 'Data', key: 'data' },
  { title: 'Descrição', key: 'descricao' },
  { title: 'Categoria', key: 'categoria' },
  { title: 'Quantidade (kg)', key: 'quantidade' }
]

const data = [
  { data: '01/05/2025', descricao: 'Venda para AgroX', categoria: 'Venda', quantidade: '500' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' }
]

const evolucaoPureza = {
  meses: ['Jan', 'Fev', 'Mar', 'Abr', 'Mai'],
  pureza: [92, 88, 90, 89, 91]
}

const movimentacaoKg = {
  meses: ['Jan', 'Fev', 'Mar', 'Abr', 'Mai'],
  entrada: [1200, 1500, 1300, 1100, 1700],
  saida: [800, 1000, 900, 950, 1300]
}

const tratamentoPP = {
  tipos: ['Golden', 'Podium', 'Convencional'],
  pontos: [4000, 3500, 2800]
}


</script>
