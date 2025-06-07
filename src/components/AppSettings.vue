<template>
    <n-modal v-model:show="settingsModal" preset="card" draggable title="Configurações" style="width: 600px; min-height: 600px">
        <n-tabs type="segment" animated>
            <n-tab-pane name="seeds" tab="Cultivares">
                
                <!-- Data Table Seeds-->
                <n-data-table
                    :columns="seedColumns"
                    :data="dataTableSeeds"
                    :max-height="300"
                    
                />
                
                <div style="text-align: center; margin-top: 24px;">
                    <n-button strong secondary type="info" @click="showCreateSeed = true">
                        
                        <template #icon>
                            <n-icon>
                                <PlusOutlined />
                            </n-icon>
                        </template>
                        Nova Cultivar
                    </n-button>
                    <AppCreateSeed v-model:show="showCreateSeed" />
                    <AppEditSeed v-model:show="showEditSeed" :seed="selectedSeed" />
                </div>
            </n-tab-pane>

            <n-tab-pane name="coatings" tab="Tratamentos">
                <!-- Data Table Coatings-->
                <n-data-table
                    :columns="coatingColumns"
                    :data="dataTableCoatings"
                    :max-height="400"
                    
                />
                
                <div style="text-align: center; margin-top: 24px;">
                    <n-button strong secondary type="info" @click="showCreateCoating = true">
                        <template #icon>
                            <n-icon>
                                <PlusOutlined />
                            </n-icon>
                        </template>
                        Novo Tratamento
                    </n-button>
                    
                    <!-- Componentes de modais -->
                    <AppCreateCoating v-model:show="showCreateCoating" />
                    <AppEditCoating v-model:show="showEditCoating" :coating="selectedCoating" />
                </div>
            </n-tab-pane>

            <n-tab-pane name="brands" tab="Marcas">
                <!-- Data Table Brands-->
                <n-data-table
                    :columns="brandColumns"
                    :data="dataTableBrands"
                    :max-height="400"
                    
                />
            
                <div style="text-align: center; margin-top: 24px;">
                    <n-button strong secondary type="info" @click="showCreateBrand = true">
                        <template #icon>
                            <n-icon>
                                <PlusOutlined />
                            </n-icon>
                        </template>
                        Nova Marca
                    </n-button>
                    <AppCreateBrand v-model:show="showCreateBrand" />
                    <AppEditBrand v-model:show="showEditBrand" :brand="selectedBrand" />
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
import { ref, watch, h} from 'vue'
import { useSettingsStore } from '@/stores/settingsStore'
import { useGlobalStore } from '@/stores/globalStore';
import { useSeedsStore } from '@/stores/seedsStore';
import { useCoatingsStore } from '@/stores/coatingsStore';
import { useBrandsStore } from '@/stores/brandsStore';
import AppCreateBrand from '@/components/brand/AppCreateBrand.vue'
import AppCreateSeed from '@/components/seed/AppCreateSeed.vue'
import AppCreateCoating from '@/components/coating/AppCreateCoating.vue'
import AppEditSeed from '@/components/seed/AppEditSeed.vue'
import AppEditBrand from '@/components/brand/AppEditBrand.vue'
import AppEditCoating from '@/components/coating/AppEditCoating.vue'
import { EditOutlined, DeleteOutlined, PlusOutlined } from '@vicons/material'
import type { DataTableSeed } from '@/types/seeds'
import type { DataTableCoating } from '@/types/coatings'
import { storeToRefs } from 'pinia';
import { RowData, TableColumn } from 'naive-ui/es/data-table/src/interface';
import {
  NModal,
  NTabs,
  NTabPane,
  NButton,
  NDataTable,
  NSpace,
  NIcon,
  useDialog,
} from 'naive-ui'
import { DataTableBrand } from '@/types/brands';

