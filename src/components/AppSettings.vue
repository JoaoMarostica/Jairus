<template>
    <n-modal v-model:show="settingsModal" preset="card" title="Configurações" style="width: 600px; min-height: 600px">
        <n-tabs type="segment" animated>
            <n-tab-pane name="seeds" tab="Cultivares">
                <n-data-table
                    :columns="seedColumns"
                    :data="seeds"
                />
                <div style="text-align: center; margin-top: 24px;">
                    <n-button strong secondary type="info">
                        <template #icon>
                            <n-icon>
                                <PlusOutlined />
                            </n-icon>
                        </template>
                        Nova Cultivar
                    </n-button>
                </div>
            </n-tab-pane>

            <n-tab-pane name="coatings" tab="Revestimentos">
                <n-data-table
                    :columns="coatingColumns"
                    :data="coatings"
                />
                <div style="text-align: center; margin-top: 24px;">
                    <n-button strong secondary type="info">
                        <template #icon>
                            <n-icon>
                                <PlusOutlined />
                            </n-icon>
                        </template>
                        Novo Revestimento
                    </n-button>
                </div>
            </n-tab-pane>

            <n-tab-pane name="brands" tab="Marcas">
                <n-data-table
                    :columns="brandColumns"
                    :data="brands"
                />
                <div style="text-align: center; margin-top: 24px;">
                    <n-button strong secondary type="info">
                        <template #icon>
                            <n-icon>
                                <PlusOutlined />
                            </n-icon>
                        </template>
                        Nova Marca
                    </n-button>
                </div>
            </n-tab-pane>
        </n-tabs>
  
        <template #footer>
            <n-space justify="space-between" style="width: 100%">
                <n-button @click="handleCancel">Cancelar</n-button>
                <n-button type="primary" @click="handleSave">Salvar</n-button>
            </n-space>
        </template>
    </n-modal>
  </template>

<script setup lang="ts">
import { ref, watch, h } from 'vue'
import { useSettingsStore } from '@/stores/settingsStore'
import { storeToRefs } from 'pinia';
import { RowData, TableColumn } from 'naive-ui/es/data-table/src/interface';
import { EditOutlined, DeleteOutlined, PlusOutlined } from '@vicons/material'
import {
  NModal,
  NTabs,
  NTabPane,
  NButton,
  NDataTable,
  NSpace,
  NIcon,
} from 'naive-ui'

const settingsStore = useSettingsStore()
const { settingsModal, seeds, coatings, brands } = storeToRefs(settingsStore)

const seedColumns = ref<TableColumn<RowData>[]>([]);
const coatingColumns = ref<TableColumn<RowData>[]>([]);
const brandColumns = ref<TableColumn<RowData>[]>([]);

watch(settingsModal, async () => {
  createSeedColumns();
  createCoatingColumns();
  createbrandColumns();
})

function handleSave() {
    settingsModal.value = false
}

// Função para cancelar (fechar o modal sem salvar)
const handleCancel = () => {
    settingsModal.value = false
}

function createSeedColumns() {
  seedColumns.value = [
    { title: 'Nome Científico', key: 'scientificName' },
    { title: 'Nome Popular', key: 'popularName' },
    {
        title: 'Ações',
        key: 'actions',
        titleAlign: 'center',
        align: 'center',
        width: '100px',
        render() {
            return [
                h(
                    NButton,
                    {
                    quaternary: true,
                    size: 'small',
                    renderIcon: () => h(EditOutlined)
                    }
                ),
                h(
                    NButton,
                    {
                    quaternary: true,
                    size: 'small',
                    renderIcon: () => h(DeleteOutlined)
                    }
                )
            ];
        }
    }
  ]
}

function createCoatingColumns() {
  coatingColumns.value = [
    { 
        title: 'Tipo', 
        key: 'name',
    },
    {
        title: 'Ações',
        key: 'actions',
        titleAlign: 'center',
        align: 'center',
        width: '100px',
        render() {
            return [
                h(
                    NButton,
                    {
                    quaternary: true,
                    size: 'small',
                    renderIcon: () => h(EditOutlined)
                    }
                ),
                h(
                    NButton,
                    {
                    quaternary: true,
                    size: 'small',
                    renderIcon: () => h(DeleteOutlined)
                    }
                )
            ];
        }
    }
  ]
}

function createbrandColumns() {
  brandColumns.value = [
    { title: 'Nome', key: 'name' },
    { title: 'Sacos', key: 'sackWeights'},
    {
        title: 'Ações',
        key: 'actions',
        titleAlign: 'center',
        align: 'center',
        width: '100px',
        render() {
            return [
                h(
                    NButton,
                    {
                    quaternary: true,
                    size: 'small',
                    renderIcon: () => h(EditOutlined)
                    }
                ),
                h(
                    NButton,
                    {
                    quaternary: true,
                    size: 'small',
                    renderIcon: () => h(DeleteOutlined)
                    }
                )
            ];
        }
    }
  ]
}

</script>
