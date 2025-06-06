<template>
  <n-layout-header class="header-fixed">
    <n-flex align="center" justify="space-between" style="width: 100%">
      <!-- <n-image width="100" src="@/assets/jairus-logos/navbar/Jairus3.png" fit="cover" :style="{ cursor: 'pointer' }">
        <template #error>
          <n-icon :size="100" color="lightGrey">
            <ImageOutlined />
          </n-icon>
        </template>
      </n-image> -->
      <div>Jairus</div>

      <n-flex>
        <n-tooltip placement="bottom" trigger="hover" :disabled="batches.length === 0">
          <template #trigger>
            <n-button @click="fileUploadModal = true" :disabled="batches.length === 0">
              <template #icon>
                <n-icon><UploadFileOutlined /></n-icon>
              </template>
            </n-button>
          </template>
          Importar planilha excel
        </n-tooltip>
        <n-tooltip placement="bottom" trigger="hover" :disabled="batches.length === 0">
          <template #trigger>
            <n-button @click="batchesStore.downloadPdf" :disabled="true">
              <template #icon>
                <n-icon><PictureAsPdfOutlined /></n-icon>
              </template>
            </n-button>
          </template>
          Baixar PDF
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
import { UploadFileOutlined, WbSunnyOutlined, NightlightOutlined, SettingsOutlined, PictureAsPdfOutlined, ImageOutlined } from '@vicons/material';
import AppFileUpload from './AppFileUpload.vue';
import AppSettings from './AppSettings.vue';
import { useGlobalStore } from '@/stores/globalStore';
import { useSettingsStore } from '@/stores/settingsStore'
import { useBatchesStore } from '@/stores/batchesStore';
import { storeToRefs } from 'pinia';
import {
  NLayoutHeader,
  NButton,
  NIcon,
  NFlex,
  NTooltip,
  NImage
} from 'naive-ui'

defineEmits(['toggle-sidebar']);

const globalStore = useGlobalStore();
const { theme, fileUploadModal } = storeToRefs(globalStore);

const settingsStore = useSettingsStore()
const { settingsModal } = storeToRefs(settingsStore)

const batchesStore = useBatchesStore()
const { batches } = storeToRefs(batchesStore);

</script>

<style scoped>
.header-fixed {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  padding: 16px;
  background-color: var(--n-color); /* ou outra cor de fundo */
}
</style>