const settingsStore = useSettingsStore()
const seedsStore = useSeedsStore()
const coatingsStore = useCoatingsStore()
const brandsStore = useBrandsStore()
const globalStore = useGlobalStore()
const dialog = useDialog() 

const { settingsModal } = storeToRefs(settingsStore)
const { dataTableSeeds } = storeToRefs(seedsStore)
const { dataTableCoatings } = storeToRefs(coatingsStore)
const { dataTableBrands } = storeToRefs(brandsStore)

const seedColumns = ref<TableColumn<RowData>[]>([]);
const coatingColumns = ref<TableColumn<RowData>[]>([]);
const brandColumns = ref<TableColumn<RowData>[]>([]);

const showCreateBrand = ref(false);
const showCreateSeed = ref(false);
const showEditSeed = ref(false);
const showCreateCoating = ref(false);
const showEditCoating = ref(false); // ← Novo ref
const selectedSeed = ref<DataTableSeed | null>(null);
const selectedCoating = ref<DataTableCoating | null>(null); // ← Novo ref
const showEditBrand = ref(false);
const selectedBrand = ref<DataTableBrand | null>(null);
    
watch(settingsModal, async () => {
  createSeedColumns();
  createCoatingColumns();
  createBrandColumns();
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
    { title: 'Nome Científico', key: 'scientific_name', width: 300 },
    { title: 'Nome Popular', key: 'popular_name' },
    {
        title: 'Ações',
        key: 'actions',
        titleAlign: 'center',
        align: 'center',
        width: '100px',
        render(row) {
            return [
                h(
                    NButton,
                    {
                    quaternary: true,
                    size: 'small',
                    renderIcon: () => h(EditOutlined),
                        onClick: () => editSeedHandler(row as DataTableSeed) 
                    }
                ),
                h(
                    NButton,
                    {
                        quaternary: true,
                        size: 'small',
                        type: 'error',
                        renderIcon: () => h(DeleteOutlined),
                        onClick: () => deleteSeedHandler(row) 
                    }
                )
            ];
        }
    }
  ]
}


async function deleteSeedHandler(seed: any) {
    dialog.warning({
        title: 'Confirmar Exclusão',
        content: () => h('div', [
            h('p', { style: 'margin-bottom: 12px;' }, 
                'Tem certeza que deseja excluir a cultivar:'
            ),
            h('p', { style: 'font-weight: bold; color: #d03050; margin: 12px 0; font-size: 16px;' }, 
                `"${seed.popular_name}"`
            ),
            h('p', { style: 'color: #666; font-size: 14px; margin-top: 16px;' }, 
                'Esta ação não pode ser desfeita.'
            )
        ]),
        positiveText: 'Excluir',
        negativeText: 'Cancelar',
        positiveButtonProps: {
            type: 'error'
        },
        onPositiveClick: async () => {
            try {
                await seedsStore.removeSeed(seed);
                
                globalStore.showMessage({
                    content: `Cultivar "${seed.popular_name}" excluída com sucesso!`,
                    type: 'success'
                });
            } catch (error: any) {
                globalStore.showMessage({
                    content: `Erro ao excluir cultivar: ${error?.message || error}`,
                    type: 'error'
                });
            }
        }
    });
}

// editar 
function editSeedHandler(seed: DataTableSeed) {
    selectedSeed.value = seed;
    showEditSeed.value = true;
}

function createCoatingColumns() {
  coatingColumns.value = [
    { 
        title: 'Tipo', 
        key: 'coating_name',
    },
    {
        title: 'Ações',
        key: 'actions',
        titleAlign: 'center',
        align: 'center',
        width: '100px',
        render(row) { 
            return [
                h(
                    NButton,
                    {
                        quaternary: true,
                        size: 'small',
                        renderIcon: () => h(EditOutlined),
                        onClick: () => editCoatingHandler(row as DataTableCoating) // ← Conectado ao handler
                    }
                ),
                h(
                    NButton,
                    {
                        quaternary: true,
                        size: 'small',
                        type: 'error',
                        renderIcon: () => h(DeleteOutlined),
                        onClick: () => deleteCoatingHandler(row)
                    }
                )
            ];
        }
    }
  ]
}

