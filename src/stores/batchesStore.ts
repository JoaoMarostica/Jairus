import { defineStore } from 'pinia';
import type { DataTableRowKey } from 'naive-ui'

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
    outflowPP: number;
    outflowKg: number;
    usage: string;
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
    availableQuantity: number;
    purenessScore: number;
    totalPP: number;
    status: string;
    deletedAt: number | null;
    _searchIndex: string;
};

type BatchOutflow = {
    batchNumber: number;
    outflowPP: number;
    outflowKg: number;
    outflowSack: number;
    usage: string;
    _searchIndex: string;
};

export const useBatchesStore = defineStore('batches', {
  state: () => ({
    batches: [] as DataTableBatch[],
    batchOutflows: [] as BatchOutflow[],
    batchesForDownload: [] as DataTableRowKey[],
  }),
  getters: {
  },
  actions: {
    setBatches(batches: Batch[]) {
        const toFloat2 = (value: any) => Math.round(parseFloat(value) * 100) / 100;

        const existingBatchNumbers = new Set(this.batches.map(b => b.number));

        batches.forEach((batch) => {
            if (!existingBatchNumbers.has(batch.number)) {
                const formatBatch: DataTableBatch = {
                    key: batch.number,
                    number: batch.number,
                    year: batch.year,
                    expireDate: parseExpireDate(batch.expireDate, batch.year),
                    seed: batch.seed,
                    coating: batch.coating,
                    sackBrand: batch.sackBrand,
                    sackQuantity: batch.sackQuantity,
                    sackWeight: batch.sackWeight,
                    availableQuantity: batch.sackQuantity * batch.sackWeight,
                    purenessScore: batch.purenessScore,
                    totalPP: toFloat2(batch.sackQuantity * batch.sackWeight * batch.purenessScore),
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
                            batch.purenessScore,
                            batch.usage
                        ].join(' ')
                    )
                };
                this.batches.push(formatBatch);
                existingBatchNumbers.add(batch.number);
            }

            const batchOutflow: BatchOutflow = {
                batchNumber: batch.number,
                outflowPP: toFloat2(batch.outflowPP),
                outflowKg: toFloat2(batch.outflowKg),
                outflowSack: toFloat2(batch.outflowKg / batch.sackWeight),
                usage: batch.usage,
                _searchIndex: normalizeText(
                    [
                        batch.number,
                        toFloat2(batch.outflowPP),
                        toFloat2(batch.outflowKg),
                        toFloat2(batch.outflowKg / batch.sackWeight),
                        batch.usage
                    ].join(' ')
                )
            };
            this.batchOutflows.push(batchOutflow);
        });
    },
    async getBatchOutflow(batchNumber: number) {
        return this.batchOutflows
            .filter(batch => batch.batchNumber === batchNumber)
            .map(batchOutflow => ({
                outflowPP: batchOutflow.outflowPP,
                outflowKg: batchOutflow.outflowKg,
                outflowSack: batchOutflow.outflowSack,
                usage: batchOutflow.usage,
            }));
    },
    async getBatchBalance(batch: DataTableBatch, batchOutflows: any[]) {
        let totalBatchOutflowPP = 0;
        let totalBatchOutflowKg = 0;
        let totalBatchOutflowSack = 0;

        batchOutflows.forEach((batchOutflow) => {
            totalBatchOutflowPP += batchOutflow.outflowPP;
            totalBatchOutflowKg += batchOutflow.outflowKg;
            totalBatchOutflowSack += batchOutflow.outflowSack;
        });

        const balancePP = batch.totalPP - totalBatchOutflowPP;
        const balanceKg = batch.availableQuantity - totalBatchOutflowKg;
        const balanceSack = batch.sackQuantity - totalBatchOutflowSack;

        return [
            { value: parseFloat(Math.max(balancePP, 0).toFixed(2)), name: 'Ponto de Pureza (PP)' },
            { value: parseFloat(Math.max(balanceKg, 0).toFixed(2)), name: 'Quantidade (Kg)' },
            { value: parseFloat(Math.max(balanceSack, 0).toFixed(2)), name: 'Sacos' }
        ];
    },
    async downloadPdf() {
        let downloadData: any[] = [];

        if (this.batchesForDownload.length === 0) {
            downloadData = this.batches.map((batch) => {
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
            downloadData = this.batches.map((batch) => {
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
