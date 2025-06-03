<template>
  <n-layout-sider
    :width="siderbarWidth"
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
import { ref, h, onBeforeMount } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { WarehouseOutlined, InfoOutlined, HelpOutlineOutlined } from '@vicons/material'
import {
  NMenu,
  NLayoutSider,
} from 'naive-ui'
import { useGlobalStore } from '@/stores/globalStore'
import { storeToRefs } from 'pinia'

const globalStore = useGlobalStore()
const { siderbarWidth } = storeToRefs(globalStore)

const router = useRouter()
const route = useRoute()
const activeKey = ref(route.path)

const menuOptions = [
  { label: 'Lotes', key: '/', icon: () => h(WarehouseOutlined) },
  { type: 'divider' },
  { label: 'Sobre', key: '/about', icon: () => h(InfoOutlined) },
  { label: 'Ajuda', key: '/help', icon: () => h(HelpOutlineOutlined) },
]

onBeforeMount(() => {
  const longestLabelLength = Math.max(...menuOptions.filter(i => i.label).map(i => String(i.label).length))
  const basePadding = 60 // espaço para ícone + espaçamento
  const pxPerChar = 20   // largura estimada por caractere

  siderbarWidth.value = longestLabelLength * pxPerChar + basePadding
})

function handleMenuClick(key: string) {
  activeKey.value = key
  router.push(key)
}
</script>

<style scoped>
.app-drawer {
  height: 100vh;
  padding-top: 16px;
  position: fixed;
  z-index: 1000;
}
</style>
