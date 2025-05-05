<template>
    <n-card class="inventory-card" title="Estoque">
      <!-- Tabela -->
      <AppDataTable />
    </n-card>
</template>

<script setup lang="ts">
import { NCard } from 'naive-ui'
import AppDataTable from '@/components/AppDataTable.vue'
import { useGlobalStore } from '@/stores/globalStore';
import { onMounted } from 'vue';

const globalStore = useGlobalStore()

const columns = [
  { title: 'Data', key: 'data' },
  { title: 'Texto', key: 'descricao' },
  { title: 'Categoria', key: 'categoria' },
  { title: 'Quantidade (kg)', key: 'quantidade' },
]

const data = [
  { data: '01/05/2025', descricao: 'Venda para AgroX', categoria: 'Venda', quantidade: '500' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' },
  { data: '29/04/2025', descricao: 'Recebimento Fazenda A', categoria: 'Entrada', quantidade: '700' }
]

onMounted(() => {
  globalStore.setColumns(columns)
  globalStore.setData(data)
})

</script>

<style scoped>
.inventory-card {
height: calc(100vh - 85px);
}
</style>
