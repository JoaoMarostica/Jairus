<template>
  <n-card title="Lotes">
    <!-- Filtro -->
    <n-grid cols="1 m:6" responsive="screen" x-gap="16" y-gap="16">
      <n-grid-item span="m:3">
        <n-input v-model:value="search" placeholder="Buscar..." clearable />
      </n-grid-item>
    </n-grid>

    <!-- Tabela -->
    <div class="table-wrapper">
      <n-data-table
        :columns="columns"
        :data="filteredData"
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
import { ref, computed, h, Ref } from 'vue';
import { InsertChartOutlined, ModeEditOutlined } from '@vicons/material'
import AppBacthDetails from '@/components/AppBacthDetails.vue';

const isModalOpen = ref<boolean>(false);
const selectedBatch = ref<any>(null);
const search = ref('');

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
  { title: 'Número', key: 'batchNumber' },
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

// Dados dos lotes (mock)
const mockData = ref([
  {
    key: 1,
    batchNumber: '1001',
    expireDate: '2025-11-10',
    seed: 'BRS 1010',
    coating: 'TMT Azul',
    sackBrand: 'Saco Forte',
    sackQuantity: 20,
    sackWeight: 40,
    availableQuantity: 800,
    purenessScore: 0.85,
    totalPP: 680
  },
  {
    key: 2,
    batchNumber: '1002',
    expireDate: '2026-01-05',
    seed: 'BRS 1050',
    coating: 'TMT Vermelho',
    sackBrand: 'AgroBag',
    sackQuantity: 25,
    sackWeight: 40,
    availableQuantity: 1000,
    purenessScore: 0.92,
    totalPP: 920
  },
  {
    key: 3,
    batchNumber: '1003',
    expireDate: '2025-09-18',
    seed: 'CD 2728',
    coating: 'TMT Azul',
    sackBrand: 'Embalaseed',
    sackQuantity: 30,
    sackWeight: 40,
    availableQuantity: 1200,
    purenessScore: 0.88,
    totalPP: 1056
  },
  {
    key: 4,
    batchNumber: '1004',
    expireDate: '2025-12-31',
    seed: 'BRS 1010',
    coating: 'TMT Verde',
    sackBrand: 'Saco Forte',
    sackQuantity: 15,
    sackWeight: 40,
    availableQuantity: 600,
    purenessScore: 0.87,
    totalPP: 522
  },
  {
    key: 5,
    batchNumber: '1005',
    expireDate: '2026-03-15',
    seed: 'CD 2737',
    coating: 'TMT Vermelho',
    sackBrand: 'AgroBag',
    sackQuantity: 10,
    sackWeight: 40,
    availableQuantity: 400,
    purenessScore: 0.90,
    totalPP: 360
  },
  {
    key: 6,
    batchNumber: '1006',
    expireDate: '2025-08-22',
    seed: 'BRS 1010',
    coating: 'TMT Azul',
    sackBrand: 'Sementeira',
    sackQuantity: 12,
    sackWeight: 40,
    availableQuantity: 480,
    purenessScore: 0.84,
    totalPP: 403.2
  },
  {
    key: 7,
    batchNumber: '1007',
    expireDate: '2026-04-01',
    seed: 'BRS 1050',
    coating: 'TMT Verde',
    sackBrand: 'Saco Forte',
    sackQuantity: 18,
    sackWeight: 40,
    availableQuantity: 720,
    purenessScore: 0.89,
    totalPP: 640.8
  },
  {
    key: 8,
    batchNumber: '1008',
    expireDate: '2025-07-30',
    seed: 'CD 2728',
    coating: 'TMT Azul',
    sackBrand: 'AgroBag',
    sackQuantity: 22,
    sackWeight: 40,
    availableQuantity: 880,
    purenessScore: 0.91,
    totalPP: 800.8
  },
  {
    key: 9,
    batchNumber: '1009',
    expireDate: '2025-10-10',
    seed: 'BRS 1010',
    coating: 'TMT Vermelho',
    sackBrand: 'Sementeira',
    sackQuantity: 16,
    sackWeight: 40,
    availableQuantity: 640,
    purenessScore: 0.88,
    totalPP: 563.2
  },
  {
    key: 10,
    batchNumber: '1010',
    expireDate: '2026-02-20',
    seed: 'CD 2737',
    coating: 'TMT Azul',
    sackBrand: 'Embalaseed',
    sackQuantity: 14,
    sackWeight: 40,
    availableQuantity: 560,
    purenessScore: 0.93,
    totalPP: 520.8
  },
  {
    key: 11,
    batchNumber: '1011',
    expireDate: '2025-09-01',
    seed: 'BRS 1050',
    coating: 'TMT Verde',
    sackBrand: 'AgroBag',
    sackQuantity: 20,
    sackWeight: 40,
    availableQuantity: 800,
    purenessScore: 0.91,
    totalPP: 728
  },
  {
    key: 12,
    batchNumber: '1012',
    expireDate: '2026-05-10',
    seed: 'CD 2728',
    coating: 'TMT Azul',
    sackBrand: 'Sementeira',
    sackQuantity: 24,
    sackWeight: 40,
    availableQuantity: 960,
    purenessScore: 0.86,
    totalPP: 825.6
  },
  {
    key: 13,
    batchNumber: '1013',
    expireDate: '2025-06-25',
    seed: 'BRS 1010',
    coating: 'TMT Vermelho',
    sackBrand: 'Saco Forte',
    sackQuantity: 19,
    sackWeight: 40,
    availableQuantity: 760,
    purenessScore: 0.89,
    totalPP: 676.4
  },
  {
    key: 14,
    batchNumber: '1014',
    expireDate: '2026-01-10',
    seed: 'CD 2737',
    coating: 'TMT Verde',
    sackBrand: 'AgroBag',
    sackQuantity: 13,
    sackWeight: 40,
    availableQuantity: 520,
    purenessScore: 0.88,
    totalPP: 457.6
  },
  {
    key: 15,
    batchNumber: '1015',
    expireDate: '2025-10-05',
    seed: 'CD 2728',
    coating: 'TMT Azul',
    sackBrand: 'Sementeira',
    sackQuantity: 17,
    sackWeight: 40,
    availableQuantity: 680,
    purenessScore: 0.92,
    totalPP: 625.6
  },
  {
    key: 16,
    batchNumber: '1016',
    expireDate: '2025-11-15',
    seed: 'BRS 1050',
    coating: 'TMT Vermelho',
    sackBrand: 'Embalaseed',
    sackQuantity: 21,
    sackWeight: 40,
    availableQuantity: 840,
    purenessScore: 0.87,
    totalPP: 730.8
  },
  {
    key: 17,
    batchNumber: '1017',
    expireDate: '2026-03-30',
    seed: 'CD 2737',
    coating: 'TMT Azul',
    sackBrand: 'Saco Forte',
    sackQuantity: 23,
    sackWeight: 40,
    availableQuantity: 920,
    purenessScore: 0.90,
    totalPP: 828
  },
  {
    key: 18,
    batchNumber: '1018',
    expireDate: '2025-08-12',
    seed: 'BRS 1010',
    coating: 'TMT Verde',
    sackBrand: 'AgroBag',
    sackQuantity: 26,
    sackWeight: 40,
    availableQuantity: 1040,
    purenessScore: 0.85,
    totalPP: 884
  },
  {
    key: 19,
    batchNumber: '1019',
    expireDate: '2026-06-01',
    seed: 'CD 2728',
    coating: 'TMT Azul',
    sackBrand: 'Sementeira',
    sackQuantity: 11,
    sackWeight: 40,
    availableQuantity: 440,
    purenessScore: 0.93,
    totalPP: 409.2
  },
  {
    key: 20,
    batchNumber: '1020',
    expireDate: '2025-12-05',
    seed: 'BRS 1050',
    coating: 'TMT Vermelho',
    sackBrand: 'Embalaseed',
    sackQuantity: 29,
    sackWeight: 40,
    availableQuantity: 1160,
    purenessScore: 0.86,
    totalPP: 997.6
  }
])

// Normalização de texto para busca
function normalizeText(text: string | null): string {
  return text
    ? text.trim().toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, '')
    : '';
}

// Filtro de busca
const filteredData: Ref<RowData[]> = computed(() => {
  const term = normalizeText(search.value);
  if (!term) return mockData.value;

  return mockData.value.filter(item => {
    const flatText = Object.values(item)
      .map(val =>
        typeof val === 'object'
          ? Object.values(val).join(' ')
          : String(val)
      )
      .join(' ');
    return normalizeText(flatText).includes(term);
  });
});
</script>

<style scoped>
.table-wrapper {
  margin-top: 30px;
}

.n-button {
  margin-right: 6px;
}
</style>

