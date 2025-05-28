<template>
  <n-card title="Lotes">
    <!-- Filtro -->
    <n-grid cols="1 m:6" responsive="screen" x-gap="16" y-gap="16">
      <n-grid-item span="m:3">
        <n-input-group>
          <n-input
            @change="handleSearch"
            placeholder="Pesquisar"
            autosize
            :style="{ minWidth: '50%', width: '33%' }"
            clearable
          />
          <n-select
            v-model:value="columnFilter"
            filterable
            placeholder="Coluna"
            :options="columnFilterOptions"
            :style="{ width: '33%' }"
            clearable
          />
          <n-select
            v-model:value="yearFilter"
            filterable
            placeholder="Ano"
            :options="yearFilterOptions"
            :style="{ width: '33%' }"
            clearable
          />
        </n-input-group>
      </n-grid-item>
      <n-grid-item>
        <n-button type="success" ghost @click="createBatch">
          <template #icon>
            <n-icon>
              <PlusOutlined />
            </n-icon>
          </template>
          Criar Lote
        </n-button>
      </n-grid-item>
    </n-grid>

    <!-- Tabela -->
    <div class="table-wrapper">
      <n-data-table
        :columns="columns"
        :data="filteredData"
        :pagination="pagination"
        @update:checked-row-keys="handleCheck"
        @update:sorter="handleUpdateSorter"
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
  NTag,
  NIcon,
  NSelect,
  NInputGroup,
  NTooltip,
  NDropdown
} from 'naive-ui';
import type { DataTableRowKey } from 'naive-ui'
import { RowData, TableColumn } from 'naive-ui/es/data-table/src/interface';
import { ref, computed, h, Ref, reactive, watch, Component } from 'vue';
import { AutoAwesomeMosaicOutlined, EditOutlined, DeleteOutlined, PlusOutlined, MoreVertOutlined } from '@vicons/material'
import AppBacthDetails from '@/components/AppBacthDetails.vue';
import { useBatchesStore } from '@/stores/batchesStore';
import { storeToRefs } from 'pinia';

type DataTableBatch = {
  key: number;
  number: number;
  year: number;
  expireDate: string;
  seed: string;
  coating: string;
  sackBrand: string;
  sackQuantity: number;
  sackWeight: number;
  availableQuantity: number;
  purenessScore: number;
  totalPP: number;
  status: string;
  deletedAt: Date | null;
  _searchIndex: string;
};

type Sorter = {
  columnKey: string;
  order: 'ascend' | 'descend' | false;
};

const isModalOpen = ref<boolean>(false);

const batchesStore = useBatchesStore();
const { batches, batchesForDownload } = storeToRefs(batchesStore);

const selectedBatch = ref<any>(null);
const search = ref('');
const sortStates = ref<Sorter[]>([]);

const columns = ref<TableColumn<RowData>[]>([]);
const columnFilter = ref(null);
const columnFilterOptions = ref<{label: string, value: string}[]>([]);
const yearFilter = ref(null);
const yearFilterOptions = ref<{label: string, value: string}[]>([]);

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

const sortKeyMapOrder = computed<Record<string, 'ascend' | 'descend' | false>>(() =>
  sortStates.value.reduce<Record<string, 'ascend' | 'descend' | false>>((result, { columnKey, order }) => {
    result[columnKey] = order || false
    return result
  }, {})
)

const filteredData: Ref<RowData[]> = computed(() => {
  const term = normalizeText(search.value);
  const column = columnFilter.value || 'all';
  const year = yearFilter.value || 'all';

  let filteredBatches = batches.value.filter(batch => {
    const matchesYear = year === 'all' || batch.year.toString() === year;

    if (!matchesYear) return false;

    if (column === 'all') {
      return !term || batch._searchIndex.includes(term);
    } else {
      const key = column as keyof DataTableBatch;
      const value = batch[key];
      const valueStr = value == null ? '' : String(value);
      const matchesColumn = !term || normalizeText(valueStr).includes(term);

      return matchesColumn;
    }
  });

  if (sortStates.value.length > 0) {
    const { columnKey, order } = sortStates.value[0];

    filteredBatches.sort((a: any, b: any) => {
      const aVal = a[columnKey];
      const bVal = b[columnKey];

      if (aVal == null) return -1;
      if (bVal == null) return 1;

      if (typeof aVal === 'number' && typeof bVal === 'number') {
        return order === 'ascend' ? aVal - bVal : bVal - aVal;
      }

      const aStr = String(aVal).toLowerCase();
      const bStr = String(bVal).toLowerCase();
      return order === 'ascend'
        ? aStr.localeCompare(bStr)
        : bStr.localeCompare(aStr);
    });
  }

  const start = (pagination.page - 1) * pagination.pageSize;
  return filteredBatches.slice(start, start + pagination.pageSize);
});

watch(batches.value, async () => {
  createColumns();
  await setColumnFilterOptions();
  await setYearFilterOptions();
})

function createBatch() {
  console.log('Criar novo lote');
  // Implementar lógica de criação de lote
}

function handleCheck(rowKeys: DataTableRowKey[]) {
  batchesForDownload.value = rowKeys
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
  isModalOpen.value = true;
}

