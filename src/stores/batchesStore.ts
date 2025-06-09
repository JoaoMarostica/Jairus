import { defineStore } from 'pinia';
import type { BatchDB, DataTableBatch } from '@/types/batches';
import type { DataTableRowKey } from 'naive-ui';
import { parseExpireDate } from '@/utils/parsing';
import { save } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
export const useBatchesStore = defineStore('batches', {
  state: () => ({
    batches: [] as BatchDB[],
    dataTableBatches: [] as DataTableBatch[],
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
    async removeAllBatches() {
        try {
            for (const batch of this.dataTableBatches) {
                await invoke('remove_batch', {
                    batchNumber: batch.batch_number,
                    batchYear: batch.batch_year
                });
            }

            this.selectedBatches = [];
            await this.fetchBatchesData();
        } catch (err) {
            console.error(err);
            throw err;
        }
    },
    getLastBatch(year: Number) {
        let lastBatchNumber = 0;

        const batchesNumber = this.dataTableBatches
            .filter(batch => (batch.batch_status === 1 && batch.batch_year === year))
            .map(batch => batch.batch_number);

        if (batchesNumber.length !== 0) {
            lastBatchNumber = Math.max(...batchesNumber);
        }

        return lastBatchNumber;
    },
    async generateReportPDF() {
        try {
            const filePath = await save({
                filters: [{ name: 'PDF', extensions: ['pdf'] }],
                defaultPath: 'relatorio_lotes.pdf'
            });

            if (!filePath) return;

            let selectedBatchesIds: any[] = [];

            if (this.selectedBatches.length === 0) {
                selectedBatchesIds = this.dataTableBatches.map((batch) => {
                    return [batch.batch_number, batch.batch_year];
                });
            } else {
                selectedBatchesIds = this.dataTableBatches
                    .filter((batch) =>
                        this.selectedBatches.includes(batch.key)
                    )
                    .map((batch) => [batch.batch_number, batch.batch_year]);
            }

            await invoke('generate_selected_batches_pdf', {
                selectedIds: selectedBatchesIds,
                path: filePath
            });

            return filePath;
        } catch (err) {
            console.error(err);
            throw err;
        }
    }
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
