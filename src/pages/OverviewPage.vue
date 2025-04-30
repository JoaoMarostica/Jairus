<template>
  <div class="p-4">
    <!-- Top cards -->
    <n-grid cols="1 m:4" responsive="screen" x-gap="16" y-gap="16">
      <n-grid-item v-for="n in 4" :key="n">
        <n-card :title="'Indicador ' + n" class="text-center">
          <div class="text-subtitle-1">Subtítulo</div>
        </n-card>
      </n-grid-item>
    </n-grid>

    <!-- Chart Section -->
    <n-card class="mt-6" title="Receita geral" style="height: 300px;">
      <div style="height: 200px; background: #595959; border-radius: 8px;"></div>
    </n-card>

    <!-- Filter + Table section -->
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

      <div style="height: 200px; background: #595959; border-radius: 6px;" class="mt-4"></div>
    </n-card>
  </div>
</template>

<script setup lang="ts">
// Nenhum script necessário por enquanto
</script>
