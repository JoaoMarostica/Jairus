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
            this.$reset;

            this.brands = await invoke('list_brands');

            this.brands.forEach((brand: BrandDB) =>
                this.dataTableBrands.push(formatBrandForTable(brand))
            );
        } catch (err) {
            console.error(err);
        }
    },
    
    async createBrand(newBrand: BrandDB) {
        try {
            const createdBrand: BrandDB = await invoke('add_brand', {
                new: newBrand
            });

           this.dataTableBrands.push(formatBrandForTable(createdBrand));
        } catch (err) {
            console.error('Erro ao criar Marca:', err);
        }
    },

    async editBrand(originalBrandName: string, updatedBrand: BrandDB) {
        try {
            const editedBrand: BrandDB = await invoke('change_brand', {
                brandName: originalBrandName,
                new_name: updatedBrand.brand_name,
            });

            const brandIndex = this.brands.findIndex(b => b.brand_name === originalBrandName);
            if (brandIndex !== -1) {
                this.brands[brandIndex] = editedBrand;
            }

            const index = this.dataTableBrands.findIndex(b => b.key === originalBrandName);

            if (index !== -1) {
                this.dataTableBrands[index] = formatBrandForTable(editedBrand);
            }

            this.brands = await invoke('list_brands');
        } catch (err) {
            console.error(err);
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
