import { defineStore } from 'pinia'
import { computed } from 'vue'
import { useWindowSize } from '@vueuse/core'

type MessageData = {
  content: string
  placement?: 'top' | 'bottom' | 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right'
  closable?: boolean
  duration?: number
  icon?: () => void
  keepAliveOnHover?: boolean
  showIcon?: boolean
  type: 'info' | 'success' | 'warning' | 'error' | 'loading' | 'default'
  onAfterLeave?: () => void
  onClose?: () => void
  onLeave?: () => void
}

const { width, height } = useWindowSize()

export const useGlobalStore = defineStore('global', {
  state: () => ({
    theme: (localStorage.getItem('theme') as 'light' | 'dark') ??
      (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light') as 'light' | 'dark',
    sidebar: false,
    fileUploadModal: false,
    siderbarWidth: 16,
    message: {
      content: '',
      placement: 'top',
      closable: true,
      duration: 3000,
      icon: () => {},
      keepAliveOnHover: false,
      showIcon: true,
      type: 'default',
      onAfterLeave: () => {},
      onClose: () => {},
      onLeave: () => {},
    } as MessageData,
  }),

  getters: {
    windowWidth: () => Number(width),
    windowHeight: () => Number(height),
    isMobile: () => computed(() => width.value <= 768),
  },

  actions: {
    toggleTheme() {
      this.theme = this.theme === 'light' ? 'dark' : 'light'
    },
    setTheme(theme: 'light' | 'dark') {
      this.theme = theme
    },
    detectSystemTheme() {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
      this.theme = mediaQuery.matches ? 'dark' : 'light'

      mediaQuery.addEventListener('change', (e) => {
        this.theme = e.matches ? 'dark' : 'light'
      })
    },
    toggleSidebar() {
      this.sidebar = !this.sidebar
    },
    showMessage(messageOptions: Partial<MessageData>) {
      Object.assign(this.message, {
        ...this.message,
        ...messageOptions,
      })
    },
  }
})
