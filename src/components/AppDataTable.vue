<template>
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
  
  <div class="table-wrapper">
    <n-data-table
      :columns="columns"
      :data="data"
      :pagination="{ pageSize: pageSize }"
      scroll-x="max-content"
    />
  </div>
</template>

<script setup lang="ts">
import { NGrid, NGridItem, NInput, NSelect, NDataTable } from 'naive-ui'
import { useGlobalStore } from '@/stores/globalStore';
import { storeToRefs } from 'pinia';
import { ref, watch } from 'vue';

const globalStore = useGlobalStore()
const { columns, data } = storeToRefs(globalStore)

const props = defineProps(['pageSize'])
const pageSize = ref(props.pageSize) || ref(12)

watch(props, () => {
  pageSize.value = props.pageSize
})

</script>

<style scoped>
.table-wrapper {
margin-top: 30px;
}
</style>
