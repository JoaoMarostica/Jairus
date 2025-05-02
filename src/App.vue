<template>
  <n-config-provider 
    :theme="theme === 'dark' ? darkTheme : null"
    :theme-overrides="theme === 'dark' ? darkThemeOverrides : lightThemeOverrides"
  >
    <MainView />
  </n-config-provider>
</template>

<script setup lang="ts">
import { onMounted, watch } from 'vue';
import { storeToRefs } from 'pinia';
import { useGlobalStore } from '@/stores/globalStore';
import { lightThemeOverrides, darkThemeOverrides } from '@/styles/naiveUI';
import { NConfigProvider, darkTheme } from 'naive-ui';
import MainView from '@/views/MainView.vue';

const globalStore = useGlobalStore();
const { theme } = storeToRefs(globalStore);

onMounted(() => {
  globalStore.detectSystemTheme();
});

watch(theme, (newTheme) => {
  localStorage.setItem('theme', newTheme);
});

</script>
