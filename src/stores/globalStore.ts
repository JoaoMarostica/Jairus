import { defineStore } from 'pinia';
import { ref } from 'vue';

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

export const useGlobalStore = defineStore('global', {
  state: () => ({
    theme: (localStorage.getItem('theme') as 'light' | 'dark') ?? 
    (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light') as 'light' | 'dark',
    sidebar: false,
    fileUploadModal: false,
    siderbarWidth: 16, // If using the AppDrawer component this can be set to 250
    message: ref<MessageData>({
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
    }),
  }),
  actions: {
    toggleTheme() {
      this.theme = this.theme === 'light' ? 'dark' : 'light';
    },
    setTheme(theme: 'light' | 'dark') {
      this.theme = theme;
    },
    detectSystemTheme() {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
      this.theme = 'light';

      mediaQuery.addEventListener('change', (e) => {
        this.theme = e.matches ? 'dark' : 'light';
      });
    },
    toggleSidebar() {
      this.sidebar = !this.sidebar;
    },
    showMessage(messageOptions: any) {
      this.message.content = messageOptions.content || '';
      this.message.placement = messageOptions.placement || 'top';
      this.message.closable = messageOptions.closable !== undefined ? messageOptions.closable : true;
      this.message.duration = messageOptions.duration || 3000;
      this.message.icon = messageOptions.icon || (() => {});
      this.message.keepAliveOnHover = messageOptions.keepAliveOnHover !== undefined ? messageOptions.keepAliveOnHover : false;
      this.message.showIcon = messageOptions.showIcon !== undefined ? messageOptions.showIcon : true;
      this.message.type = messageOptions.type || 'default';
      this.message.onAfterLeave = messageOptions.onAfterLeave || (() => {});
      this.message.onClose = messageOptions.onClose || (() => {});
      this.message.onLeave = messageOptions.onLeave || (() => {});
    },
  }
});
