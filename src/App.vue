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
const { dataTableBatches } = storeToRefs(batchesStore)

onMounted(async () => {
  globalStore.detectSystemTheme();
  await batchesStore.fetchBatches().then(() => {
    // globalStore.showMessage({
    //   content: 'Lotes carregados com sucesso!',
    //   type: 'success',
    // })
    // globalStore.showMessage({
    //   content: JSON.stringify(dataTableBatches.value[0], null, 2),
    //   type: 'info',
    //   keepAliveOnHover: true
    // })
  })
  .catch((err) => {
    globalStore.showMessage({
      content: `Erro ao carregar lotes: ${err.message}`,
      type: 'error',
    })
  })
});

watch(theme, (newTheme) => {
  localStorage.setItem('theme', newTheme);
});

</script>
