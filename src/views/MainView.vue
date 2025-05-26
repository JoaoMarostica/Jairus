<template>
  <n-layout 
    has-sider 
    position="absolute"
    class="app-layout" 
  >
    <AppToolbar @toggle-sidebar="globalStore.toggleSidebar" />
    <AppDrawer />
    
    <n-layout-content
      class="content-wrapper"
      :style="{ marginLeft: siderbarWidth + 'px' }"
    >
      <router-view />
    </n-layout-content>
  </n-layout>
</template>

<script setup lang="ts">
import AppDrawer from '../components/AppDrawer.vue'
import AppToolbar from '../components/AppToolbar.vue'
import { useGlobalStore } from '@/stores/globalStore';
import { storeToRefs } from 'pinia'
import {
  NLayout,
  NLayoutContent,
  useMessage
} from 'naive-ui'
import { watch } from 'vue';

defineEmits(['toggle-sidebar']);

const globalStore = useGlobalStore();
const { siderbarWidth, message } = storeToRefs(globalStore)

const messageApi = useMessage();

watch(message.value, () => {
  if (message.value) {
    createMessage();
  }
});

function createMessage() {
  if (!message.value.content) {
    console.error('Message content is empty');
    return;
  }
  messageApi.create(
    message.value.content,
    {
      closable: message.value.closable,
      duration: message.value.duration,
      keepAliveOnHover: message.value.keepAliveOnHover,
      showIcon: message.value.showIcon,
      type: message.value.type,
      onAfterLeave: message.value.onAfterLeave,
      onClose: message.value.onClose,
      onLeave: message.value.onLeave,
    }
  );
}

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
