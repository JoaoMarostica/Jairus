import { defineStore } from 'pinia';
import type { BrandDB, DataTableBrand} from '@/types/brands';
import { invoke } from '@tauri-apps/api/core';



export const useBrandsStore = defineStore('brands', {
  state: () => ({
    brands: [] as BrandDB[],
    dataTableBrands: [] as DataTableBrand[],
  }),
  actions: {
    async fetchBrands() {
        try {
            console.log('🔍 Iniciando fetchBrands...');
            this.$reset();

            console.log('🔍 Chamando API list_brands...');
            const brandsData = await invoke('list_brands');
            console.log('✅ Dados recebidos da API:', brandsData);
            
            this.brands = brandsData as BrandDB[];
            this.dataTableBrands = this.brands.map(brand => formatBrandForTable(brand));
            console.log('📊 dataTableBrands após formatação:', this.dataTableBrands);
        } catch (err) {
            console.error('❌ Erro em fetchBrands:', err);
        }
    },
    
    async createBrand(newBrand: BrandDB) {
        try {
            console.log('📝 Tentando criar marca:', newBrand);
            
            const createdBrand: BrandDB = await invoke('add_brand', {
                new: newBrand
            });
            console.log('✅ Marca criada com sucesso:', createdBrand);

            console.log('🔄 Recarregando marcas...');
            await this.fetchBrands();
            console.log('✅ Recarga concluída. Total de marcas:', this.brands.length);

            return createdBrand;
        } catch (err) {
            console.error('❌ Erro ao criar marca:', err);
            throw err; // Propagar o erro para tratamento adequado
        }
    },

    async editBrand(originalBrandName: string, updatedBrand: BrandDB) {
        try {
            console.log('🔄 Iniciando edição da marca', originalBrandName, '->', updatedBrand);
            
            // 1. Encontrar a marca original para comparar
            const originalBrand = this.brands.find(b => b.brand_name === originalBrandName);
            if (!originalBrand) {
                throw new Error(`Marca ${originalBrandName} não encontrada`);
            }
            
            // 2. Mudar o nome da marca (apenas se foi alterado)
            if (originalBrandName !== updatedBrand.brand_name) {
                console.log('✏️ Alterando nome da marca:', originalBrandName, '->', updatedBrand.brand_name);
                await invoke('change_brand', {
                    id: originalBrandName,
                    new_name: updatedBrand.brand_name,
                });
            }

            // 3. Calcular pesos a adicionar e remover
            const pesosAtuais = new Set(originalBrand.weights);
            const pesosNovos = new Set(updatedBrand.weights);
            
            // Pesos para adicionar (estão nos novos mas não nos atuais)
            const pesosParaAdicionar = updatedBrand.weights.filter(w => !pesosAtuais.has(w));
            
            // Pesos para remover (estão nos atuais mas não nos novos)
            const pesosParaRemover = originalBrand.weights.filter(w => !pesosNovos.has(w));
            
            // 4. Remover pesos antigos
            for (const peso of pesosParaRemover) {
                console.log('🗑️ Removendo peso:', peso, 'da marca:', updatedBrand.brand_name);
                await invoke('remove_brand_weight', {
                    id: updatedBrand.brand_name,
                    value: peso
                });
            }
            
            // 5. Adicionar novos pesos
            for (const peso of pesosParaAdicionar) {
                console.log('➕ Adicionando peso:', peso, 'à marca:', updatedBrand.brand_name);
                await invoke('add_brand_weight', {
                    id: updatedBrand.brand_name, 
                    value: peso
                });
            }
            
            console.log('✅ Atualizações concluídas, recarregando marcas...');
            // 6. Recarregar todas as marcas para garantir dados consistentes
            await this.fetchBrands();
            return this.brands.find(b => b.brand_name === updatedBrand.brand_name);
        } catch (err) {
            console.error('❌ Erro ao editar marca:', err);
            throw err;
        }
    },

    async removeBrand(brand: DataTableBrand) {
        try {
            await invoke('remove_brand', {
                id: brand.brand_name
            });

            await this.fetchBrands();
        } catch (err) {
            console.error(err);
            throw err;
        }
    },
    async addBrandWeight(brandName: string, weight: number) {
        try{
            const brandIndex = this.brands.findIndex(b => b.brand_name === brandName);
            if (brandIndex === -1){
                console.error(`Marca ${brandName} não encontrada`);

            }

            if( this.brands[brandIndex].weights.includes(weight)){
               console.error(`Peso ${weight}Kg já existe para a marca ${brandName}`);
            }

            await invoke('add_brand_weight', {
                id: brandName,
                value: weight
            });

            this.brands[brandIndex].weights.push(weight);
            const tableIndex = this.dataTableBrands.findIndex(b => b.key === brandName);
            if (tableIndex !== -1) {
                this.dataTableBrands[tableIndex] = formatBrandForTable(this.brands[brandIndex]);
            }
        } catch (err) {
            console.error('Erro ao adicionar peso ', err);
            throw err;
        }
    },
    async removeBrandWeight(brandName: string, weight: number) {
        try{
            const brandIndex = this.brands.findIndex(b => b.brand_name === brandName);
            if (brandIndex === -1){
                console.error(`Marca ${brandName} não encontrada`);

            }

            if(!this.brands[brandIndex].weights.includes(weight)){
                console.error(`Peso ${weight}Kg não existe para a marca ${brandName}`);
            }

            await invoke('remove_brand_weight', {
                id: brandName,
               value: weight
            });

            this.brands[brandIndex].weights = this.brands[brandIndex].weights.filter(w => w !== weight);
            const tableIndex = this.dataTableBrands.findIndex(b => b.key === brandName);
            if (tableIndex !== -1) {
                this.dataTableBrands[tableIndex] = formatBrandForTable(this.brands[brandIndex]);
            }
        } catch (err) {
            console.error('Erro ao remover peso ', err);
            throw err;
        }
    },

    
    
}});

function formatBrandForTable(brand: BrandDB): DataTableBrand {
    const brandForTable: DataTableBrand = {
        key: brand.brand_name,
        brand_name: brand.brand_name,
        weights: brand.weights.map(weight => ({
            label: weight.toString() + 'Kg',
            value: weight.toString()
        }))
   
    };

    return brandForTable
}
