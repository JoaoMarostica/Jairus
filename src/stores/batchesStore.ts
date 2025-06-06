import { defineStore } from 'pinia';
import type { BatchDB, DataTableBatch } from '@/types/batches';
import type { BalanceDB, DataTableBalanceOutflow } from '@/types/balance';
import type { DataTableRowKey } from 'naive-ui';
import { parseExpireDate } from '@/utils/parsing';
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

export const useBatchesStore = defineStore('batches', {
  state: () => ({
    batches: ref<BatchDB[]>([]),
    dataTableBatches: ref<DataTableBatch[]>([]),
    selectedBatches: [] as DataTableRowKey[],
  }),
  actions: {
    async fetchBatchesData() {
        try {
            this.batches = await invoke('list_batches');
            
            this.dataTableBatches = this.batches.map(formatBatchForTable);
        } catch (err) {
            console.error(err);
            throw err;
        }
    },
    async createBatch(newBatch: BatchDB) {
        try {
            const createdBatch: BatchDB = await invoke('add_batch', {
                batch: newBatch
            });

            this.batches.push(createdBatch);
            this.dataTableBatches.push(formatBatchForTable(createdBatch));
        } catch (err) {
            console.error(err);
            throw err;
        }
    },
    async editBatch(batch: BatchDB) {
        try {
            const editedBatch: BatchDB = await invoke('change_batch', {
                batchNumber: batch.batch_number,
                batchYear: batch.batch_year,
                batch: batch
            });

            const index = this.dataTableBatches.findIndex(b => b.batch_number === batch.batch_number && b.batch_year === batch.batch_year);

            if (index !== -1) {
                this.dataTableBatches[index] = formatBatchForTable(editedBatch);
            }

            this.batches = await invoke('list_batches');
        } catch (err) {
            console.error(err);
            throw err;
        }
    },
    async removeBatch(batch: DataTableBatch) {
        try {
            await invoke('remove_batch', {
                batchNumber: batch.batch_number,
                batchYear: batch.batch_year
            });

            await this.fetchBatchesData();
        } catch (err) {
            console.error(err);
            throw err;
        }
    },
    async removeSelectedBatches() {
        try {
            this.selectedBatches.forEach(async (batchKey) => {
                const batch = this.dataTableBatches.find(batch => batch.key === batchKey);
                if (batch) {
                    await invoke('remove_batch', {
                        batchNumber: batch.batch_number,
                        batchYear: batch.batch_year
                    });
                }
            });
            this.selectedBatches = [];

            await this.fetchBatchesData();
        } catch (err) {
            console.error(err);
            throw err;
        }
    },
    getLastBatch() {
        const batchesNumber = this.dataTableBatches
            .filter(batch => batch.batch_status === 1)
            .map(batch => batch.batch_number);

        const lastBatchNumber = Math.max(...batchesNumber);

        return lastBatchNumber;
    },
    async getBatchBalance(batchNumber: number, batchYear: number): Promise<DataTableBalanceOutflow> {
        try {
            const outflowTotals: BalanceDB = await invoke('get_total_outflow', {
                batchNumber: batchNumber,
                batchYear: batchYear
            })

            const batch = this.batches.find(batch => batch.batch_number === batchNumber && batch.batch_year === batchYear);

            if (!batch) {
                throw new Error('Batch not found');
            }

            // Calculate totalPP and totalWeight from batch properties
            const totalPP = batch?.total_pureness_score;
            const totalWeight = batch?.total_weight;
            const sackAmount = batch?.sack_amount;

            const balancePP = totalPP - outflowTotals.total_pureness_score;
            const balanceWeight = totalWeight - outflowTotals.total_weight;
            const balanceSackAmount = sackAmount - outflowTotals.sack_amount;

            const balance: DataTableBalanceOutflow = {
                key: createDataTableKey(batchNumber, batchYear),
                sack_amount: Math.max(balanceSackAmount, 0),
                total_weight: parseFloat(Math.max(balanceWeight, 0).toFixed(2)).toLocaleString("pt-BR"),
                total_pureness_score: parseFloat(Math.max(balancePP, 0).toFixed(2)).toLocaleString("pt-BR"),
                _searchIndex: normalizeText([
                    batchNumber,
                    batchYear,
                    (balanceSackAmount).toString(),
                    parseFloat(Math.max(balanceWeight, 0).toFixed(2)).toLocaleString("pt-BR"),
                    parseFloat(Math.max(balancePP, 0).toFixed(2)).toLocaleString("pt-BR")
                ].join(' '))
            }

            return balance
        } catch (err) {
            console.error(err);
            throw err;
        }
    },
    async downloadPdf() {
        let downloadData: any[] = [];

        if (this.selectedBatches.length === 0) {
            downloadData = this.dataTableBatches.map((batch) => {
                return {
                    number: batch.batch_number,
                    year: batch.batch_year,
                    expireDate: batch.expire_date,
                    seed: batch.seed,
                    coating: batch.coating,
                    sackBrand: batch.brand,
                    sackAmount: batch.sack_amount,
                    sackWeight: batch.sack_weight,
                    totalWeight: batch.total_weight,
                };
            })
        } else {
            downloadData = this.dataTableBatches.map((batch) => {
                if (this.selectedBatches.some((b: any) => b.number === batch.batch_number)) {
                    return {
                        number: batch.batch_number,
                        year: batch.batch_year,
                        expireDate: batch.expire_date,
                        seed: batch.seed,
                        coating: batch.coating,
                        sackBrand: batch.brand,
                        sackAmount: batch.sack_amount,
                        sackWeight: batch.sack_weight,
                        totalWeight: batch.total_weight,
                    };
                }
            })
        }
        console.log('Download data: ', downloadData);
    },
  },
  getters: {
    getBatchKeys: (state) => {
        return state.dataTableBatches
            .filter(batch => batch.batch_status === 1)
            .map(batch => batch.key)
    }
  },
});

function createDataTableKey(batchNumber: number, batchYear: number): string {
    return `${batchNumber}${batchYear}`
}

function normalizeText(text: string | null): string {
  return text
    ? text.trim().toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, '')
    : '';
}

function formatBatchForTable(batch: BatchDB): DataTableBatch {
    const batchForTable: DataTableBatch = {
        key: createDataTableKey(batch.batch_number, batch.batch_year),
        batch_number: batch.batch_number,
        batch_year: batch.batch_year,
        expire_date: parseExpireDate(batch.batch_month, batch.batch_year),
        seed: batch.seed,
        coating: batch.coating,
        brand: batch.brand,
        sack_weight: batch.sack_weight,
        sack_amount: batch.sack_amount,
        total_weight: batch.total_weight.toLocaleString("pt-BR"),
        pureness_score: batch.pureness_score.toLocaleString("pt-BR"),
        total_pureness_score: batch.total_pureness_score.toLocaleString("pt-BR"),
        batch_status: 1,
        deleted_at: null,
        _searchIndex: normalizeText([
            batch.batch_number,
            batch.batch_year,
            parseExpireDate(batch.batch_month, batch.batch_year),
            batch.seed,
            batch.coating,
            batch.brand,
            batch.sack_weight,
            batch.sack_amount,
            batch.total_weight.toLocaleString("pt-BR"),
            batch.pureness_score.toLocaleString("pt-BR"),
            batch.total_pureness_score.toLocaleString("pt-BR"),
        ].join(' '))
    };

    return batchForTable
}
