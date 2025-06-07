import { defineStore } from 'pinia';
import type { BatchOutflowDB, DataTableBatchOutflow } from '@/types/batches';
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

export const useOutflowsStore = defineStore('outflows', {
  state: () => ({
    batchOutflows: ref<BatchOutflowDB[]>([]),
    dataTableBatchOutflows: ref<DataTableBatchOutflow[]>([]),
  }),
  actions: {
    async fetchOutflowsData() {
        try {
            this.batchOutflows = await invoke('list_outflows');
            
            this.dataTableBatchOutflows = this.batchOutflows.map(formatOutflowForTable);
        } catch (err) {
            console.error(err);
            throw err;
        }
    },
    async createOutflow(newOutflow: any) {
        try {
            const createdOutflow: BatchOutflowDB = await invoke('add_outflow', {
                new: newOutflow
            });

            this.batchOutflows.push(createdOutflow);
            this.dataTableBatchOutflows.push(formatOutflowForTable(createdOutflow));
        } catch (err) {
            console.error(err);
            throw err;
        }
    },
    async editOutflow(outflow: any) {
        try {
            console.log(outflow);
            
            const editedOutflow: BatchOutflowDB = await invoke('change_outflow', {
                id: outflow.outflow_id,
                changes: outflow
            });

            const index = this.dataTableBatchOutflows.findIndex(o => o.outflow_id === outflow.outflow_id);

            if (index !== -1) {
                this.dataTableBatchOutflows[index] = formatOutflowForTable(editedOutflow);
            }

            this.batchOutflows = await invoke('list_outflows');
        } catch (err) {
            console.error(err);
            throw err;
        }
    },
    async removeOutflow(outflow: DataTableBatchOutflow) {
        try {
            await invoke('remove_outflow', {
                id: outflow.outflow_id,
            });

            await this.fetchOutflowsData();
        } catch (err) {
            console.error(err);
            throw err;
        }
    },
    async getBatchOutflows(batchNumber: number, batchYear: number): Promise<DataTableBatchOutflow[]> {
        try {
            const outflows: BatchOutflowDB[] = await invoke('list_outflows_by_batch', {
                batchNumber: batchNumber,
                batchYear: batchYear
            })

            return outflows.map(formatOutflowForTable);
        } catch (err) {
            console.error(err);
            throw err;
        }
    },
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

function formatOutflowForTable(batchOutflow: BatchOutflowDB): DataTableBatchOutflow {
    const batchOutflowForTable: DataTableBatchOutflow = {
        key: createDataTableKey(batchOutflow.batch_number, batchOutflow.batch_year),
        outflow_id: batchOutflow.outflow_id,
        batch_number: batchOutflow.batch_number,
        batch_year: batchOutflow.batch_year,
        sack_amount: batchOutflow.sack_amount,
        total_weight: batchOutflow.total_weight.toLocaleString("pt-BR"),
        total_pureness_score: batchOutflow.total_pureness_score.toLocaleString("pt-BR"),
        pureness_score: (batchOutflow.total_pureness_score / batchOutflow.total_weight).toLocaleString("pt-BR"),
        usage: batchOutflow.usage,
        _searchIndex: normalizeText([
            batchOutflow.batch_number,
            batchOutflow.batch_year,
            batchOutflow.sack_amount,
            batchOutflow.total_weight.toLocaleString("pt-BR"),
            batchOutflow.total_pureness_score.toLocaleString("pt-BR"), 
            (batchOutflow.total_pureness_score / batchOutflow.total_weight).toLocaleString("pt-BR"),
            batchOutflow.usage 
        ].join(' '))
    };

    return batchOutflowForTable
}
