import { defineStore } from 'pinia';

export const useGlobalStore = defineStore('global', {
  state: () => ({
    user: null as null | { id: string; name: string },
    isAuthenticated: false,
    theme: (localStorage.getItem('theme') as 'light' | 'dark') ?? 
    (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light') as 'light' | 'dark',
    sidebar: false,
  }),
  getters: {
    isLoggedIn: (state) => state.isAuthenticated,
  },
  actions: {
    login(user: { id: string; name: string }) {
      this.user = user;
      this.isAuthenticated = true;
    },
    logout() {
      this.user = null;
      this.isAuthenticated = false;
    },
    toggleTheme() {
      this.theme = this.theme === 'light' ? 'dark' : 'light';
    },
    setTheme(theme: 'light' | 'dark') {
      this.theme = theme;
    },
    detectSystemTheme() {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
      // this.theme = mediaQuery.matches ? 'dark' : 'light';
      this.theme = 'light';

      // Escuta mudanças no sistema
      mediaQuery.addEventListener('change', (e) => {
        this.theme = e.matches ? 'dark' : 'light';
      });
    },
    toggleSidebar() {
      this.sidebar = !this.sidebar;
    },
  }
});
