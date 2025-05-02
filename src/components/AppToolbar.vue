<template>
  <n-layout-header class="header-fixed">
    <n-flex align="center" justify="space-between" style="width: 100%">
      <div>Jairus</div>

      <n-flex>
        <n-button>
          <template #icon>
            <n-icon><FileUploadOutlined /></n-icon>
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
        <n-button>
          <template #icon>
            <n-icon><SettingsOutlined /></n-icon>
          </template>
        </n-button>
      </n-flex>
    </n-flex>
  </n-layout-header>
</template>

<script setup lang="ts">
import { FileUploadOutlined, WbSunnyOutlined, NightlightOutlined, SettingsOutlined } from '@vicons/material';
import { useGlobalStore } from '@/stores/globalStore';
import { storeToRefs } from 'pinia';
import {
  NLayoutHeader,
  NButton,
  NIcon,
  NFlex,
} from 'naive-ui'

defineEmits(['toggle-sidebar']);

const globalStore = useGlobalStore();
const { theme } = storeToRefs(globalStore);

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
