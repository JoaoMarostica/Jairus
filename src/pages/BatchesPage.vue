<template>
  <n-card title="Lotes">
    <!-- Filtro -->
    <n-grid cols="1 m:6" responsive="screen" x-gap="16" y-gap="16">
      <n-grid-item span="m:3">
        <n-input
          @change="handleSearch"
          placeholder="Buscar..."
          autosize
          style="min-width: 50%"
          clearable
        />
      </n-grid-item>
    </n-grid>

    <!-- Tabela -->
    <div class="table-wrapper">
      <n-data-table
        :columns="columns"
        :data="filteredData"
        :pagination="pagination"
        scroll-x="max-content"
      />
    </div>
    
    <!-- Modal de detalhes -->
    <AppBacthDetails v-model:model="isModalOpen" :selectedBatch="selectedBatch"/>
  </n-card>
</template>

<script setup lang="ts">
import {
  NCard,
  NGrid,
  NGridItem,
  NInput,
  NDataTable,
  NButton,
} from 'naive-ui';
import { RowData, TableColumn } from 'naive-ui/es/data-table/src/interface';
import { ref, computed, h, Ref, reactive } from 'vue';
import { InsertChartOutlined, ModeEditOutlined } from '@vicons/material'
import AppBacthDetails from '@/components/AppBacthDetails.vue';
import { useBatchesStore } from '@/stores/batchesStore';
import { storeToRefs } from 'pinia';

const isModalOpen = ref<boolean>(false);

const batchesStore = useBatchesStore();
const { batches } = storeToRefs(batchesStore);

const selectedBatch = ref<any>(null);
const search = ref('');

const pagination = reactive({
  page: 1,
  pageSize: 25,
  pageSizes: [10, 25, 50, 100],
  showSizePicker: true,
  onchange: (newPage: number) => {
    pagination.page = newPage
  },
  onUpdatePageSize: (newSize: number) => {
    pagination.pageSize = newSize
    pagination.page = 1
  }
})

function handleSearch(searchTerm: string) {
  search.value = searchTerm;
}

// Filtro de busca
const filteredData: Ref<RowData[]> = computed(() => {
  const term = normalizeText(search.value);
  if (!term) return batches.value;

  const filteredBatches = batches.value.filter(batch => batch._searchIndex.includes(term));
  const start = (pagination.page - 1) * pagination.pageSize;

  return filteredBatches.slice(start, start + pagination.pageSize)
});

// Ações
function openBatchDetails(batch: any) {
  selectedBatch.value = batch;
  isModalOpen.value = true;
}

function editBatch(batch: any) {
  console.log('Editar', batch);
  // Implementar redirecionamento ou modal de edição
}

// Colunas da tabela
const columns = ref<TableColumn<RowData>[]>([
  {
    type: 'selection' as const,
    disabled(row: RowData) {
      return (row as any).name === 'Edward King 3'
    }
  },
  { title: 'Número', key: 'number' },
  { title: 'Ano', key: 'year' },
  { title: 'Data de Validade', key: 'expireDate' },
  { title: 'Cultivar', key: 'seed' },
  { title: 'Revestimento', key: 'coating' },
  {
    title: 'Sacaria',
    key: 'sack',
    children: [
      { title: 'Marca', key: 'sackBrand' },
      { title: 'Quantidade', key: 'sackQuantity' },
      { title: 'Peso', key: 'sackWeight' }
    ]
  },
  { title: 'Quantidade (kg)', key: 'availableQuantity' },
  { title: 'Ponto de Pureza (PP)', key: 'purenessScore' },
  { title: 'Total PP', key: 'totalPP' },
  {
    title: 'Ações',
    key: 'actions',
    render(batch: any) {
      return [
        h(
          NButton,
          {
            renderIcon: () => h(InsertChartOutlined),
            onClick: () => openBatchDetails(batch)
          }
        ),
        h(
          NButton,
          {
            renderIcon: () => h(ModeEditOutlined),
            onClick: () => editBatch(batch)
          }
        )
      ];
    }
  }
]);

// Normalização de texto para busca
function normalizeText(text: string | null): string {
  return text
    ? text.trim().toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, '')
    : '';
}
</script>

<style scoped>
.table-wrapper {
  margin-top: 30px;
}

.n-button {
  margin-right: 6px;
}
</style>

