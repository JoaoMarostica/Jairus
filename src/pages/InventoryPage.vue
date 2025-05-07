<template>
    <n-card class="inventory-card" title="Estoque">
      <!-- Tabela -->
      <AppDataTable
        :columns="columns"
        :data="data"
        category-key="category"
      />
    </n-card>
</template>

<script setup lang="ts">
import { NCard } from 'naive-ui'
import AppDataTable from '@/components/AppDataTable.vue'

const columns = [
  { title: 'Código', key: 'code' },
  { title: 'Nome do Produto', key: 'productName' },
  { title: 'Categoria', key: 'category' },
  { title: 'Quantidade Disponível (kg)', key: 'availableQuantity' },
  { title: 'Preço Unitário (R$)', key: 'unityPrice' }
]

const data = [
  { code: '001', productName: 'Semente de Soja', category: 'Grão', availableQuantity: 1200, unityPrice: 25.5 },
  { code: '002', productName: 'Semente de Milho', category: 'Grão', availableQuantity: 800, unityPrice: 15.0 },
  { code: '003', productName: 'Fertilizante A', category: 'Fertilizante', availableQuantity: 150, unityPrice: 80.0 },
  { code: '004', productName: 'Adubo Orgânico', category: 'Fertilizante', availableQuantity: 350, unityPrice: 40.0 },
  { code: '005', productName: 'Semente de Feijão', category: 'Grão', availableQuantity: 500, unityPrice: 20.0 },
  { code: '006', productName: 'Semente de Trigo', category: 'Grão', availableQuantity: 1000, unityPrice: 18.0 },
  { code: '007', productName: 'Calcário', category: 'Fertilizante', availableQuantity: 300, unityPrice: 60.0 },
  { code: '008', productName: 'Fungicida B', category: 'Proteção', availableQuantity: 200, unityPrice: 45.0 },
  { code: '009', productName: 'Inseticida C', category: 'Proteção', availableQuantity: 150, unityPrice: 55.0 },
  { code: '010', productName: 'Semente de Arroz', category: 'Grão', availableQuantity: 950, unityPrice: 22.5 },
  { code: '011', productName: 'Adubo Fosfatado', category: 'Fertilizante', availableQuantity: 400, unityPrice: 50.0 },
  { code: '012', productName: 'Herbicida D', category: 'Proteção', availableQuantity: 500, unityPrice: 35.0 },
  { code: '013', productName: 'Semente de Girassol', category: 'Grão', availableQuantity: 300, unityPrice: 28.0 },
  { code: '014', productName: 'Semente de Trigo', category: 'Grão', availableQuantity: 700, unityPrice: 19.5 },
  { code: '015', productName: 'Fertilizante E', category: 'Fertilizante', availableQuantity: 600, unityPrice: 75.0 },
  { code: '016', productName: 'Adubo NPK', category: 'Fertilizante', availableQuantity: 500, unityPrice: 45.0 },
  { code: '017', productName: 'Semente de Feijão', category: 'Grão', availableQuantity: 350, unityPrice: 20.0 },
  { code: '018', productName: 'Pesticida F', category: 'Proteção', availableQuantity: 250, unityPrice: 30.0 },
  { code: '019', productName: 'Semente de Milho', category: 'Grão', availableQuantity: 1200, unityPrice: 15.5 },
  { code: '020', productName: 'Fertilizante H', category: 'Fertilizante', availableQuantity: 800, unityPrice: 65.0 }
]

</script>

<style scoped>
.inventory-card {
height: calc(100vh - 85px);
}
</style>
