<template>
    <n-modal v-model:show="settingsModal" preset="card" title="Configurações" style="width: 600px; min-height: 600px">
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
                <!-- Data Table Brands-->
                <n-data-table
                    :columns="brandColumns"
                    :data="dataTableBrands"
                    :max-height="400"
                    
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
import { ref, watch, h, onMounted} from 'vue'
import { useSettingsStore } from '@/stores/settingsStore'
import { useGlobalStore } from '@/stores/globalStore';
import { useSeedsStore } from '@/stores/seedsStore';
import { useCoatingsStore } from '@/stores/coatingsStore';
import { useBrandsStore } from '@/stores/brandsStore';
import AppCreateSeed from '@/components/AppCreateSeed.vue'
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
  useDialog,
} from 'naive-ui'
   
     

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

const showCreateSeed = ref(false);

onMounted(async() => {
    await seedsStore.fetchSeeds();
    await coatingsStore.fetchCoatings();
    await brandsStore.fetchBrands();
});

    
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
                        onClick: () => editSeedHandler(row) 
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
        content: `Tem certeza que deseja excluir a cultivar "${seed.popular_name}"?\n\n\nEsta ação não pode ser desfeita.`,
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

// editar (implementar depois)
function editSeedHandler(seed: any) {
    globalStore.showMessage({
        content: 'Função de editar será implementada em breve',
        type: 'info'
    });
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
                    type: 'error',
                    renderIcon: () => h(DeleteOutlined)
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
                    type: 'error',
                    renderIcon: () => h(DeleteOutlined)
                    }
                )
            ];
        }
    }
  ]
}

</script>

