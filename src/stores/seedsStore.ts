import { defineStore } from 'pinia';
import type { SeedDB, DataTableSeed } from '@/types/seeds';
import { invoke } from '@tauri-apps/api/core';

export const useSeedsStore = defineStore('seeds', {
  state: () => ({
    seeds: [] as SeedDB[],
    dataTableSeeds: [] as DataTableSeed[],
  }),
  actions: {
    async fetchSeed() {
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
    async createSeed(newSeed: SeedDB) {
        try {
            const createdSeed: SeedDB = await invoke('create_seed', {
                seed: newSeed
            });

           this.dataTableSeeds.push(formatSeedForTable(createdSeed));
        } catch (err) {
            console.error('Erro ao criar Cultivar:', err);
        }
    },
    
});

function normalizeText(text: string | null): string {
  return text
    ? text.trim().toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, '')
    : '';
}

function formatSeedForTable(seed: SeedDB): DataTableSeed {
    const seedForTable: DataTableSeed = {
        key: seed.popular_name;
        popular_name: seed.popular_name;
        scientific_name: seed.scientific_name;
        deleted_at: null;
   
        _searchIndex: normalizeText([
          seed.popular_name,
          seed.scientific_name
        ].join(' '))
    };

    return seedForTable
}