function handleEdit(batch: any) {
  console.log('Editar', batch);
  // Implementar redirecionamento ou modal de edição
}

function handleDelete(batch: any) {
  console.log('Deletar', batch);
  // Implementar redirecionamento ou modal de remoção
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
  const uniqueYears = Array.from(new Set(batches.value
    .map(batch => batch.year)
    .filter(year => year != null)));

  yearFilterOptions.value = [
    ...uniqueYears.sort().map(year => ({
      label: String(year),
      value: String(year)
    }))
  ];
}

// Normalização de texto para busca
function normalizeText(text: string | null): string {
  return text
    ? text.trim().toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, '')
    : '';
}

function getStatusLabel(status: string) {
  switch (status) {
    case 'active':
      return 'Em uso'
    case 'closed':
      return 'Encerrado'
    default:
      return 'erro'
  }
}

function getStatusType(status: string) {
  switch (status) {
    case 'active':
      return 'success'
    case 'closed':
      return 'warning'
    default:
      return 'error'
  }
}

function parseExpireDate(dateStr: string): Date | null {
  if (!dateStr) return null;

  const parts = dateStr.toLowerCase().split('/');
  if (parts.length !== 2) return null;

  const month = monthMap[parts[0]];
  const year = Number(parts[1]);

  if (month === undefined || isNaN(year)) return null;

  return new Date(year, month, 1);
}

const monthMap: Record<string, number> = {
  jan: 0,
  fev: 1,
  mar: 2,
  abr: 3,
  mai: 4,
  jun: 5,
  jul: 6,
  ago: 7,
  set: 8,
  out: 9,
  nov: 10,
  dez: 11,
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
      title: 'Número', 
      key: 'number',
      sortOrder: sortKeyMapOrder.value['number'] || false,
      sorter: {
        compare: (a, b) => a.number - b.number,
        multiple: 1
      }
    },
    { 
      title: 'Ano', 
      key: 'year',
      sortOrder: sortKeyMapOrder.value['year'] || false,
      sorter: {
        compare: (a, b) => a.year - b.year,
        multiple: 1
      }
    },
    {
      title: 'Vencimento',
      key: 'expireDate',
      sortOrder: sortKeyMapOrder.value['expireDate'] || false,
      sorter: {
        compare: (a, b) => {
          const dateA = parseExpireDate(a.expireDate);
          const dateB = parseExpireDate(b.expireDate);

          if (!dateA && !dateB) return 0;
          if (!dateA) return -1;
          if (!dateB) return 1;

          return dateA.getTime() - dateB.getTime();
        },
        multiple: 1
      }
    },
    { title: 'Cultivar', key: 'seed' },
    { title: 'Revestimento', key: 'coating' },
    {
      title: 'Sacaria',
      key: 'sack',
      titleAlign: 'center',
      align: 'center',
      children: [
        { title: 'Marca', key: 'sackBrand' },
        { 
          title: 'Quantidade', 
          key: 'sackQuantity',
          titleAlign: 'center',
          align: 'center',
          sortOrder: sortKeyMapOrder.value['sackQuantity'] || false,
          sorter: {
            compare: (a, b) => a.sackQuantity - b.sackQuantity,
            multiple: 1
          }
        },
        { 
          title: 'Peso', 
          key: 'sackWeight',
          titleAlign: 'center',
          align: 'center',
          sortOrder: sortKeyMapOrder.value['sackWeight'] || false,
          sorter: {
            compare: (a, b) => a.sackWeight - b.sackWeight,
            multiple: 1
          }
        }
      ]
    },
    { 
      title: 'Quantidade (kg)', 
      key: 'availableQuantity',
      titleAlign: 'center',
      align: 'center',
      sortOrder: sortKeyMapOrder.value['availableQuantity'] || false,
      sorter: {
        compare: (a, b) => a.availableQuantity - b.availableQuantity,
        multiple: 1
      }
    },
    { 
      title: 'PP/Kg', 
      key: 'purenessScore',
      sortOrder: sortKeyMapOrder.value['purenessScore'] || false,
      sorter: {
        compare: (a, b) => a.purenessScore - b.purenessScore,
        multiple: 1
      }
    },
    { 
      title: 'Total PP', 
      key: 'totalPP',
      sortOrder: sortKeyMapOrder.value['totalPP'] || false,
      sorter: {
        compare: (a, b) => a.totalPP - b.totalPP,
        multiple: 1
      }
    },
    { 
      title: 'Status', 
      key: 'status',
      titleAlign: 'center',
      align: 'center',
      render(row) {
        return h(
          NTag,
          {
            type: getStatusType(row.status),
            bordered: false
          },
          { default: () => getStatusLabel(row.status) }
        )
      }
    },
    {
      title: 'Ações',
      key: 'actions',
      titleAlign: 'center',
      align: 'center',
      width: '100px',
      render(batch: any) {
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
            NDropdown,
            {
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
                  }, { default: () => h(DeleteOutlined) })
                }
              ],
              onSelect: (key: string) => {
                if (key === 'edit') {
                  handleEdit(batch)
                } else if (key === 'delete') {
                  handleDelete(batch)
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
</style>

