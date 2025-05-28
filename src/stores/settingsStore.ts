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
    coatings: [{
        name: 'Golden',
      },{
        name: 'Podium',
      },
    ],
    brands: [{
        name: 'Ponto Alto',
        sackWeights: '10, 15, 25, 30',
      },{
        name: 'Nova Safra',
        sackWeights: '10, 15, 25, 30',
      },
    ],
  }),
  actions: {
  }
})
