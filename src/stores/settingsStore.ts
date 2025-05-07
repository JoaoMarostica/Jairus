import { defineStore } from 'pinia'

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    settingsModal: false,
    safraAtual: '',
    unidade: 'kg',
    precoPP: 0,
    tratamentos: ['Golden', 'Podium'],
    cultivares: ['Mombaça', 'Piatã']
  }),
  actions: {
    atualizarTratamentos(novos: string[]) {
      this.tratamentos = novos
    },
    atualizarCultivares(novos: string[]) {
      this.cultivares = novos
    }
  }
})
