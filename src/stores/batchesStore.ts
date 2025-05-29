import { defineStore } from 'pinia';
import type { DataTableRowKey } from 'naive-ui'
import { invoke } from '@tauri-apps/api/core'

type RawBatch = {
    batch_number: number;
    batch_year: number;
    batch_month: number;
    seed: string;
    coating: string;
    brand: string;
    sack_weight: number;
    sack_amount: number;
    total_weight: number;
    pureness_score: number;
    total_pureness_score: number;
    outflow_total_pureness_score: number;
    outflow_total_weight: number;
    usage: string;
    batch_status: string;
    deleted_at: number | null;
    origin: string | null;
};

type BatchDB = {
    batch_number: number;
    batch_year: number;
    batch_month: number;
    seed: string;
    coating: string;
    brand: string;
    sack_weight: number;
    sack_amount: number;
    total_weight: number;
    pureness_score: number;
    total_pureness_score: number;
    batch_status: string;
    deleted_at: number | null;
    origin: string | null;
};

type BatchOutflowDB = {
    batch_number: number;
    batch_year: number;
    sack_amount: number;
    total_weight: number;
    total_pureness_score: number;
    usage: string;
};

type DataTableBatch = {
    key: string;
    batch_number: number;
    batch_year: number;
    expire_date: string;
    seed: string;
    coating: string;
    brand: string;
    sack_weight: number;
    sack_amount: number;
    total_weight: string;
    pureness_score: string;
    total_pureness_score: string;
    batch_status: string;
    deleted_at: number | null;
    _searchIndex: string;
};

type DataTableBatchOutflow = {
    batch_number: number;
    batch_year: number;
    sack_amount: number;
    total_weight: string;
    total_pureness_score: string;
    pureness_score: string;
    usage: string;
    _searchIndex: string;
};

