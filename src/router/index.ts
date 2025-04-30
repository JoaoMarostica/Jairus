import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  // main pages routes
  { path: '/', name: 'overview', component: () => import('@/pages/OverviewPage.vue') },
  { path: '/inventory', name: 'inventory', component: () => import('@/pages/InventoryPage.vue') },
  { path: '/cargo', name: 'cargo', component: () => import('@/pages/CargoPage.vue') },
  { path: '/treatments', name: 'treatments', component: () => import('@/pages/TreatmentsPage.vue') },
  { path: '/sales', name: 'sales', component: () => import('@/pages/SalesPage.vue') },
  { path: '/report', name: 'report', component: () => import('@/pages/ReportPage.vue') },
  { path: '/spv-calculator', name: 'spaCalculator', component: () => import('@/pages/SpvCalculatorPage.vue') },

  // static pages routes
  { path: '/about', name: 'about', component: () => import('@/pages/AboutPage.vue') },
  { path: '/help', name: 'help', component: () => import('@/pages/HelpPage.vue') },

  // route catch-all 404
  { path: '/:pathMatch(.*)*', name: 'not-found', component: () => import('@/pages/NotFoundPage.vue') }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
