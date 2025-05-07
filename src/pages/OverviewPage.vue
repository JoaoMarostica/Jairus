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
      <n-card class="mt-6" title="Transações recentes" style="height: 500px;">
        <AppDataTable
          :page-size="5"
          :columns="columns"
          :data="data"
          category-key="category"
        />
      </n-card>
    </n-grid-item>
  </n-grid>
</template>

<script setup lang="ts">
import { NCard, NGrid, NGridItem } from 'naive-ui'
import { ref, onMounted } from 'vue'
import AppDataTable from '@/components/AppDataTable.vue'
import * as echarts from 'echarts'

const chartPurezaRef = ref<HTMLElement | null>(null)
const chartMovimentacaoRef = ref<HTMLElement | null>(null)
const chartTratamentoRef = ref<HTMLElement | null>(null)

const columns = [
  { title: 'Data', key: 'date' },
  { title: 'Descrição', key: 'description' },
  { title: 'Categoria', key: 'category' },
  { title: 'Quantidade (kg)', key: 'quantity' }
]

const data = [
  { date: '01/05/2025', description: 'Venda para AgroX', category: 'Venda', quantity: 500 },
  { date: '29/04/2025', description: 'Recebimento Fazenda A', category: 'Entrada', quantity: 700 },
  { date: '29/04/2025', description: 'Recebimento Fazenda B', category: 'Entrada', quantity: 300 },
  { date: '28/04/2025', description: 'Venda para AgroY', category: 'Venda', quantity: 150 },
  { date: '27/04/2025', description: 'Venda para AgroZ', category: 'Venda', quantity: 400 },
  { date: '26/04/2025', description: 'Recebimento Fazenda C', category: 'Entrada', quantity: 1200 },
  { date: '25/04/2025', description: 'Venda para AgroX', category: 'Venda', quantity: 650 },
  { date: '24/04/2025', description: 'Recebimento Fazenda D', category: 'Entrada', quantity: 500 },
  { date: '23/04/2025', description: 'Venda para AgroY', category: 'Venda', quantity: 800 },
  { date: '22/04/2025', description: 'Recebimento Fazenda E', category: 'Entrada', quantity: 450 },
  { date: '21/04/2025', description: 'Venda para AgroZ', category: 'Venda', quantity: 350 },
  { date: '20/04/2025', description: 'Venda para AgroW', category: 'Venda', quantity: 950 },
  { date: '19/04/2025', description: 'Recebimento Fazenda F', category: 'Entrada', quantity: 1300 },
  { date: '18/04/2025', description: 'Venda para AgroX', category: 'Venda', quantity: 600 },
  { date: '17/04/2025', description: 'Recebimento Fazenda G', category: 'Entrada', quantity: 800 },
  { date: '16/04/2025', description: 'Venda para AgroY', category: 'Venda', quantity: 700 },
  { date: '15/04/2025', description: 'Recebimento Fazenda H', category: 'Entrada', quantity: 450 },
  { date: '14/04/2025', description: 'Venda para AgroZ', category: 'Venda', quantity: 500 },
  { date: '13/04/2025', description: 'Recebimento Fazenda I', category: 'Entrada', quantity: 1100 },
  { date: '12/04/2025', description: 'Venda para AgroW', category: 'Venda', quantity: 400 },
  { date: '11/04/2025', description: 'Recebimento Fazenda J', category: 'Entrada', quantity: 850 }
]


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

<style scoped>
.table-wrapper {
margin-top: 30px;
}
</style>
