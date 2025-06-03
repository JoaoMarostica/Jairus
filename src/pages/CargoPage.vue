<template>
  <n-grid cols="1" responsive="screen" x-gap="16" y-gap="16">
    <n-grid-item>  
      <n-card class="cargotable-card" title="Carga">
        <!-- Tabela -->
        <AppDataTable
          :columns="columns"
          :data="data"
          :page-size="5"
          category-key="date"
        /> 
      </n-card>
    </n-grid-item>

    <!-- Gráfico -->
    <n-grid-item>
      <n-grid cols="1 m:2" x-gap="16" y-gap = "16" class="mt-6" padding="16">
        <n-grid-item>
          <n-card title="Quebra por Carga" style="height: 300px;">
            <div ref="chartQuebraCargaRef" style="height: 250px;" />
          </n-card>
        </n-grid-item>
      </n-grid>
    </n-grid-item> 
  
</n-grid>
</template>

<script setup lang="ts">
import { NCard, NGrid, NGridItem } from 'naive-ui'
import { ref, onMounted } from 'vue'
import AppDataTable from '@/components/AppDataTable.vue'
import * as echarts from 'echarts'

const chartQuebraCargaRef = ref<HTMLElement | null>(null)

const columns = [
{ title: 'Código', key: 'code' },
{ title: 'Cultivar', key: 'productName' },
{ title: 'Data', key: 'date' },
{ title: 'PP estimado vs. PP real', key: 'estimatedPP' },
{ title: 'Quebra(%)', key: 'breakage' }
]

const data = [
{ code: '001', productName: 'Semente de Soja', date: '12/04/2023', estimatedPP: 1200, breakage: 25.5 },
{ code: '002', productName: 'Semente de Milho', date: '21/04/2023', estimatedPP: 800, breakage: 15.0 },
{ code: '003', productName: 'Fertilizante A', date: '12/04/2023', estimatedPP: 150, breakage: 80.0 },
{ code: '004', productName: 'Adubo Orgânico', date: '03/04/2023', estimatedPP: 350, breakage: 40.0 },
{ code: '005', productName: 'Semente de Feijão', date: '12/05/2023', estimatedPP: 500, breakage: 20.0 },
{ code: '006', productName: 'Semente de Trigo', date: '13/05/2023', estimatedPP: 1000, breakage: 18.0 },
{ code: '007', productName: 'Calcário', date: '13/04/2023', estimatedPP: 300, breakage: 60.0 },
{ code: '008', productName: 'Fungicida B', date: '11/02/2023', estimatedPP: 200, breakage: 45.0 },
{ code: '009', productName: 'Inseticida C', date: '13/04/2023', estimatedPP: 150, breakage: 55.0 },
{ code: '010', productName: 'Semente de Arroz', date: '15/04/2023', estimatedPP: 950, breakage: 22.5 },
{ code: '011', productName: 'Adubo Fosfatado', date: '19/03/2023', estimatedPP: 400, breakage: 50.0 },
{ code: '012', productName: 'Herbicida D', date: '18/05/2023', estimatedPP: 500, breakage: 35.0 },
{ code: '013', productName: 'Semente de Girassol', date: '10/04/2023', estimatedPP: 300, breakage: 28.0 },
{ code: '014', productName: 'Semente de Trigo', date: '13/05/2023', estimatedPP: 700, breakage: 19.5 },
{ code: '015', productName: 'Fertilizante E', date: '16/04/2023', estimatedPP: 600, breakage: 75.0 },
{ code: '016', productName: 'Adubo NPK', date: '16/04/2023', estimatedPP: 500, breakage: 45.0 },
{ code: '017', productName: 'Semente de Feijão', date: '12/05/2023', estimatedPP: 350, breakage: 20.0 },
{ code: '018', productName: 'Pesticida F', date: '03/05/2023', estimatedPP: 250, breakage: 30.0 },
{ code: '019', productName: 'Semente de Milho', date: '21/04/2023', estimatedPP: 1200, breakage: 15.5 },
{ code: '020', productName: 'Fertilizante H', date: '04/05/2023', estimatedPP: 800, breakage: 65.0 }
]

onMounted(() => {
  if (chartQuebraCargaRef.value) {
    const chart = echarts.init(chartQuebraCargaRef.value)
    chart.setOption({
      tooltip: { trigger: 'axis' },
      legend: { data: ['Entrada', 'Saída'] },
      xAxis: { type: 'category', data: quebraporCarga.meses },
      yAxis: { type: 'value' },
      series: [
        { name: 'Entrada', type: 'bar', data: quebraporCarga.entrada },
        
      ]
    })
  }

})

const quebraporCarga = {
  meses: ['Jan', 'Fev', 'Mar', 'Abr', 'Mai'],
  entrada: [1200, 1500, 1300, 1100, 1700],
  saida: [800, 1000, 900, 950, 1300]
}


</script>

<style scoped>
.cargotable-card {
  height: calc(65vh - 85px);
  
}

.n-grid {
  overflow: hidden; /* Evita barra de rolagem no grid */
}


</style>