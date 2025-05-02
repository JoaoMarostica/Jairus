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
import { BarChartOutlined, WarehouseOutlined, ShowChartOutlined, AttachMoneyOutlined, AgricultureOutlined, CalculateOutlined, InfoOutlined, CategoryOutlined, HelpOutlineOutlined } from '@vicons/material'
import {
  NMenu,
  NLayoutSider,
} from 'naive-ui'

const router = useRouter()
const route = useRoute()
const activeKey = ref(route.path)

const menuOptions = computed(() => [
  {
    label: 'Visão Geral',
    key: '/',
    icon: () => h(BarChartOutlined),
  },
  {
    label: 'Estoque',
    key: '/inventory',
    icon: () => h(WarehouseOutlined),
  },
  {
    label: 'Cargas',
    key: '/cargo',
    icon: () => h(AgricultureOutlined),
  },
  {
    label: 'Tratamentos',
    key: '/treatments',
    icon: () => h(CategoryOutlined),
  },
  {
    label: 'Vendas',
    key: '/sales',
    icon: () => h(AttachMoneyOutlined),
  },
  {
    label: 'Relatório',
    key: '/report',
    icon: () => h(ShowChartOutlined),
  },
  {
    label: 'Cálculadora SPV',
    key: '/spv-calculator',
    icon: () => h(CalculateOutlined),
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
