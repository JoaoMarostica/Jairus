<template>
  <!-- Loading Batches -->
  <n-card v-if="loading">
    <n-empty description="Carregando Lotes..." size="large">
      <template #icon>
        <n-icon>
          <HourglassBottomRound />
        </n-icon>
      </template>
    </n-empty>
  </n-card>

  <!-- Batches -->
  <n-card title="Lotes" v-else>
    <!-- Filter -->
    <n-grid cols="1 m:6" responsive="screen" x-gap="16" y-gap="16" v-if="dataTableBatches.length !== 0">
      <n-grid-item span="m:3">
        <n-input-group>
          <n-input
            @change="handleSearch"
            placeholder="Pesquisar"
            autosize
            :style="{ width: '60%' }"
            clearable
          />
          <n-select
            v-model:value="columnFilter"
            filterable
            placeholder="Coluna"
            :options="columnFilterOptions"
            :style="{ width: '20%' }"
            clearable
          />
          <n-select
            v-model:value="yearFilter"
            filterable
            placeholder="Ano"
            :options="yearFilterOptions"
            :style="{ width: '20%' }"
            clearable
          />
        </n-input-group>
      </n-grid-item>
      <n-grid-item :span="2">
        <n-space>
          <n-button strong secondary type="info" @click="openCreateBatchModal">
            <template #icon>
              <n-icon>
                <PlusOutlined />
              </n-icon>
            </template>
            Novo Lote
          </n-button>
          <n-button strong secondary type="error" @click="handleRemoveSelected" v-if="selectedBatches.length !== 0">
            <template #icon>
              <n-icon>
                <DeleteForeverOutlined />
              </n-icon>
            </template>
            Remover Lotes Selecionados
          </n-button>
        </n-space>
      </n-grid-item>
    </n-grid>

    <!-- Data Table -->
    <div class="table-wrapper">
      <n-card v-if="dataTableBatches.length === 0">
        <n-empty description="Nenhum Lote Encontrado" size="large">
          <template #extra>
            <n-space>
              <n-button secondary type="info" size="small" @click="fileUploadModal = true" class="empty-button">
                <template #icon>
                  <n-icon>
                    <UploadFileOutlined />
                  </n-icon>
                </template>
                Importar Planilha
              </n-button>
              ou
              <n-button secondary type="info" size="small" @click="openCreateBatchModal" class="empty-button">
                <template #icon>
                  <n-icon>
                    <PlusOutlined />
                  </n-icon>
                </template>
                Novo Lote
              </n-button>
            </n-space>
          </template>
        </n-empty>
      </n-card>
      <n-data-table
        v-else
        :columns="columns"
        :data="filteredData"
        :pagination="pagination"
        @update:checked-row-keys="handleCheck"
        @update:sorter="handleUpdateSorter"
        :max-height="640"
      />
    </div>

    <!-- Batch Details Modal -->
    <AppBatchDetails v-model:modal="batchDetailsModal" :selectedBatch="selectedBatch"/>
    
    <!-- Batch CRUD -->
    <AppCreateBatch v-model:modal="createBatchModal"/>
    <AppEditBatch v-model:modal="editBatchModal" :selectedBatch="selectedBatch" />
    <AppRemoveBatch v-model:modal="removeBatchModal" :selectedBatch="selectedBatch" :multiple="multipleRemove" />
    
    <!-- Create Outflow Modal -->
    <AppCreateOutflow v-model:modal="createOutflowModal"  :selectedBatch="selectedBatch"/>
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
  NIcon,
  NSelect,
  NInputGroup,
  NEmpty,
  NSpace,
  NTag,
  NTooltip,
  NDropdown,
} from 'naive-ui';
import type { DataTableRowKey } from 'naive-ui'
import { RowData, TableColumn } from 'naive-ui/es/data-table/src/interface';
import * as batchesUtils from '@/utils/batches'
import { ref, computed, reactive, watch, onMounted, h } from 'vue';
import { AutoAwesomeMosaicOutlined, EditOutlined, DeleteForeverOutlined, MoreVertOutlined, PlusOutlined, UploadFileOutlined, HourglassBottomRound, PostAddOutlined } from '@vicons/material'
import AppBatchDetails from '@/components/AppBatchDetails.vue';
import AppCreateOutflow from '@/components/AppCreateOutflow.vue';
import AppCreateBatch from '@/components/AppCreateBatch.vue';
import AppEditBatch from '@/components/AppEditBatch.vue';
import AppRemoveBatch from '@/components/AppRemoveBatch.vue';
import { useBatchesStore } from '@/stores/batchesStore';
import { useGlobalStore } from '@/stores/globalStore';
import { storeToRefs } from 'pinia';
import type { DataTableBatch } from '@/types/batches';

