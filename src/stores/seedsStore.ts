import { defineStore } from 'pinia';
import type { SeedDB, DataTableSeed, RawSeed } from '@/types/seeds';
import { invoke } from '@tauri-apps/api/core';
import cultivarsInfo from '@/assets/cultivarsInfo.json';

export const useSeedsStore = defineStore('seeds', {
  state: () => ({
    seeds: [] as SeedDB[],
    dataTableSeeds: [] as DataTableSeed[],
    jsonMigration: false,
  }),
  actions: {
    async fetchSeeds() {
        try {
            
            
            this.seeds = [];
            this.dataTableSeeds = [];
              
            this.seeds = await invoke('list_seeds');
            
            this.seeds.forEach((seed: SeedDB) =>
                this.dataTableSeeds.push(formatSeedForTable(seed))
            );

            // Migração automática na primeira carga
            if (!this.jsonMigration && this.seeds.length === 0) {
                await this.migrateJsonToDatabase();
                this.jsonMigration= true;
                this.seeds = await invoke('list_seeds');
                this.dataTableSeeds = [];
                this.seeds.forEach((seed: SeedDB) =>
                this.dataTableSeeds.push(formatSeedForTable(seed))
                );
            }

        } catch (err) {
            console.error(err);
        }
    },
    async migrateJsonToDatabase() {
        try {
            console.log('Iniciando migração dos seeds do JSON para o banco...');
            
            let migratedCount = 0;
            let skippedCount = 0;
            
            for (const jsonSeed of cultivarsInfo) {
                const seedForDB = formatSeedForDB(jsonSeed);
                
                // Verifica se já existe no banco
                const existsInDB = this.seeds.find(
                    s => s.popular_name.toLowerCase() === seedForDB.popular_name.toLowerCase()
                );
                
                if (!existsInDB) {
                    try {
                        await invoke('add_seed', {
                            new: seedForDB
                        });
                        migratedCount++;
                        console.log(`✅ Migrado: ${seedForDB.popular_name}`);
                    } catch (error) {
                        console.error(`❌ Erro ao migrar ${seedForDB.popular_name}:`, error);
                    }
                } else {
                    skippedCount++;
                    console.log(`⏭️ Já existe: ${seedForDB.popular_name}`);
                }
            }
            
            // Atualiza a lista após migração
            await this.fetchSeeds();
            
            console.log(`🎉 Migração concluída: ${migratedCount} adicionados, ${skippedCount} já existiam`);
            return { migratedCount, skippedCount };
            
        } catch (err) {
            console.error('Erro na migração:', err);
            throw err;
        }
    },


    async createSeed(newSeed: SeedDB) {
        try {
            const existingSeed = this.seeds.find(
            s => s.popular_name.toLowerCase() === newSeed.popular_name.toLowerCase()
            );
            if (existingSeed) {
                throw new Error('Cultivar já existe');
            }

            const createdSeed: SeedDB = await invoke('add_seed', {
                new: newSeed
            });
            

            await this.fetchSeeds();

            return createdSeed;
        } catch (err) {
            console.error('Erro ao criar Cultivar:', err);
            throw err;
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
