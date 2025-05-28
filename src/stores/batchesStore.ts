import { defineStore } from 'pinia';
import type { DataTableRowKey } from 'naive-ui'
import { invoke } from '@tauri-apps/api/core'

type RawBatch = {
    number: number;
    year: number;
    expireDate: number;
    seed: string;
    coating: string;
    sackBrand: string;
    sackQuantity: number;
    sackWeight: number;
    purenessScore: number;
    outflowTotalPP: number;
    outflowKg: number;
    usage: string;
    status: string;
    deletedAt: number | null;
};

type Batch = {
    number: number;
    year: number;
    expireDate: number;
    seed: string;
    coating: string;
    sackBrand: string;
    sackQuantity: number;
    sackWeight: number;
    purenessScore: number;
    status: string;
    deletedAt: number | null;
};

type DataTableBatch = {
    key: number;
    number: number;
    year: number;
    expireDate: string;
    seed: string;
    coating: string;
    sackBrand: string;
    sackQuantity: number;
    sackWeight: number;
    availableQuantity: string;
    purenessScore: string;
    totalPP: string;
    status: string;
    deletedAt: number | null;
    _searchIndex: string;
};

type BatchOutflow = {
    batchNumber: number;
    batchYear: number;
    outflowTotalPP: number;
    outflowKg: number;
    outflowSack: number;
    usage: string;
};

type DataTableBatchOutflow = {
    batchNumber: number;
    batchYear: number;
    outflowTotalPP: string;
    outflowKg: string;
    outflowPP: string;
    outflowSack: number;
    usage: string;
    _searchIndex: string;
};

