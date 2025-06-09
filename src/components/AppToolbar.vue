<template>
  <n-layout-header class="header-fixed">
    <n-flex align="center" justify="space-between" style="width: 100%">
      <router-link to="/" style="display: flex; align-items: center">
        <img
          v-if="theme === 'light'"
          :src="logoLightTheme"
          alt="Jairus Logo"
          style="height: 30px; cursor: pointer; margin-left: 5px"
        />
        <img
          v-else
          :src="logoDarkTheme"
          alt="Jairus Logo"
          style="height: 30px; cursor: pointer; margin-left: 5px"
        />
      </router-link>

      <n-flex>
        <n-tooltip placement="bottom" trigger="hover" :disabled="batches.length === 0">
          <template #trigger>
            <n-button @click="generateReportPDF">
              <template #icon>
                <n-icon><DocumentPdf /></n-icon>
              </template>
            </n-button>
          </template>
          Gerar Relatório em PDF
        </n-tooltip>
        <n-tooltip placement="bottom" trigger="hover" :disabled="batches.length === 0">
          <template #trigger>
            <n-button @click="fileUploadModal = true" :disabled="batches.length === 0 || selectedBatches.length === 0">
              <template #icon>
                <n-icon><UploadFileOutlined /></n-icon>
              </template>
            </n-button>
          </template>
          <div v-if="selectedBatches.length === 0">Selecione os Lotes Desejados</div>
          <div v-else>Importar Planilha Excel</div>
        </n-tooltip>
        <n-tooltip placement="bottom" trigger="hover">
          <template #trigger>
            <n-button @click="globalStore.toggleTheme">
              <template #icon v-if="theme === 'light'">
                <n-icon><WbSunnyOutlined /></n-icon>
              </template>
              <template #icon v-else>
                <n-icon><NightlightOutlined /></n-icon>
              </template>
            </n-button>
          </template>
          Tema {{ theme === 'light' ? 'Claro' : 'Escuro' }}
        </n-tooltip>
        <n-tooltip placement="bottom-end" trigger="hover">
          <template #trigger>
            <n-button @click="settingsModal = true">
              <template #icon>
                <n-icon><SettingsOutlined /></n-icon>
              </template>
            </n-button>
          </template>
          Configurações
        </n-tooltip>
      </n-flex>
    </n-flex>
    <AppFileUpload />
    <AppSettings />
  </n-layout-header>
</template>

<script setup lang="ts">
import { UploadFileOutlined, WbSunnyOutlined, NightlightOutlined, SettingsOutlined } from '@vicons/material';
import { DocumentPdf } from '@vicons/carbon';
import logoLightTheme from '@/assets/jairus-logos/navbar/Jairus2.png'
import logoDarkTheme from '@/assets/jairus-logos/navbar/Jairus3.png'
import AppFileUpload from './AppFileUpload.vue';
import AppSettings from './AppSettings.vue';
import { useGlobalStore } from '@/stores/globalStore';
import { useSettingsStore } from '@/stores/settingsStore';
import { useBatchesStore } from '@/stores/batchesStore';
import { storeToRefs } from 'pinia';
import {
  NLayoutHeader,
  NButton,
  NIcon,
  NFlex,
  NTooltip
} from 'naive-ui'

defineEmits(['toggle-sidebar']);

const globalStore = useGlobalStore();
const { theme, fileUploadModal } = storeToRefs(globalStore);

const settingsStore = useSettingsStore();
const { settingsModal } = storeToRefs(settingsStore);

const batchesStore = useBatchesStore();
const { batches, selectedBatches } = storeToRefs(batchesStore);

async function generateReportPDF() {
  try {
    const filePath = await batchesStore.generateReportPDF();

    globalStore.showMessage({
      content: 'Relatório gerado com sucesso em ' + filePath,
      type: 'success',
      keepAliveOnHover: true,
    })
  } catch (error: any) {
    console.error(error);
    globalStore.showMessage({
      content: 'Erro ao gerar relatório.',
      type: 'error',
    })
  }
}

</script>

<style scoped>
.header-fixed {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  padding: 16px;
  background-color: var(--n-color);
}
</style>
