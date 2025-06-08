<template>
  <n-modal
    v-model:show="batchDetailsModal"
    preset="card"
    :closable="true"
    v-on:update-show="closeModal"
    :title="modalTitle"
    :style="{
      width: '55vw',
      overflow: 'auto'
    }"
  >
    <n-grid cols="1" x-gap="16" y-gap="16" responsive="screen">
      <!-- Batch info -->
      <n-grid-item>
        <n-card title="Informações do Lote" class="border border-gray-200">
          <n-descriptions label-placement="top" :column="11" size="small">
            <n-descriptions-item
              v-for="(indicator, index) in batchData"
              :key="index"
              :label="indicator.titulo"
            >
              {{ indicator.valor }} {{ indicator.unidade }}
            </n-descriptions-item>
          </n-descriptions>
        </n-card>
      </n-grid-item>
      
      <!-- Statistics card -->
      <n-grid-item>
        <n-space vertical :size="16">
          <n-card :title="statisticTitle">
            <n-space justify="space-between" align="center" size="large">
              <n-statistic label="Lote">{{ batchStatistic }}</n-statistic>
              <n-icon size="22"><MinusOutlined /></n-icon>
              <n-statistic label="Pedidos">{{ outflowStatistic }}</n-statistic>
              <n-icon size="22"><EqualsOutlined /></n-icon>
              <n-statistic label="Saldo">{{ balanceStatistic }}</n-statistic>
            </n-space>
          </n-card>
        </n-space>
      </n-grid-item>

      <!-- Statistics chart -->
      <n-grid-item>
        <n-card title="Gráfico de Totais">
          <n-space justify="space-between" align="center">
            <n-select
              v-model:value="statisticMode"
              :options="statisticModeOptions"
              size="small"
              style="width: 180px"
            />
          </n-space>
          <div ref="totalChart" style="height: 250px;" />
        </n-card>
      </n-grid-item>
        
      <!-- Outflow table -->
      <n-grid-item span="1">
        <n-card v-if="outflowData.length === 0">
          <n-empty description="Nenhum Pedido Encontrado" size="large" />
        </n-card>
        <n-card v-else title="Pedidos do Lote">
          <n-data-table
            :columns="outflowColumns"
            :data="outflowData"
            :pagination="false"
            :max-height="650"
          />
        </n-card>
      </n-grid-item>
    </n-grid>

    <!-- Outflow CRUD modals -->
    <AppEditOutflow
      v-model:modal="editOutflowModal"
      :selectedBatch="selectedBatch"
      :selectedOutflow="selectedOutflow"
      @reloadData="reloadData"
    />
    <AppRemoveOutflow
      v-model:modal="removeOutflowModal"
      :selectedOutflow="selectedOutflow"
      @reloadData="reloadData"
    />
  </n-modal>
</template>


<script setup lang="ts">
import { NModal, NCard, NGrid, NGridItem, NDataTable, NDescriptions, NDescriptionsItem, NEmpty, NButton, NIcon, NDropdown, NStatistic, NSelect, NSpace } from 'naive-ui'
import type { DataTableBatchOutflow } from '@/types/batches';
import { RowData, TableColumn } from 'naive-ui/es/data-table/src/interface';
import { computed, h, nextTick, ref, watch } from 'vue'
import * as echarts from 'echarts'
import { parsePtBrNumber, formatNumber, parseNumber } from '@/utils/parsing';
import { useOutflowsStore } from '@/stores/outflowsStore';
import { useBalancesStore } from '@/stores/balancesStore';
import { useGlobalStore } from '@/stores/globalStore';
import { storeToRefs } from 'pinia';
import { DeleteOutlined, EditOutlined, MoreVertOutlined, MinusOutlined, EqualsOutlined } from '@vicons/material';
import AppEditOutflow from '@/components/outflow/AppEditOutflow.vue';
import AppRemoveOutflow from '@/components/outflow/AppRemoveOutflow.vue';
import { BalanceDB } from '@/types/balance';

type BatchBalance = {value: number; name: string};

const globalStore = useGlobalStore();
const { theme } = storeToRefs(globalStore);

const outflowsStore = useOutflowsStore();
const balancesStore = useBalancesStore();

const props = defineProps<{
  selectedBatch: any
}>();

const batchDetailsModal = defineModel('modal', {
  type: Boolean,
  default: false
});

const selectedBatch = computed(() => props.selectedBatch);
const modalTitle = ref('')
const selectedOutflow = ref<any>(null);