export const useBatchesStore = defineStore('batches', {
  state: () => ({
    batches: [] as Batch[],
    batchOutflows: [] as BatchOutflow[],
    dataTableBatches: [] as DataTableBatch[],
    dataTableBatchOutflows: [] as DataTableBatchOutflow[],
    batchesForDownload: [] as DataTableRowKey[],
  }),
  getters: {
  },
  actions: {
    setBatches(data: RawBatch[]) {
        const toFloat2 = (value: any) => Math.round(parseFloat(value) * 100) / 100;

        const existingBatchNumbers = new Set(this.dataTableBatches.map(b => b.number));

        data.forEach((batch) => {
            if (!existingBatchNumbers.has(batch.number)) {
                const formatBatch: Batch = {
                    number: batch.number,
                    year: batch.year,
                    expireDate: batch.expireDate,
                    seed: batch.seed,
                    coating: batch.coating,
                    sackBrand: batch.sackBrand,
                    sackQuantity: batch.sackQuantity,
                    sackWeight: batch.sackWeight,
                    purenessScore: batch.purenessScore,
                    status: batch.status,
                    deletedAt: batch.deletedAt,
                };
                this.batches.push(formatBatch);

                const formatBatchForDataTable: DataTableBatch = {
                    key: batch.number,
                    number: batch.number,
                    year: batch.year,
                    expireDate: parseExpireDate(batch.expireDate, batch.year),
                    seed: batch.seed,
                    coating: batch.coating,
                    sackBrand: batch.sackBrand,
                    sackQuantity: batch.sackQuantity,
                    sackWeight: batch.sackWeight,
                    availableQuantity: (batch.sackQuantity * batch.sackWeight).toLocaleString("pt-BR"),
                    purenessScore: batch.purenessScore.toLocaleString("pt-BR"),
                    totalPP: toFloat2(batch.sackQuantity * batch.sackWeight * batch.purenessScore).toLocaleString("pt-BR"),
                    status: batch.status,
                    deletedAt: batch.deletedAt,
                    _searchIndex: normalizeText(
                        [
                            batch.number,
                            batch.year,
                            parseExpireDate(batch.expireDate, batch.year),
                            batch.seed,
                            batch.coating,
                            batch.sackBrand,
                            batch.sackQuantity,
                            batch.sackWeight,
                            (batch.sackQuantity * batch.sackWeight).toLocaleString("pt-BR"),
                            batch.purenessScore,
                            toFloat2(batch.sackQuantity * batch.sackWeight * batch.purenessScore).toLocaleString("pt-BR"),
                        ].join(' ')
                    )
                };
                this.dataTableBatches.push(formatBatchForDataTable);
                existingBatchNumbers.add(batch.number);
            }

            const batchOutflow: DataTableBatchOutflow = {
                batchNumber: batch.number,
                batchYear: batch.year,
                outflowTotalPP: toFloat2(batch.outflowTotalPP).toLocaleString("pt-BR"),
                outflowKg: toFloat2(batch.outflowKg).toLocaleString("pt-BR"),
                outflowPP: toFloat2(batch.outflowTotalPP / batch.outflowKg).toLocaleString("pt-BR"),
                outflowSack: toFloat2(batch.outflowKg / batch.sackWeight),
                usage: batch.usage,
                _searchIndex: normalizeText(
                    [
                        batch.number,
                        toFloat2(batch.outflowTotalPP).toLocaleString("pt-BR"),
                        toFloat2(batch.outflowKg).toLocaleString("pt-BR"),
                        toFloat2(batch.outflowTotalPP / batch.outflowKg).toLocaleString("pt-BR"),
                        toFloat2(batch.outflowKg / batch.sackWeight),
                        batch.usage
                    ].join(' ')
                )
            };
            this.dataTableBatchOutflows.push(batchOutflow);
        });

        // send data to backend
    },
    async getBatchOutflow(batchNumber: number, batchYear: number) {
        return this.dataTableBatchOutflows
            .filter(batch => batch.batchNumber === batchNumber && batch.batchYear === batchYear)
            .map(batchOutflow => ({
                outflowTotalPP: batchOutflow.outflowTotalPP,
                outflowKg: batchOutflow.outflowKg,
                outflowPP: batchOutflow.outflowPP,
                outflowSack: batchOutflow.outflowSack,
                usage: batchOutflow.usage,
            }));
    },
    async getBatchBalance(dataTableBatch: DataTableBatch, batchOutflows: any[]) {
        const batch = this.batches.find(batch => batch.number === dataTableBatch.number && batch.year === dataTableBatch.year);

        let totalBatchOutflowPP = 0;
        let totalBatchOutflowKg = 0;
        let totalBatchOutflowSack = 0;

        batchOutflows.forEach((batchOutflow) => {
            totalBatchOutflowPP += batchOutflow.outflowPP;
            totalBatchOutflowKg += batchOutflow.outflowKg;
            totalBatchOutflowSack += batchOutflow.outflowSack;
        });

        // Calculate totalPP and availableQuantity from batch properties
        const totalPP = batch ? batch.sackQuantity * batch.sackWeight * batch.purenessScore : 0;
        const availableQuantity = batch ? batch.sackQuantity * batch.sackWeight : 0;
        const sackQuantity = batch ? batch.sackQuantity : 0;

        const balancePP = totalPP - totalBatchOutflowPP;
        const balanceKg = availableQuantity - totalBatchOutflowKg;
        const balanceSack = sackQuantity - totalBatchOutflowSack;

        return [
            { value: parseFloat(Math.max(balancePP, 0).toFixed(2)), name: 'Ponto de Pureza (PP)' },
            { value: parseFloat(Math.max(balanceKg, 0).toFixed(2)), name: 'Quantidade (Kg)' },
            { value: parseFloat(Math.max(balanceSack, 0).toFixed(2)), name: 'Sacos' }
        ];
    },
    async downloadPdf() {
        let downloadData: any[] = [];

        if (this.batchesForDownload.length === 0) {
            downloadData = this.dataTableBatches.map((batch) => {
                return {
                    number: batch.number,
                    year: batch.year,
                    expireDate: batch.expireDate,
                    seed: batch.seed,
                    coating: batch.coating,
                    sackBrand: batch.sackBrand,
                    sackQuantity: batch.sackQuantity,
                    sackWeight: batch.sackWeight,
                    availableQuantity: batch.availableQuantity,
                };
            })
        } else {
            downloadData = this.dataTableBatches.map((batch) => {
                if (this.batchesForDownload.some((b: any) => b.number === batch.number)) {
                    return {
                        number: batch.number,
                        year: batch.year,
                        expireDate: batch.expireDate,
                        seed: batch.seed,
                        coating: batch.coating,
                        sackBrand: batch.sackBrand,
                        sackQuantity: batch.sackQuantity,
                        sackWeight: batch.sackWeight,
                        availableQuantity: batch.availableQuantity,
                    };
                }
            })
        }
        console.log('Download data: ', downloadData);
    }
  }
});

function normalizeText(text: string | null): string {
  return text
    ? text.trim().toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, '')
    : '';
}

function parseExpireDate(expireDate: number, year: number): string {
  const monthNames = ['jan', 'fev', 'mar', 'abr', 'mai', 'jun', 'jul', 'ago', 'set', 'out', 'nov', 'dez'];
  const month = monthNames[expireDate] || '--';
  return `${month}/${year + 1}`;
}
