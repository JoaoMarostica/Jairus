<template>
  <n-layout 
    has-sider 
    class="app-layout" 
  >
    <AppDrawer />

    <n-layout-content
      :style="{
        marginLeft: sidebar ? '0' : '250px',
        transition: 'margin-left 0.3s ease',
      }"
    >
      <n-layout>
        <n-layout-header bordered>
          <AppToolbar @toggle-sidebar="globalStore.toggleSidebar" />
        </n-layout-header>

        <n-layout-content class="main-content">
          <router-view />
        </n-layout-content>
      </n-layout>
    </n-layout-content>
  </n-layout>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import AppDrawer from '../components/AppDrawer.vue'
import AppToolbar from '../components/AppToolbar.vue'
import { useGlobalStore } from '@/stores/globalStore';
import {
  NLayout,
  NLayoutHeader,
  NLayoutContent,
} from 'naive-ui'

defineEmits(['toggle-sidebar']);

const globalStore = useGlobalStore();
const { sidebar } = storeToRefs(globalStore);

</script>

<style scoped>
.app-layout {
  height: 100vh;
  padding-top: 50px;
}

.main-content {
  padding: 16px;
  z-index: 1;
}
</style>
