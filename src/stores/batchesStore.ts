import { defineStore } from 'pinia';

type Batch = {
    number: number;
    year: number;
    expireDate: Date;
    seed: string;
    coating: string;
    sackBrand: string;
    sackQuantity: number;
    sackWeight: number;
    purenessScore: number;
    outflowPP: number;
    outflowKg: number;
    usage: string;
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
  }),
  getters: {
  },
  actions: {
    setBatches(batches: Batch[]) {
        const toFloat2 = (value: any) => Math.round(parseFloat(value) * 100) / 100
        
        batches.forEach((batch) => {
            const formatBatch: DataTableBatch = {
                key: batch.number,
                number: batch.number,
                year: batch.year,
                // Convert the expireDate to a MM/YYYY format
                expireDate: parseExpireDate(batch.expireDate),
                seed: batch.seed,
                coating: batch.coating,
                sackBrand: batch.sackBrand,
                sackQuantity: batch.sackQuantity,
                sackWeight: batch.sackWeight,
                availableQuantity: batch.sackQuantity * batch.sackWeight,
                purenessScore: batch.purenessScore,
                totalPP: toFloat2(batch.sackQuantity * batch.sackWeight * batch.purenessScore),
                _searchIndex: normalizeText(
                    [
                        batch.number,
                        batch.year,
                        parseExpireDate(batch.expireDate),
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

            this.batches.push(formatBatch);
            this.batchOutflows.push(batchOutflow);
        });
    },
    getBatchOutflowBy(batchNumber: number) {
        const batchOutflow = this.batchOutflows.find((batch) => batch.batchNumber === batchNumber);
        if (batchOutflow) {
            return batchOutflow;
        }
        return false;
    }
  }
});

function normalizeText(text: string | null): string {
  return text
    ? text.trim().toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, '')
    : '';
}

function parseExpireDate(expireDate: Date): string {
    const monthNumber = expireDate.getMonth() + 1;
    const year = expireDate.getFullYear();
    let month = "--"

    switch (monthNumber) {
        case 1:
            month = 'jan';
            break;
        case 2:
            month = 'fev';
            break;
        case 3:
            month = 'mar';
            break;
        case 4:
            month = 'abr';
            break;
        case 5:
            month = 'mai';
            break;
        case 6:
            month = 'jun';
            break;
        case 7:
            month = 'jul';
            break;
        case 8:
            month = 'ago';
            break;
        case 9:
            month = 'set';
            break;
        case 10:
            month = 'out';
            break;
        case 11:
            month = 'nov';
            break;
        case 12:
            month = 'dez';
            break;
        default:
            return 'formato inválido';
    }

    return `${month}/${year}`;
}
