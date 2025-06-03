import { defineStore } from 'pinia'

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    settingsModal: false,
    seeds: [{
        scientificName: 'Panicum Maximum cv Mombaça',
        popularName: 'Mombaça',
      },
      {
        scientificName: 'Brachiaria Brizantha cv BRS Piatã',
        popularName: 'Piatã',
      },
    ],
    coatings: [
      {
        name: 'Golden',
      },
      {
        name: 'Podium',
      },
      {
        name: 'Convencional',
      },
    ],
    brands: [{
        name: 'Ponto Alto',
        sackWeights: [
          {
            label: '10',
            value: '10',
          },
          {
            label: '15',
            value: '15',
          },
          {
            label: '25',
            value: '25',
          },
          {
            label: '30',
            value: '30',
          },
        ],
      },{
        name: 'Nova Safra',
        sackWeights: [
          {
            label: '10',
            value: '10',
          },
          {
            label: '15',
            value: '15',
          },
          {
            label: '25',
            value: '25',
          },
          {
            label: '30',
            value: '30',
          },
        ],
      },
    ],
  }),
  actions: {
  }
})
