<template>
    <n-modal v-model:show="settingsModal" preset="card" title="Configurações do Sistema" style="width: 500px; min-height: 500px">
        <n-tabs type="segment" animated>
            <n-tab-pane name="geral" tab="Geral">
                <n-form label-placement="top">
                <n-form-item label="Safra atual">
                    <n-input v-model:value="settings.safraAtual.value" placeholder="Ex: 2024/2025" />
                </n-form-item>
                <n-form-item label="Unidade padrão">
                    <n-select v-model:value="settings.unidade.value" :options="unidadeOptions" />
                </n-form-item>
                <n-form-item label="Preço padrão por ponto de pureza (R$/PP)">
                    <n-input-number v-model:value="settings.precoPP.value" :min="0" :precision="2" />
                </n-form-item>
                </n-form>
            </n-tab-pane>

            <n-tab-pane name="tratamentos" tab="Tipos de Tratamento">
                <n-dynamic-input
                v-model:value="settings.tratamentos.value"
                placeholder="Digite um tipo de tratamento"
                />
            </n-tab-pane>

            <n-tab-pane name="cultivares" tab="Cultivares">
                <n-dynamic-input
                v-model:value="settings.cultivares.value"
                placeholder="Digite uma cultivar"
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
const { settingsModal, safraAtual, unidade, precoPP, tratamentos, cultivares } = storeToRefs(settingsStore)

const settings = {
  safraAtual,
  unidade,
  precoPP,
  tratamentos,
  cultivares
}

const unidadeOptions = [
  { label: 'kg', value: 'kg' },
  { label: 'sacas', value: 'sacas' },
  { label: 'toneladas', value: 'toneladas' }
]

const show = ref(false) // controle local — substitua pelo global se quiser

function handleSave() {
  settingsStore.atualizarTratamentos(settings.tratamentos.value)
  settingsStore.atualizarCultivares(settings.cultivares.value)
  show.value = false
}

// Função para cancelar (fechar o modal sem salvar)
const handleCancel = () => {
    settingsModal.value = false
}
</script>
