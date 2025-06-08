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

             // Verifica se já existe
            const existingCoating = this.coatings.find(
                c => c.coating_name.toLowerCase() === newCoating.coating_name.toLowerCase()
            );
            if (existingCoating) {
                throw new Error('Tratamento já existe');
            }

            const createdCoating: CoatingDB = await invoke('add_coating', {
                new: newCoating
            });

           await this.fetchCoatings();
           return createdCoating;
        } catch (err) {
            console.error('Erro ao criar Tratamento:', err);
        }
    },

    async editCoating(updatedCoating: CoatingDB, originalName: string) {
        try {
            const editedCoating: CoatingDB = await invoke('change_coating', {
                id: originalName,
                changes: {
                    coating_name: updatedCoating.coating_name, 
                }
            });

            await this.fetchCoatings();
            return editedCoating;
        } catch (err) {
            console.error('Erro ao editar tratamento:', err);
            throw err;
        }
    },



    async removeCoating(coating: DataTableCoating) {
        try {
            await invoke('remove_coating', {
                id: coating.coating_name
                    
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