type Sorter = {
  columnKey: string;
  order: 'ascend' | 'descend' | false;
};

const batchDetailsModal = ref<boolean>(false);
const createOutflowModal = ref<boolean>(false);
const createBatchModal = ref<boolean>(false);
const editBatchModal = ref<boolean>(false);
const removeBatchModal = ref<boolean>(false);
const multipleRemove = ref<boolean>(false);

const globalStore = useGlobalStore()
const { fileUploadModal } = storeToRefs(globalStore);

const batchesStore = useBatchesStore();
const { dataTableBatches, selectedBatches } = storeToRefs(batchesStore);

const selectedBatch = ref<any>(null);
const search = ref('');

const sortKeyMapOrder = computed<Record<string, 'ascend' | 'descend' | false>>(() =>
  sortStates.value.reduce<Record<string, 'ascend' | 'descend' | false>>((result, { columnKey, order }) => {
    result[columnKey] = order || false
    return result
  }, {})
)
const sortStates = ref<Sorter[]>([]);

const columns = ref<TableColumn<RowData>[]>([]);
const columnFilter = ref(null);
const columnFilterOptions = ref<{label: string, value: string}[]>([]);
const yearFilter = ref<string>(new Date().getFullYear().toString());
const yearFilterOptions = ref<{label: string, value: string}[]>([]);

const loading = ref(true)

const pagination = reactive({
  page: 1,
  pageSize: 25,
  pageSizes: [10, 25, 50, 100],
  showSizePicker: true,
  onUpdatePage: (newPage: number) => {
    pagination.page = newPage
  },
  onUpdatePageSize: (newPageSize: number) => {
    pagination.pageSize = newPageSize
    pagination.page = 1
  }
})

const filteredData = computed(() => {
  const term = batchesUtils.normalizeText(search.value);
  const column = columnFilter.value || 'all';
  const year = yearFilter.value || 'all';

  // já evita processar se não tiver dados
  if (!dataTableBatches.value.length) return [];

  let result = dataTableBatches.value;

  // Filtro por ano
  if (year !== 'all') {
    result = result.filter(batch => batch.batch_year?.toString() === year);
  }

  // Filtro por coluna
  if (term) {
    result = result.filter(batch => {
      if (column === 'all') {
        return batch._searchIndex?.includes(term);
      } else if (column in batch) {
        const val = batch[column as keyof DataTableBatch];
        return batchesUtils.normalizeText((val as any)?.toString() ?? '').includes(term);
      } else {
        return false;
      }
    });
  }

  return result
});

onMounted(async () => {
  try {
    loading.value = true
    multipleRemove.value = false;
    selectedBatches.value = [];
    selectedBatch.value = null;

    createColumns();
    setColumnFilterOptions();
    setYearFilterOptions();

    try {
      await batchesStore.fetchBatches();

      globalStore.showMessage({
      content: 'Lotes carregados com sucesso!',
      type: 'success',
    });
    } catch (error: any) {
      globalStore.showMessage({
        content: `Erro ao carregar lotes: ${error?.message || error}`,
        type: 'error',
        keepAliveOnHover: true,
      })
    }
  } catch (err) {
    globalStore.showMessage({
      content: `Erro ao carregar lotes: ${err instanceof Error ? err.message : String(err)}`,
      type: 'error',
    });
  }
  loading.value = false
})

watch(dataTableBatches.value, () => {
  globalStore.showMessage({
    content: 'Lotes atualizados com sucesso!',
    type: 'success',
  });
});

function openCreateBatchModal() {
  createBatchModal.value = true
}

function handleCheck(rowKeys: DataTableRowKey[]) {
  selectedBatches.value = rowKeys
}

function handleRemoveSelected() {
  if (selectedBatches.value.length === 0) {
    globalStore.showMessage({
      content: 'Nenhum lote selecionado para remoção.',
      type: 'warning',
    });
    return;
  }
  removeBatchModal.value = true;
  multipleRemove.value = true;
}