function createBrandColumns() {
  brandColumns.value = [
    { title: 'Nome', key: 'brand_name' },
    { title: 'Sacos', key: 'weights', render: (row) =>{
         return row.weights.map((w: {label: string, value: string}) => w.label).join(', ');
        }
        
    },
    {
        title: 'Ações',
        key: 'actions',
        titleAlign: 'center',
        align: 'center',
        width: '100px',
        render(row) {
            return [
                h(
                    NButton,
                    {
                        quaternary: true,
                        size: 'small',
                        renderIcon: () => h(EditOutlined),
                        onClick: () => editBrandHandler(row as DataTableBrand)
                    }
                ),
                h(
                    NButton,
                    {
                    quaternary: true,
                    size: 'small',
                    type: 'error',
                    renderIcon: () => h(DeleteOutlined),
                    onClick: () => deleteBrandHandler(row as DataTableBrand) // ← Conectado ao handler
                }
                )
            ];
        }
    }
  ]
}

// Função para editar tratamento
function editCoatingHandler(coating: DataTableCoating) {
    selectedCoating.value = coating;
    showEditCoating.value = true;
}

async function deleteCoatingHandler(coating: any) {
    dialog.warning({
        title: 'Confirmar Exclusão',
        content: () => h('div', [
            h('p', { style: 'margin-bottom: 12px;' }, 
                'Tem certeza que deseja excluir o Tratamento:'
            ),
            h('p', { style: 'font-weight: bold; color: #d03050; margin: 12px 0; font-size: 16px;' }, 
                `"${coating.coating_name}"`
            ),
            h('p', { style: 'color: #666; font-size: 14px; margin-top: 16px;' }, 
                'Esta ação não pode ser desfeita.'
            )
        ]),
        positiveText: 'Excluir',
        negativeText: 'Cancelar',
        positiveButtonProps: {
            type: 'error'
        },
        onPositiveClick: async () => {
            try {
                await coatingsStore.removeCoating(coating);
                
                globalStore.showMessage({
                    content: `Tratamento "${coating.coating_name}" excluído com sucesso!`,
                    type: 'success'
                });
            } catch (error: any) {
                globalStore.showMessage({
                    content: `Erro ao excluir Tratamento: ${error?.message || error}`,
                    type: 'error'
                });
            }
        }
    });
}

// Função para editar marca
function editBrandHandler(brand: DataTableBrand) {
    selectedBrand.value = brand;
    showEditBrand.value = true;
}

async function deleteBrandHandler(brand: DataTableBrand) {
    dialog.warning({
        title: 'Confirmar Exclusão',
        content: () => h('div', [
            h('p', { style: 'margin-bottom: 12px;' }, 
                'Tem certeza que deseja excluir a marca:'
            ),
            h('p', { style: 'font-weight: bold; color: #d03050; margin: 12px 0; font-size: 16px;' }, 
                `"${brand.brand_name}"`
            ),
            h('p', { style: 'color: #666; font-size: 14px; margin-top: 16px;' }, 
                'Esta ação não pode ser desfeita.'
            )
        ]),
        positiveText: 'Excluir',
        negativeText: 'Cancelar',
        positiveButtonProps: {
            type: 'error'
        },
        onPositiveClick: async () => {
            try {
                await brandsStore.removeBrand(brand);
                
                globalStore.showMessage({
                    content: `Marca "${brand.brand_name}" excluída com sucesso!`,
                    type: 'success'
                });
            } catch (error: any) {
                globalStore.showMessage({
                    content: `Erro ao excluir marca: ${error?.message || error}`,
                    type: 'error'
                });
            }
        }
    });
}
</script>

