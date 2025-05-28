import { defineStore } from 'pinia'

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    settingsModal: false,
    seeds: ['Mombaça', 'Piatã'],
    coatings: ['Golden', 'Podium'],
    brands: ['Ponto Alto', 'Nova Safra'],
  }),
  actions: {
    updateSeeds(newSeeds: string[]) {
      this.seeds = newSeeds
    },
    updateCoatings(newCoatings: string[]) {
      this.coatings = newCoatings
    },
    updateBrands(newBrands: string[]) {
      this.brands = newBrands
    },
  }
})
