import { defineStore } from 'pinia';

export const useGlobalStore = defineStore('global', {
  state: () => ({
    theme: (localStorage.getItem('theme') as 'light' | 'dark') ?? 
    (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light') as 'light' | 'dark',
    sidebar: false,
    fileUploadModel: false,
    columns: Array<{ title: string; key: string }>(),
    data: Array<{}>()
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
    setColumns(columns: Array<{ title: string; key: string }>) {
      this.columns = columns;
    },
    setData(data: Array<{  }>) {
      this.data = data;
    },
  }
});
