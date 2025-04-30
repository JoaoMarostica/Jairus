<template>
  <n-layout 
    has-sider 
    class="app-layout" 
  >
    <AppToolbar @toggle-sidebar="globalStore.toggleSidebar" />
    <AppDrawer />
    
    <n-layout-content  class="content-wrapper">
      <n-card 
        class="main-content" 
        content-style="padding: 24px"
        embedded
      >
        <router-view />
      </n-card>
    </n-layout-content>
  </n-layout>
</template>

<script setup lang="ts">
// import { storeToRefs } from 'pinia';
import AppDrawer from '../components/AppDrawer.vue'
import AppToolbar from '../components/AppToolbar.vue'
import { useGlobalStore } from '@/stores/globalStore';
import {
  NLayout,
  NLayoutContent,
  NCard,
} from 'naive-ui'

defineEmits(['toggle-sidebar']);

const globalStore = useGlobalStore();
// const { sidebar } = storeToRefs(globalStore);

</script>

<style scoped>
.app-layout {
  height: 100vh;
  padding-top: 50px;
}

.content-wrapper {
  height: calc(100vh - 50px); /* altura restante após AppToolbar */
  overflow: auto;
}

.main-content {
  height: 100%;

  border-top-left-radius: 12px;
  border-top-right-radius: 0;
  border-bottom-left-radius: 0;
  border-bottom-right-radius: 0;
}
</style>
