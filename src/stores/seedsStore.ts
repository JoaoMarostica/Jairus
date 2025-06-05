import { defineStore } from 'pinia';
import type { SeedDB, DataTableSeed, RawSeed } from '@/types/seeds';
import { invoke } from '@tauri-apps/api/core';
import cultivarsInfo from '@/assets/cultivarsInfo.json';

export const useSeedsStore = defineStore('seeds', {
  state: () => ({
    seeds: [] as SeedDB[],
    dataTableSeeds: [] as DataTableSeed[],
  }),
  actions: {
    async fetchSeeds() {
        try {
            this.$reset;

            this.seeds = await invoke('list_seeds');

            this.seeds.forEach((seed: SeedDB) =>
                this.dataTableSeeds.push(formatSeedForTable(seed))
            );
        } catch (err) {
            console.error(err);
        }
    },
    fetchSeedsFromJsonFile() {
      for (const seed of cultivarsInfo) {
            this.seeds.push(formatSeedForDB(seed));
      }
    },
    async createSeed(newSeed: SeedDB) {
        try {
            const createdSeed: SeedDB = await invoke('add_seed', {
                seed: newSeed
            });

           this.dataTableSeeds.push(formatSeedForTable(createdSeed));
        } catch (err) {
            console.error('Erro ao criar Cultivar:', err);
        }
    },

    async editSeed(seed: SeedDB) {
            try {
                const editedSeed: SeedDB = await invoke('change_seed', {
                    seedName: seed.popular_name,
                    scientificName: seed.scientific_name
                });
    
                const index = this.dataTableSeeds.findIndex(s => s.key === seed.popular_name);
    
                if (index !== -1) {
                    this.dataTableSeeds[index] = formatSeedForTable(editedSeed);
                }
    
                this.seeds = await invoke('list_seeds');
            } catch (err) {
                console.error(err);
                throw err;
            }
        },
        async removeSeed(seed: DataTableSeed) {
            try {
                await invoke('remove_seed', {
                    seedName: seed.popular_name
                });
    
                await this.fetchSeeds();
            } catch (err) {
                console.error(err);
                throw err;
            }
        },
    
    
}});

function normalizeText(text: string | null): string {
  return text
    ? text.trim().toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, '')
    : '';
}

function formatSeedForDB(seed: RawSeed): SeedDB {
    const seedForDB: SeedDB = {
        popular_name: seed.name,
        scientific_name: seed.scientific_name
        
    };

    return seedForDB
}

function formatSeedForTable(seed: SeedDB): DataTableSeed {
    const seedForTable: DataTableSeed = {
        key: seed.popular_name,
        popular_name: seed.popular_name,
        scientific_name: seed.scientific_name,
        deleted_at: null,
   
        _searchIndex: normalizeText([
          seed.popular_name,
          seed.scientific_name
        ].join(' '))
    };

    return seedForTable
}