const editOutflowModal = ref<boolean>(false);
const removeOutflowModal = ref<boolean>(false);

const batchData = ref<{ titulo: string; valor: any; unidade: string }[]>([]);
const batchBalance = ref<BatchBalance[]>([]);
const balanceChart = ref<HTMLElement | null>(null);

const totalChart = ref<HTMLElement | null>(null);
const outflowData = ref<DataTableBatchOutflow[]>([]);
const outflowColumns = ref<TableColumn<RowData>[]>([]);
const outflowTotals = ref<BalanceDB | null>(null);

const statisticMode = ref('weight')
const statisticTitle = computed(() => {
  if (statisticMode.value === 'weight') {
    return 'Quantidade (Kg)'
  } else if (statisticMode.value === 'totalPP') {
    return 'Total PP'
  }
  return 'Sacos'
})
const statisticModeOptions = ref([
  { label: 'quantidade (Kg)', value: 'weight' },
  { label: 'Total PP', value: 'totalPP' },
  { label: 'Sacos', value: 'sackAmount' }
])

const batchStatistic = computed(() => getStatistic(selectedBatch.value, statisticMode.value))
const outflowStatistic = computed(() => getStatistic(outflowTotals.value, statisticMode.value))
const balanceStatistic = computed(() => {
  if (!batchBalance.value.length) return '0'
  return {
    totalPP: formatNumber(batchBalance.value[0].value),
    weight: formatNumber(batchBalance.value[1].value),
    sackAmount: batchBalance.value[2].value.toString(),
  }[statisticMode.value] ?? '0'
})

function getStatistic(source: any, mode: string) {
  if (mode === 'weight') {
    return formatStatistic(source?.total_weight)
  }
  if (mode === 'totalPP') {
    return formatStatistic(source?.total_pureness_score)
  }
  return formatStatistic(source?.sack_amount)
}

function formatStatistic(value: any) {
  // Se já está formatado em pt-BR (ex: '1.234,56') retorna como está
  if (typeof value === 'string') {
    return value
  }

  // Se for número retorna formata em pt-BR
  if (typeof value === 'number') {
    return formatNumber(value)
  }

  return '0'
}

watch(batchDetailsModal, async (val) => {
  if (!val) return;
  await nextTick();
  createColumns();
  modalTitle.value = `Lote ${selectedBatch.value.batch_number}/${selectedBatch.value.batch_year.toString()}`;
  getbatchData();
  await getbatchOutflowData();
  await getbatchBalanceData();
  renderCharts();
});

async function reloadData() {
  await getbatchOutflowData();
  await getbatchBalanceData();
  renderCharts();
}

watch(statisticMode, () => {
  renderCharts();
});

function renderCharts() {
  const isDark = theme.value === 'dark'

  const axisLabelColor = isDark ? '#ccc' : '#333'
  const axisLineColor = isDark ? '#888' : '#999'
  const splitLineColor = isDark ? '#555' : '#e0e0e0'
  const legendTextColor = isDark ? '#ccc' : '#333'

  // Gráfico de barra (totais)
  if (totalChart.value && selectedBatch.value) {
    const existing = echarts.getInstanceByDom(totalChart.value)
    if (existing) existing.dispose()

    const chart = echarts.init(totalChart.value)

    chart.setOption({
      xAxis: {
        type: 'category',
        data: [statisticTitle.value],
        axisLabel: {
          color: axisLabelColor,
          fontSize: 12
        },
        axisLine: {
          lineStyle: {
            color: axisLineColor
          }
        }
      },
      yAxis: {
        type: 'value',
        axisLabel: {
          color: axisLabelColor,
          fontSize: 12
        },
        axisLine: {
          lineStyle: {
            color: axisLineColor
          }
        },
        splitLine: {
          lineStyle: {
            color: splitLineColor
          }
        }
      },
      tooltip: {
        trigger: 'axis'
      },
      series: [
        {
          name: 'Lote',
          type: 'bar',
          data: [parseNumber(batchStatistic.value)],
          itemStyle: { color: '#4CAF50' } // verde
        },
        {
          name: 'Pedidos',
          type: 'bar',
          data: [parseNumber(outflowStatistic.value)],
          itemStyle: { color: '#F44336' } // vermelho
        },
        {
          name: 'Saldo',
          type: 'bar',
          data: [parseNumber(balanceStatistic.value)],
          itemStyle: { color: '#2196F3' } // azul
        }
      ],
      legend: {
        textStyle: {
          color: legendTextColor
        }
      }
    })
  }

  // Gráfico de pizza (saldo)
  if (balanceChart.value && batchBalance.value) {
    const existing = echarts.getInstanceByDom(balanceChart.value)
    if (existing) existing.dispose()

    const chart = echarts.init(balanceChart.value)

    chart.setOption({
      tooltip: {
        trigger: 'item',
        formatter: '{b}: {d}%'
      },
      series: [
        {
          name: 'Saldo',
          type: 'pie',
          radius: '50%',
          data: batchBalance.value,
          label: {
            show: true,
            formatter: '{c}',
            fontSize: 14,
            color: axisLabelColor
          },
        }
      ],
      legend: {
        textStyle: {
          color: legendTextColor
        },
      }
    })
  }
}

