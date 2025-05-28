<template>
    <n-modal v-model:show="settingsModal" preset="card" title="Configurações do Sistema" style="width: 500px; min-height: 500px">
        <n-tabs type="segment" animated>
            <n-tab-pane name="seeds" tab="Cultivares">
                <n-dynamic-input
                v-model:value="settings.seeds.value"
                placeholder="Digite uma cultivar"
                />
            </n-tab-pane>

            <n-tab-pane name="coatings" tab="Revestimentos">
                <n-dynamic-input
                v-model:value="settings.coatings.value"
                placeholder="Digite um tipo de revestimento"
                />
            </n-tab-pane>

            <n-tab-pane name="brands" tab="Marcas">
                <n-dynamic-input
                v-model:value="settings.brands.value"
                placeholder="Digite uma marca"
                />
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
import { ref } from 'vue'
import { useSettingsStore } from '@/stores/settingsStore'
import { storeToRefs } from 'pinia';
import {
  NModal,
  NTabs,
  NTabPane,
  NForm,
  NFormItem,
  NInput,
  NInputNumber,
  NSelect,
  NDynamicInput,
  NButton
} from 'naive-ui'

const settingsStore = useSettingsStore()
const { settingsModal, seeds, coatings, brands } = storeToRefs(settingsStore)

const settings = {
  seeds,
  coatings,
  brands
}

const show = ref(false) // controle local — substitua pelo global se quiser

function handleSave() {
    settingsStore.updateSeeds(settings.seeds.value)
    settingsStore.updateCoatings(settings.coatings.value)
    settingsStore.updateBrands(settings.brands.value)
    show.value = false
}

// Função para cancelar (fechar o modal sem salvar)
const handleCancel = () => {
    settingsModal.value = false
}
</script>