function handleSearch(searchTerm: string) {
  search.value = searchTerm;
}

function handleUpdateSorter(sorter: Sorter[]) {
  const s = sorter[sorter.length - 1];

  if (!s) {
    sortStates.value = [];
    return;
  }

  const current = sortStates.value.find(state => state.columnKey === s.columnKey);
  let nextOrder: 'ascend' | 'descend' | false;

  if (!current) {
    nextOrder = 'ascend';
  } else if (current.order === 'ascend') {
    nextOrder = 'descend';
  } else if (current.order === 'descend') {
    nextOrder = false;
  } else {
    nextOrder = 'ascend';
  }

  if (nextOrder) {
    sortStates.value = [{ columnKey: s.columnKey, order: nextOrder }];
  } else {
    sortStates.value = [];
  }
}

// Ações
function openBatchDetails(batch: any) {
  selectedBatch.value = batch;
  batchDetailsModal.value = true;
}

function createOutflow(batch: any) {
  selectedBatch.value = batch;
  createOutflowModal.value = true;
}

function handleEdit(batch: any) {
  selectedBatch.value = batch;
  editBatchModal.value = true;
}

function handleRemove(batch: any) {
  selectedBatch.value = batch;
  removeBatchModal.value = true;
}

async function setColumnFilterOptions() {
  columns.value.forEach((column: any) => {
    if (column.type !== 'selection' && column.key !== 'actions' && column.key !== 'year') {
      if (column.key === 'sack') {
        column.children.forEach((child: any) => {
          columnFilterOptions.value.push({
            label: child.title,
            value: child.key
          });
        });
      } else {
        columnFilterOptions.value.push({
            label: column.title,
            value: column.key
          });
      }
    }
  });
}

async function setYearFilterOptions() {
  const uniqueYears = Array.from(new Set(dataTableBatches.value
    .map(batch => batch.batch_year)
    .filter(year => year != null)));

  yearFilterOptions.value = [
    ...uniqueYears.sort().map(year => ({
      label: String(year),
      value: String(year)
    }))
  ];
}

watch(sortStates, () => {
  createColumns();
});

