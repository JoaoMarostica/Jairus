<template>
  <n-modal
    v-model:show="batchDetailsModal"
    :style="{
      top: '0px',
      left: '0px',
      margin: '0',
      padding: '0',
      width: '100vw',
      height: '100vh',
      maxHeight: '100vh'
    }"
    :mask-closable="false"
    preset="card"
    :closable="true"
    v-on:update-show="closeModal"
    :title="modalTitle"
    size="huge"
    class="!w-screen !h-screen"
  >
    <div style="width: 100%; height: 100%; overflow: auto;" class="bg-white dark:bg-black p-4">
      <n-grid cols="1" responsive="screen" x-gap="16" y-gap="16">
        <!-- Dados do lote -->
        <n-grid-item>
          <n-card class="border border-gray-200">
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

        <!-- Gráficos -->
        <n-grid-item>
          <n-grid cols="1 m:2" responsive="screen" x-gap="16" y-gap="16" class="mt-6">
            <n-grid-item>
              <n-card title="Saídas" style="height: 300px;">
                <div ref="outflowChart" style="height: 250px;" />
              </n-card>
            </n-grid-item>
            <n-grid-item>
              <n-card title="Saldo" style="height: 300px;">
                <div ref="balanceChart" style="height: 250px;" />
              </n-card>
            </n-grid-item>
          </n-grid>
        </n-grid-item>

        <!-- Tabela de saídas -->
        <n-grid-item>
          <n-card v-if="outflowData.length === 0">
            <n-empty description="Nenhuma Saída Encontrado" size="large">
            </n-empty>
          </n-card>
          <n-card title="Saídas do lote" v-else>
            <n-data-table :columns="outflowColumns" :data="outflowData" :pagination="false" :max-height="250" />
          </n-card>
        </n-grid-item>
      </n-grid>
    </div>
  </n-modal>

  <!-- Outflow CRUD -->
  <AppEditOutflow v-model:modal="editOutflowModal" :selectedOutflow="selectedOutflow" @close="batchDetailsModal = true" />
  <AppRemoveOutflow v-model:modal="removeOutflowModal" :selectedOutflow="selectedOutflow" @close="batchDetailsModal = true" />
</template>

<script setup lang="ts">
import { NModal, NCard, NGrid, NGridItem, NDataTable, NDescriptions, NDescriptionsItem, NEmpty, NButton, NIcon, NDropdown } from 'naive-ui'
import type { DataTableBatchOutflow } from '@/types/batches';
import { RowData, TableColumn } from 'naive-ui/es/data-table/src/interface';
import { computed, h, nextTick, ref, watch, watchEffect } from 'vue'
import * as echarts from 'echarts'
import { useBatchesStore } from '@/stores/batchesStore';
import { useGlobalStore } from '@/stores/globalStore';
import { storeToRefs } from 'pinia';
import { DeleteOutlined, EditOutlined, MoreVertOutlined } from '@vicons/material';
import AppEditOutflow from '@/components/AppEditOutflow.vue';
import AppRemoveOutflow from '@/components/AppRemoveOutflow.vue';

type BatchBalance = {value: number; name: string};

const globalStore = useGlobalStore();
const { theme } = storeToRefs(globalStore);

const batchesStore = useBatchesStore();

const props = defineProps<{
  selectedBatch: any
}>()

const batchDetailsModal = defineModel('modal', {
  type: Boolean,
  default: false
})

const selectedBatch = computed(() => props.selectedBatch)
const modalTitle = ref(`Detalhes do Lote ${props.selectedBatch}`)
const selectedOutflow = ref<any>(null);

const editOutflowModal = ref<boolean>(false);
const removeOutflowModal = ref<boolean>(false);

const batchData = ref<{ titulo: string; valor: any; unidade: string }[]>([])
const outflowChart = ref<HTMLElement | null>(null)
const balanceChart = ref<HTMLElement | null>(null)
const outflowData = ref<DataTableBatchOutflow[]>([])
const outflowColumns = ref<TableColumn<RowData>[]>([]);
const batchBalance = ref<BatchBalance[]>([])

