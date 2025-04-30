import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  // main pages routes
  { path: '/', name: 'overview', component: () => import('@/pages/OverviewPage.vue') },
  { path: '/inventory', name: 'inventory', component: () => import('@/pages/InventoryPage.vue') },
  { path: '/report', name: 'report', component: () => import('@/pages/ReportPage.vue') },
  { path: '/sales', name: 'sales', component: () => import('@/pages/SalesPage.vue') },
  { path: '/purchases', name: 'purchases', component: () => import('@/pages/PurchasesPage.vue') },
  { path: '/spv-calculator', name: 'spaCalculator', component: () => import('@/pages/SpvCalculatorPage.vue') },
  { path: '/projects', name: 'projects', component: () => import('@/pages/ProjectsPage.vue') },

  // static pages routes
  { path: '/about', name: 'about', component: () => import('@/pages/AboutPage.vue') },
  { path: '/terms', name: 'terms', component: () => import('@/pages/TermsPage.vue') },
  { path: '/privacy', name: 'privacy', component: () => import('@/pages/PrivacyPage.vue') },
  { path: '/account', name: 'account', component: () => import('@/pages/AccountPage.vue') },
  { path: '/help', name: 'help', component: () => import('@/pages/HelpPage.vue') },

  // route catch-all 404
  { path: '/:pathMatch(.*)*', name: 'not-found', component: () => import('@/pages/NotFoundPage.vue') }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
