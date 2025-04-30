import { defineStore } from 'pinia';
import cultivarsInfo from '@/assets/cultivarsInfo.json';

export const useCultivarsStore = defineStore('cultivars', {
  state: () => ({
    cultivars: [] as any[],
  }),
  getters: {
  },
  actions: {
    fetchCultivars() {
        for (const cultivar of cultivarsInfo) {
            this.cultivars.push(cultivar);
        }
    }
  }
});
