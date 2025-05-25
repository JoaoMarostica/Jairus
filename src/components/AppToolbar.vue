<template>
  <n-layout-header class="header-fixed">
    <n-flex align="center" justify="space-between" style="width: 100%">
      <div>Jairus</div>

      <n-flex>
        <n-button @click="fileUploadModal = true">
          <template #icon>
            <n-icon><UploadFileOutlined /></n-icon>
          </template>
        </n-button>
        <n-button @click="batchesStore.downloadPdf">
          <template #icon>
            <n-icon><PictureAsPdfOutlined /></n-icon>
          </template>
        </n-button>
        <n-button @click="globalStore.toggleTheme">
          <template #icon v-if="theme === 'light'">
            <n-icon><NightlightOutlined /></n-icon>
          </template>
          <template #icon v-else>
            <n-icon><WbSunnyOutlined /></n-icon>
          </template>
        </n-button>
        <n-button @click="settingsModal = true">
          <template #icon>
            <n-icon><SettingsOutlined /></n-icon>
          </template>
        </n-button>
      </n-flex>
    </n-flex>
    <AppFileUpload />
    <AppSettings />
  </n-layout-header>
</template>

<script setup lang="ts">
import { UploadFileOutlined, WbSunnyOutlined, NightlightOutlined, SettingsOutlined, PictureAsPdfOutlined } from '@vicons/material';
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
} from 'naive-ui'

defineEmits(['toggle-sidebar']);

const globalStore = useGlobalStore();
const { theme, fileUploadModal } = storeToRefs(globalStore);

const settingsStore = useSettingsStore()
const { settingsModal } = storeToRefs(settingsStore)

const batchesStore = useBatchesStore()

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
