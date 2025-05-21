<template>
  <n-layout-sider
    :width="250"
    class="app-drawer"
  >
    <n-menu
      :value="activeKey"
      :options="menuOptions"
      @update:value="handleMenuClick"
    />
  </n-layout-sider>
</template>

<script setup lang="ts">
import { ref, computed, h } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { WarehouseOutlined, InfoOutlined, HelpOutlineOutlined } from '@vicons/material'
import {
  NMenu,
  NLayoutSider,
} from 'naive-ui'

const router = useRouter()
const route = useRoute()
const activeKey = ref(route.path)

const menuOptions = computed(() => [
  {
    label: 'Lotes',
    key: '/',
    icon: () => h(WarehouseOutlined),
  },
  {
    type: 'divider',
  },
  {
    label: 'Sobre',
    key: '/about',
    icon: () => h(InfoOutlined),
  },
  {
    label: 'Ajuda',
    key: '/help',
    icon: () => h(HelpOutlineOutlined),
  }
])

function handleMenuClick(key: string) {
  activeKey.value = key
  router.push(key)
}
</script>

<style scoped>
.app-drawer {
  height: 100vh;
  padding: 16px;
  position: fixed;
  z-index: 1000;
}
</style>
