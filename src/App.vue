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
import { lightThemeOverrides, darkThemeOverrides } from '@/styles/naiveUI';
import { NConfigProvider, NMessageProvider, darkTheme } from 'naive-ui';
import MainView from '@/views/MainView.vue';

const globalStore = useGlobalStore();
const { theme, message } = storeToRefs(globalStore);

onMounted(async () => {
  globalStore.detectSystemTheme();
});

watch(theme, (newTheme) => {
  localStorage.setItem('theme', newTheme);
});

</script>
