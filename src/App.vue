<template>
  <n-config-provider 
    :theme="theme === 'dark' ? darkTheme : null"
    :theme-overrides="theme === 'dark' ? darkThemeOverrides : lightThemeOverrides"
  >
    <n-message-provider :placement="message.placement">
      <MainView />
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { onMounted, watch } from 'vue';
import { storeToRefs } from 'pinia';
import { useGlobalStore } from '@/stores/globalStore';
import { useBatchesStore } from './stores/batchesStore';
import { lightThemeOverrides, darkThemeOverrides } from '@/styles/naiveUI';
import { NConfigProvider, NMessageProvider, darkTheme } from 'naive-ui';
import MainView from '@/views/MainView.vue';

const globalStore = useGlobalStore();
const { theme, message } = storeToRefs(globalStore);

const batchesStore = useBatchesStore();

onMounted(async () => {
  // initializeApp();
});

watch(theme, (newTheme) => {
  localStorage.setItem('theme', newTheme);
});

async function initializeApp() {
  globalStore.detectSystemTheme();

  try {
    await batchesStore.fetchBatches();
    globalStore.showMessage({
      content: 'Lotes carregados com sucesso!',
      type: 'success',
    });
  } catch (err) {
    globalStore.showMessage({
      content: `Erro ao carregar lotes: ${err instanceof Error ? err.message : String(err)}`,
      type: 'error',
    });
  }
}

</script>