function closeModal(model: boolean) {
  if (!model) {
    modalTitle.value = ''
    batchData.value = []

    if (totalChart.value) {
      const chart = echarts.getInstanceByDom(totalChart.value)
      if (chart) {
        chart.dispose()
      }
    }

    if (balanceChart.value) {
      const chart = echarts.getInstanceByDom(balanceChart.value)
      if (chart) {
        chart.dispose()
      }
    }
  }
}

async function getbatchOutflowData() {
  try {
    outflowTotals.value = await outflowsStore.getOutflowTotals(selectedBatch.value)
    outflowData.value = await outflowsStore.getBatchOutflows(selectedBatch.value.batch_number, selectedBatch.value.batch_year)
  } catch (error: any) {
    console.error(error);
    globalStore.showMessage({
      content: 'Erro ao carregar pedidos do lote.',
      type: 'error',
    })
  }
}

async function getbatchBalanceData() {
  try {
    const balance = await balancesStore.getBatchBalance(selectedBatch.value)

    batchBalance.value = [
      { value: parsePtBrNumber(balance.total_pureness_score), name: 'Total PP' },
      { value: parsePtBrNumber(balance.total_weight), name: 'Quantidade (Kg)' },
      { value: balance.sack_amount, name: 'Sacos' }
    ];
  } catch (error: any) {
    console.error(error);
    globalStore.showMessage({
      content: 'Erro ao carregar saldo do lote.',
      type: 'error',
    })
  }
}

function getbatchData() {
  batchData.value = [
    // { titulo: 'Ano', valor: selectedBatch.value.batch_year, unidade: '' },
    { titulo: 'Vencimento', valor: selectedBatch.value.expire_date, unidade: '' },
    { titulo: 'Cultivar', valor: selectedBatch.value.seed, unidade: '' },
    { titulo: 'Tratamento', valor: selectedBatch.value.coating, unidade: '' },
    { titulo: 'Marca da Sacaria', valor: selectedBatch.value.brand, unidade: '' },
    // { titulo: 'Sacos', valor: selectedBatch.value.sack_amount, unidade: '' },
    { titulo: 'Peso da Sacaria', valor: selectedBatch.value.sack_weight, unidade: 'Kg' },
    // { titulo: 'Quantidade (Kg)', valor: selectedBatch.value.total_weight, unidade: 'kg' },
    { titulo: 'PP/Kg', valor: selectedBatch.value.pureness_score, unidade: '' },
    // { titulo: 'Total de PP', valor: selectedBatch.value.total_pureness_score, unidade: '' },
  ]
}

// Ações
async function handleEdit(outflow: any) {
  selectedOutflow.value = outflow;
  editOutflowModal.value = true;
}

async function handleRemove(outflow: any) {
  selectedOutflow.value = outflow;
  removeOutflowModal.value = true;
}

function createColumns() {
  outflowColumns.value = [
    { title: 'Total de PP', key: 'total_pureness_score' },
    { title: 'Quantidade (kg)', key: 'total_weight', titleAlign: 'center', align: 'center' },
    { title: 'PP/Kg', key: 'pureness_score' },
    { title: 'Sacos', key: 'sack_amount' },
    { title: 'Pedido', key: 'usage' },
    {
      title: 'Ações',
      key: 'actions',
      titleAlign: 'center',
      align: 'center',
      width: '150px',
      render(outflow: RowData): ReturnType<typeof h>[]  {
        return [
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
                  }, { default: () => h(DeleteOutlined) })
                }
              ],
              onSelect: (key: string) => {
                if (key === 'edit') {
                  handleEdit(outflow)
                } else if (key === 'delete') {
                  handleRemove(outflow)
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
