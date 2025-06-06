import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  // main pages routes
  { path: '/', name: 'batch', component: () => import('@/pages/BatchesPage.vue') },

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
