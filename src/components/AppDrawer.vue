<template>
  <n-layout-sider
    :collapsed="sidebar"
    :width="250"
    :collapsed-width="0"
    collapse-mode="transform"
    class="drawer"
    @update:sidebar="sidebar = $event"
  >
    <div class="p-4 flex items-center text-lg font-medium">
      <n-avatar
        lazy
        round
        :size="48"
        src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg"
      />
      Samuel Krabbe
    </div>

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
import { useGlobalStore } from '@/stores/globalStore';
import { BarChartOutlined, Inventory2Outlined, ShowChartOutlined, ShoppingCartOutlined, AddShoppingCartOutlined, CalculateOutlined, FolderOpenOutlined, InfoOutlined, InsertDriveFileOutlined, LockOutlined, AccountCircleOutlined, LogOutOutlined, HelpOutlineOutlined } from '@vicons/material'
import {
  NMenu,
  NAvatar,
  NLayoutSider,
} from 'naive-ui'
import { storeToRefs } from 'pinia';

const router = useRouter()
const route = useRoute()
const activeKey = ref(route.path)

const globalStore = useGlobalStore()
const { sidebar } = storeToRefs(globalStore)

const menuOptions = computed(() => [
  {
    label: 'Visão Geral',
    key: '/',
    icon: () => h(BarChartOutlined),
  },
  {
    label: 'Estoque',
    key: '/inventory',
    icon: () => h(Inventory2Outlined),
  },
  {
    label: 'Relatório',
    key: '/report',
    icon: () => h(ShowChartOutlined),
  },
  {
    label: 'Vendas',
    key: '/sales',
    icon: () => h(ShoppingCartOutlined),
  },
  {
    label: 'Compras',
    key: '/purchases',
    icon: () => h(AddShoppingCartOutlined),
  },
  {
    label: 'Cálculo SPV',
    key: '/spv-calculator',
    icon: () => h(CalculateOutlined),
  },
  {
    label: 'Projetos',
    key: '/projects',
    icon: () => h(FolderOpenOutlined),
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
    label: 'Termos de uso',
    key: '/terms',
    icon: () => h(InsertDriveFileOutlined),
  },
  {
    label: 'Política de privacidade',
    key: '/privacy',
    icon: () => h(LockOutlined),
  },
  {
    label: 'Conta',
    key: '/account',
    icon: () => h(AccountCircleOutlined),
  },
  {
    label: 'Sair',
    key: '/logout',
    icon: () => h(LogOutOutlined),
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
.drawer {
  position: fixed;
  top: 0;
  left: 0;
  padding: 10px 0px 0px 0px;
  height: 100%;
  z-index: 999;
}
</style>