export const useBatchesStore = defineStore('batches', {
  state: () => ({
    batches: [] as BatchDB[],
    batchOutflows: [] as BatchOutflowDB[],
    dataTableBatches: [] as DataTableBatch[],
    dataTableBatchOutflows: [] as DataTableBatchOutflow[],
    batchesForDownload: [] as DataTableRowKey[],
  }),
  getters: {
  },
  actions: {
    setBatches(data: RawBatch[]) {
        const toFloat2 = (value: any) => Math.round(parseFloat(value) * 100) / 100;

        const existingBatchNumbers = new Set(this.dataTableBatches.map(b => b.batch_number));

        data.forEach((batch) => {
            if (!existingBatchNumbers.has(batch.batch_number)) {
                // Lotes

                // Para enviar para o DB
                const formatBatchForDB: BatchDB = {
                    batch_number: batch.batch_number,
                    batch_year: batch.batch_year,
                    batch_month: batch.batch_month,
                    seed: batch.seed,
                    coating: batch.coating,
                    brand: batch.brand,
                    sack_weight: batch.sack_weight,
                    sack_amount: batch.sack_amount,
                    total_weight: batch.total_weight,
                    pureness_score: batch.pureness_score,
                    total_pureness_score: batch.total_pureness_score,
                    batch_status: batch.batch_status,
                    deleted_at: batch.deleted_at,
                    origin: batch.origin,
                };
                this.batches.push(formatBatchForDB);

                // Para enviar para a tabela no front
                const formatBatchForDataTable: DataTableBatch = {
                    key: createDataTableBatchKey(batch),
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
                    batch_status: batch.batch_status,
                    deleted_at: batch.deleted_at,
                    _searchIndex: normalizeText(
                        [
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
                            batch.batch_status
                        ].join(' ')
                    )
                };
                this.dataTableBatches.push(formatBatchForDataTable);
                existingBatchNumbers.add(batch.batch_number);

                // Enviando para o DB
                invoke('create_batch', {
                    batch: formatBatchForDB
                }).then((res) => {
                    console.log(res);
                }).catch((err) => {
                    console.error('Erro ao invocar comando:', err);
                });
            }

            // Saídas

            // Para enviar para o DB
            const batchOutflowForDB: BatchOutflowDB = {
                batch_number: batch.batch_number,
                batch_year: batch.batch_year,
                sack_amount: batch.sack_amount,
                total_weight: batch.total_weight,
                total_pureness_score: batch.total_pureness_score,
                usage: batch.usage,
            };
            this.batchOutflows.push(batchOutflowForDB);

            // Para enviar para a tela de detalhes do lote
            const batchOutflowForDataTable: DataTableBatchOutflow = {
                batch_number: batch.batch_number,
                batch_year: batch.batch_year,
                sack_amount: batch.sack_amount,
                total_weight: batch.outflow_total_weight.toLocaleString("pt-BR"),
                total_pureness_score: batch.outflow_total_pureness_score.toLocaleString("pt-BR"),
                pureness_score: toFloat2(batch.outflow_total_pureness_score / batch.outflow_total_weight).toLocaleString("pt-BR"),
                usage: batch.usage,
                _searchIndex: normalizeText(
                    [
                        batch.batch_number,
                        batch.batch_year,
                        batch.sack_amount,
                        batch.outflow_total_weight.toLocaleString("pt-BR"),
                        batch.outflow_total_pureness_score.toLocaleString("pt-BR"),
                        toFloat2(batch.outflow_total_pureness_score / batch.outflow_total_weight).toLocaleString("pt-BR"),
                        batch.usage
                    ].join(' ')
                )
            };
            this.dataTableBatchOutflows.push(batchOutflowForDataTable);
        });
    },
    async getBatchOutflow(batchNumber: number, batchYear: number) {
        return this.dataTableBatchOutflows
            .filter(batch => batch.batch_number === batchNumber && batch.batch_year === batchYear)
            .map(batchOutflow => ({
                outflowTotalPP: batchOutflow.total_pureness_score,
                outflowTotalWeight: batchOutflow.total_weight,
                outflowPP: batchOutflow.pureness_score,
                outflowSackAmount: batchOutflow.sack_amount,
                usage: batchOutflow.usage,
            }));
    },
    async getBatchBalance(dataTableBatch: DataTableBatch, batchOutflows: any[]) {
        const batch = this.batches.find(batch => batch.batch_number === dataTableBatch.batch_number && batch.batch_year === dataTableBatch.batch_year);

        let BatchOutflowTotalPP = 0;
        let BatchOutflowTotalWeight = 0;
        let BatchOutflowTotalSackAmount = 0;

        batchOutflows.forEach((batchOutflow) => {
            BatchOutflowTotalPP += batchOutflow.outflowTotalPP;
            BatchOutflowTotalWeight += batchOutflow.outflowTotalWeight;
            BatchOutflowTotalSackAmount += batchOutflow.outflowSackAmount;
        });

        // Calculate totalPP and totalWeight from batch properties
        const totalPP = batch ? batch.sack_amount * batch.sack_weight * batch.pureness_score : 0;
        const totalWeight = batch ? batch.sack_amount * batch.sack_weight : 0;
        const sackAmount = batch ? batch.sack_amount : 0;

        const balancePP = totalPP - BatchOutflowTotalPP;
        const balanceWeight = totalWeight - BatchOutflowTotalWeight;
        const balanceSackAmount = sackAmount - BatchOutflowTotalSackAmount;

        return [
            { value: parseFloat(Math.max(balancePP, 0).toFixed(2)), name: 'Ponto de Pureza (PP)' },
            { value: parseFloat(Math.max(balanceWeight, 0).toFixed(2)), name: 'Quantidade (Kg)' },
            { value: parseFloat(Math.max(balanceSackAmount, 0).toFixed(2)), name: 'Sacos' }
        ];
    },
    async downloadPdf() {
        let downloadData: any[] = [];

        if (this.batchesForDownload.length === 0) {
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
                if (this.batchesForDownload.some((b: any) => b.number === batch.batch_number)) {
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
    }
  }
});

function createDataTableBatchKey(batch: RawBatch): string {
    return `${batch.batch_number}${batch.batch_year}`
}

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
