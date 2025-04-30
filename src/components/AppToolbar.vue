<template>
  <n-layout-header
    class="header-fixed"
  >
    <n-flex align="center">
      <!-- <n-button
        @click="$emit('toggle-sidebar')"
      >
        <template #icon>
          <n-icon><MenuOutlined /></n-icon>
        </template>
      </n-button> -->
      Jairus
    </n-flex>

    <n-flex justify="end" class="buttons-fixed">
      <n-button>
        <template #icon>
          <n-icon><FileUploadOutlined /></n-icon>
        </template>
      </n-button>
      <n-button 
        @click="globalStore.toggleTheme">
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
const { theme, sidebar } = storeToRefs(globalStore);

</script>

<style scoped>
.header-fixed {
position: fixed;
top: 0;
z-index: 1000;
background-color: var(--n-color-body);
height: 50px;
display: flex;
justify-content: space-between;
align-items: center;
padding: 0 10px;
transition: left 0.3s ease;
}

.buttons-fixed {
position: fixed;
right: 0;
padding: 0 10px;
}

.flat-button {
  box-shadow: none !important;
  background: transparent !important;
  border: none !important;
  padding: 6px 12px; /* opcional, para não ficar pequeno demais */
  color: inherit; /* mantém a cor do texto do tema */
}

.flat-button:hover,
.flat-button:focus,
.flat-button:active {
  background: transparent !important;
  box-shadow: none !important;
  border: none !important;
}
</style>
