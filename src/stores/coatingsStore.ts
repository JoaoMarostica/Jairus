import { defineStore } from 'pinia';
import type { CoatingDB, DataTableCoating } from '@/types/coatings';
import { invoke } from '@tauri-apps/api/core';


export const useCoatingsStore = defineStore('coatings', {
  state: () => ({
    coatings: [] as CoatingDB[],
    dataTableCoatings: [] as DataTableCoating[],
  }),
  actions: {
    async fetchCoatings() {
        try {
            this.$reset;

            this.coatings = await invoke('list_coatings');
            this.dataTableCoatings = [];

            this.coatings.forEach((coating: CoatingDB) =>
                this.dataTableCoatings.push(formatCoatingForTable(coating))
            );
        } catch (err) {
            console.error(err);
        }
    },

    async createCoating(newCoating: CoatingDB) {
        try {
            const createdCoating: CoatingDB = await invoke('add_coating', {
                coating: newCoating
            });

           this.dataTableCoatings.push(formatCoatingForTable(createdCoating));
        } catch (err) {
            console.error('Erro ao criar Tratamento:', err);
        }
    },

    async editCoating(coating: CoatingDB) {
        try {
            const editedCoating: CoatingDB = await invoke('change_coating', {
                coatingName: coating.coating_name
            });
    
            
            const index = this.dataTableCoatings.findIndex(c => c.key === coating.coating_name);


            if (index !== -1) {
                this.dataTableCoatings[index] = formatCoatingForTable(editedCoating);
            }
    
            this.coatings = await invoke('list_coatings');
        } catch (err) {
            console.error(err);
            throw err;
        }
    },

    async removeCoating(coating: DataTableCoating) {
        try {
            await invoke('remove_coating', {
                coatingName: coating.coating_name
                    
            });
    
            await this.fetchCoatings();
        } catch (err) {
            console.error(err);
            throw err;
        }
    },
  },
 
});

function formatCoatingForTable(coating: CoatingDB): DataTableCoating {
    const coatingForTable: DataTableCoating = {
        key: coating.coating_name,
        coating_name: coating.coating_name,
        
    };

    return coatingForTable
}