watch(batchDetailsModal, async () => {
  if (batchDetailsModal) {
    await nextTick()
    createColumns();
    modalTitle.value = `Detalhes do Lote ${selectedBatch.value.batch_number}`
    await getbatchData()
    renderCharts()
  }
})

watchEffect(async () => {
  if (batchDetailsModal.value) {
    outflowData.value = await batchesStore.getBatchOutflows(selectedBatch.value.batch_number, selectedBatch.value.batch_year)
    batchBalance.value = batchesStore.getBatchBalance(selectedBatch.value, outflowData.value)
  }
})

function renderCharts() {
  const isDark = theme.value === 'dark'

  const axisLabelColor = isDark ? '#ccc' : '#333'
  const axisLineColor = isDark ? '#888' : '#999'
  const splitLineColor = isDark ? '#555' : '#e0e0e0'
  const legendTextColor = isDark ? '#ccc' : '#333'

  // Gráfico de barra (saídas)
  if (outflowChart.value && selectedBatch.value) {
    // Evita instância duplicada
    const existing = echarts.getInstanceByDom(outflowChart.value)
    if (existing) existing.dispose()

    const chart = echarts.init(outflowChart.value)

    chart.setOption({
      xAxis: {
        type: 'category',
        data: ['Jan', 'Fev', 'Mar', 'Abr'],
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
        { name: 'Entrada', type: 'bar', data: [300, 400, 500, 350] },
        { name: 'Saída', type: 'bar', data: [200, 250, 300, 270] }
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

    if (outflowChart.value) {
      const chart = echarts.getInstanceByDom(outflowChart.value)
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

async function getbatchData() {
  try {
    outflowData.value = await batchesStore.getBatchOutflows(selectedBatch.value.batch_number, selectedBatch.value.batch_year)

    // globalStore.showMessage({
    //   content: 'Pedidos carregados com sucesso!',
    //   type: 'success',
    // })
  } catch (error: any) {
    globalStore.showMessage({
      content: `Erro ao carregar pedidos: ${error?.message || error}`,
      type: 'error',
      keepAliveOnHover: true,
    })
  }

  batchBalance.value = batchesStore.getBatchBalance(selectedBatch.value, outflowData.value)
  // try {

  //   // globalStore.showMessage({
  //   //   content: 'Saldo carregado com sucesso!',
  //   //   type: 'success',
  //   // })
  // } catch (error: any) {
  //   globalStore.showMessage({
  //     content: `Erro ao carregar saldo: ${error?.message || error}`,
  //     type: 'error',
  //     keepAliveOnHover: true,
  //   })
  // }

  batchData.value = [
    { titulo: 'Chave', valor: selectedBatch.value.key, unidade: '' },
    { titulo: 'Ano', valor: selectedBatch.value.batch_year, unidade: '' },
    { titulo: 'Vencimento', valor: selectedBatch.value.expire_date, unidade: '' },
    { titulo: 'Cultivar', valor: selectedBatch.value.seed, unidade: '' },
    { titulo: 'Tratamento', valor: selectedBatch.value.coating, unidade: '' },
    { titulo: 'Marca da Sacaria', valor: selectedBatch.value.brand, unidade: '' },
    { titulo: 'Sacos', valor: selectedBatch.value.sack_amount, unidade: '' },
    { titulo: 'Peso da Sacaria', valor: selectedBatch.value.sack_weight, unidade: 'Kg' },
    { titulo: 'Quantidade (Kg)', valor: selectedBatch.value.total_weight, unidade: 'kg' },
    { titulo: 'PP/Kg', valor: selectedBatch.value.pureness_score, unidade: '' },
    { titulo: 'Total de PP', valor: selectedBatch.value.total_pureness_score, unidade: '' },
  ]
}

// Ações
async function handleEdit(outflow: any) {
  selectedOutflow.value = outflow;
  batchDetailsModal.value = false;
  await nextTick();
  editOutflowModal.value = true;
}

async function handleRemove(outflow: any) {
  selectedOutflow.value = outflow;
  batchDetailsModal.value = false;
  await nextTick();
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
