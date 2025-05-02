<template>
    <n-card class="inventory-card" title="Estoque">
      <!-- Filtros -->
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
  
      <!-- Tabela -->
      <div class="table-wrapper">
        <n-data-table
          :columns="columns"
          :data="data"
          :pagination="{ pageSize: 12 }"
          scroll-x="max-content"
        />
      </div>
    </n-card>
</template>

<script setup lang="ts">
import { NCard, NGrid, NGridItem, NInput, NSelect, NDataTable } from 'naive-ui'

const columns = [
{ title: 'Data', key: 'data' },
{ title: 'Descrição', key: 'descricao' },
{ title: 'Categoria', key: 'categoria' },
{ title: 'Quantidade (kg)', key: 'quantidade' }
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
</script>

<style scoped>
.inventory-card {
height: calc(100vh - 85px);
}

.table-wrapper {
margin-top: 30px;
}
</style>
