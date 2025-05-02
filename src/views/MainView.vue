<template>
  <n-layout 
    has-sider 
    class="app-layout" 
  >
    <AppToolbar @toggle-sidebar="globalStore.toggleSidebar" />
    <AppDrawer />
    
    <n-layout-content class="content-wrapper">
      <router-view />
    </n-layout-content>
  </n-layout>
</template>

<script setup lang="ts">
import AppDrawer from '../components/AppDrawer.vue'
import AppToolbar from '../components/AppToolbar.vue'
import { useGlobalStore } from '@/stores/globalStore';
import {
  NLayout,
  NLayoutContent,
} from 'naive-ui'

defineEmits(['toggle-sidebar']);

const globalStore = useGlobalStore();

</script>

<style scoped>
.app-layout {
  padding-top: 50px;
  display: flex;
  flex-direction: column;
  min-height: 100vh; /* usa min-height para que cresça conforme o conteúdo */
}

.content-wrapper {
  flex: 1;
  padding: 16px;
  margin-left: 250px;
}

</style>