function createColumns() {
  columns.value = [
    {
      type: 'selection' as const,
      disabled(row: RowData) {
        return (row as any).number === '0'
      }
    },
    { 
      title: 'Lote', 
      key: 'batch_number',
      sortOrder: sortKeyMapOrder.value['batch_number'] || false,
      sorter: {
        compare: (a: RowData, b: RowData) => (a as DataTableBatch).batch_number - (b as DataTableBatch).batch_number,
        multiple: 1
      }
    },
    { 
      title: 'Ano', 
      key: 'batch_year',
      sortOrder: sortKeyMapOrder.value['batch_year'] || false,
      sorter: {
        compare: (a: RowData, b: RowData) => (a as DataTableBatch).batch_year - (b as DataTableBatch).batch_year,
        multiple: 1
      }
    },
    {
      title: 'Vencimento',
      key: 'expire_date',
      sortOrder: sortKeyMapOrder.value['expire_date'] || false,
      sorter: {
        compare: (a: RowData, b: RowData) => {
          const dateA = batchesUtils.parseExpireDate((a as DataTableBatch).expire_date);
          const dateB = batchesUtils.parseExpireDate((b as DataTableBatch).expire_date);

          if (!dateA && !dateB) return 0;
          if (!dateA) return -1;
          if (!dateB) return 1;

          return dateA.getTime() - dateB.getTime();
        },
        multiple: 1
      }
    },
    { title: 'Cultivar', key: 'seed' },
    { title: 'Tratamento', key: 'coating' },
    {
      title: 'Sacaria',
      key: 'sack',
      titleAlign: 'center',
      align: 'center',
      children: [
        { title: 'Marca', key: 'brand' },
        { 
          title: 'Quantidade', 
          key: 'sack_amount',
          titleAlign: 'center',
          align: 'center',
          sortOrder: sortKeyMapOrder.value['sack_amount'] || false,
          sorter: {
            compare: (a: RowData, b: RowData) => (a as DataTableBatch).sack_amount - (b as DataTableBatch).sack_amount,
            multiple: 1
          }
        },
        { 
          title: 'Peso', 
          key: 'sack_weight',
          titleAlign: 'center',
          align: 'center',
          sortOrder: sortKeyMapOrder.value['sack_weight'] || false,
          sorter: {
            compare: (a: RowData, b: RowData) => (a as DataTableBatch).sack_weight - (b as DataTableBatch).sack_weight,
            multiple: 1
          }
        }
      ]
    },
    { 
      title: 'Quantidade (kg)', 
      key: 'total_weight',
      titleAlign: 'center',
      align: 'center',
      sortOrder: sortKeyMapOrder.value['total_weight'] || false,
      sorter: {
        compare: (a: any, b: any) => a.total_weight - b.total_weight,
        multiple: 1
      }
    },
    { 
      title: 'PP/Kg', 
      key: 'pureness_score',
      sortOrder: sortKeyMapOrder.value['pureness_score'] || false,
      sorter: {
        compare: (a: RowData, b: RowData) => batchesUtils.parseBrazilianNumber(a.pureness_score) - batchesUtils.parseBrazilianNumber(b.pureness_score),
        multiple: 1
      }
    },
    { 
      title: 'Total PP', 
      key: 'total_pureness_score',
      sortOrder: sortKeyMapOrder.value['total_pureness_score'] || false,
      sorter: {
        compare: (a: RowData, b: RowData) => batchesUtils.parseBrazilianNumber(a.total_pureness_score) - batchesUtils.parseBrazilianNumber(b.total_pureness_score),
        multiple: 1
      }
    },
    { 
      title: 'Status', 
      key: 'batch_status',
      titleAlign: 'center',
      align: 'center',
        render(row: RowData): ReturnType<typeof h> {
        return h(
            NTag,
            {
            type: batchesUtils.getStatusType(row.batch_status),
            bordered: false
            },
            { default: () => batchesUtils.getStatusLabel(row.batch_status) }
        )
        }
    },
    {
      title: 'Ações',
      key: 'actions',
      titleAlign: 'center',
      align: 'center',
      width: '150px',
      render(batch: RowData): ReturnType<typeof h>[]  {
        return [
          h(
            NTooltip,
            { placement: 'bottom' },
            {
              trigger: () =>
                h(
                  NButton,
                  {
                    quaternary: true,
                    type: 'primary',
                    size: 'small',
                    onClick: () => openBatchDetails(batch),
                    renderIcon: () =>h(NIcon, null, 
                      { default: () => 
                        h(AutoAwesomeMosaicOutlined, {
                          style: { color: '#2080f0' }
                        }) 
                      })
                  }
                ),
              default: () => 'Ver detalhes'
            }
          ),
          h(
            NTooltip,
            { placement: 'bottom' },
            {
              trigger: () =>
                h(
                  NButton,
                  {
                    quaternary: true,
                    type: 'primary',
                    size: 'small',
                    onClick: () => createOutflow(batch),
                    renderIcon: () =>h(NIcon, null, 
                      { default: () => 
                        h(PostAddOutlined, {
                          style: { color: '#04853a' }
                        }) 
                      })
                  }
                ),
              default: () => 'Adicionar Saída'
            }
          ),
          h(
            NDropdown,
            {
              trigger: "click",
              options: [
                {
                  label: 'Editar',
                  key: 'edit',
                  icon: () => h(NIcon, null, { default: () => h(EditOutlined) })
                },
                {
                  label: 'Remover',
                  key: 'delete',
                  icon: () => h(NIcon, {
                    color: 'red'
                  }, { default: () => h(DeleteForeverOutlined) })
                }
              ],
              onSelect: (key: string) => {
                if (key === 'edit') {
                  handleEdit(batch)
                } else if (key === 'delete') {
                  handleRemove(batch)
                }
              },
              placement: 'bottom'
            },
            {
              default: () =>
                h(
                  NButton,
                  {
                    quaternary: true,
                    size: 'small',
                    renderIcon: () => h(NIcon, null, { default: () => h(MoreVertOutlined) })
                  }
                )
            }
          )
        ];
      }
    }
  ]
}

</script>

<style scoped>
.table-wrapper {
  margin-top: 30px;
  width: 100%;
  overflow-x: auto;
}

.n-data-table {
  width: 100%;
  min-width: 100%;
}

.n-button {
  margin-right: 6px;
}

.empty-button{
  margin: 0;
}
</style>

