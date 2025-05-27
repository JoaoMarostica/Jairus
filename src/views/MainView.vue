<template>
  <n-layout 
    has-sider 
    position="absolute"
    class="app-layout" 
  >
    <AppToolbar @toggle-sidebar="globalStore.toggleSidebar" />
    
    <n-layout-content
      class="content-wrapper"
      :style="{ marginLeft: siderbarWidth + 'px' }"
    >
      <router-view />
    </n-layout-content>
  </n-layout>
</template>

<script setup lang="ts">
import AppToolbar from '../components/AppToolbar.vue'
import { useGlobalStore } from '@/stores/globalStore';
import { storeToRefs } from 'pinia'
import {
  NLayout,
  NLayoutContent,
} from 'naive-ui'

defineEmits(['toggle-sidebar']);

const globalStore = useGlobalStore();
const { siderbarWidth } = storeToRefs(globalStore)

</script>

<style scoped>
.app-layout {
  padding-top: 50px;
  min-height: 100vh;
}

.content-wrapper {
  flex: 1;
  padding: 16px 16px 16px 0px;
}

</style>
