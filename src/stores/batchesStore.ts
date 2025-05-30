import { defineStore } from 'pinia';
import type { DataTableRowKey } from 'naive-ui';
import ExcelJS from 'exceljs';
import { invoke } from '@tauri-apps/api/core';

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
    batch_status: number;
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
    batch_status: number;
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
    async importBatchesFromExcel(file: File) {
        const batchesAux = []
        const batchOutflowsAux = []
        const dataTableBatchesAux = []
        const dataTableBatchOutflowsAux = []

        const buffer = await file.arrayBuffer();
        const workbook = new ExcelJS.Workbook();
        await workbook.xlsx.load(buffer);

        const sheet = workbook.getWorksheet('LOT');
        if (!sheet) throw new Error('formato inválido');

        const headers: string[] = [];
        sheet.getRow(2).eachCell((cell, colNumber) => {
            headers[colNumber - 1] = (cell.value as string).trim();
        });

        const existingBatchNumbers = new Set(this.dataTableBatches.map(b => b.batch_number));
        const toFloat2 = (v: any) => Math.round(parseFloat(v) * 100) / 100;

        for (let i = 3; i <= sheet.rowCount; i++) {
            const row = sheet.getRow(i);
            if (!row.getCell(1).value) continue;
            if (row.getCell(1).value === "RESUMO SEMENTES FISCALIZADAS") break;

            const rowData: Record<string, any> = {};
            row.eachCell((cell, colNumber) => {
            const header = headers[colNumber - 1];
            rowData[header] = cell.value;
            });

            const batch_number = rowData['LOTE'];
            if (existingBatchNumbers.has(batch_number)) continue;

            const batch_year = parseInt(rowData['ANO']);
            const batch_month = (new Date(rowData['VCTO']).getMonth() + 1) % 12;

            const sack_weight = toFloat2(rowData['P.SC.']);
            const sack_amount = parseInt(rowData['QT.SC.']);
            const total_weight = toFloat2(rowData['P.TOT.']?.result);
            const pureness_score = toFloat2(rowData['PP']);
            const total_pureness_score = toFloat2(rowData['TOTAL PP']?.result);
            const outflow_total_pureness_score = toFloat2(rowData['SAÍDAS PP']);
            const outflow_total_weight = toFloat2(rowData['SAÍDAS KG']);
            const usage = rowData['USO'];

            // --- DB Format ---
            const batchForDB: BatchDB = {
                batch_number,
                batch_year,
                batch_month,
                seed: rowData['VARIEDADE'],
                coating: rowData['TIPO'],
                brand: rowData['SC'],
                sack_weight,
                sack_amount,
                total_weight,
                pureness_score,
                total_pureness_score,
                batch_status: 1,
                deleted_at: null,
                origin: null,
            };
            batchesAux.push(batchForDB);
            invoke('create_batch', { batch: batchForDB }).catch(console.error);

            const batchOutflowDB: BatchOutflowDB = {
                batch_number,
                batch_year,
                sack_amount,
                total_weight,
                total_pureness_score,
                usage
            };
            batchOutflowsAux.push(batchOutflowDB);

            // --- DataTable Format ---
            // const batchForTable: DataTableBatch = {
            //     key: createDataTableBatchKey(batch_number, batch_year),
            //     batch_number,
            //     batch_year,
            //     expire_date: parseExpireDate(batch_month, batch_year),
            //     seed: batchForDB.seed,
            //     coating: batchForDB.coating,
            //     brand: batchForDB.brand,
            //     sack_weight,
            //     sack_amount,
            //     total_weight: total_weight.toLocaleString("pt-BR"),
            //     pureness_score: pureness_score.toLocaleString("pt-BR"),
            //     total_pureness_score: total_pureness_score.toLocaleString("pt-BR"),
            //     batch_status: 1,
            //     deleted_at: null,
            //     _searchIndex: normalizeText([
            //         batch_number,
            //         batch_year,
            //         parseExpireDate(batch_month, batch_year),
            //         batchForDB.seed,
            //         batchForDB.coating,
            //         batchForDB.brand,
            //         sack_weight,
            //         sack_amount,
            //         total_weight.toLocaleString("pt-BR"),
            //         pureness_score.toLocaleString("pt-BR"),
            //         total_pureness_score.toLocaleString("pt-BR"),
            //     ].join(' '))
            // };
            const batchForTable: DataTableBatch = formatBatchForTable(batchForDB)
            dataTableBatchesAux.push(batchForTable);

            // --- Outflow ---
            const batchOutflow: DataTableBatchOutflow = {
                batch_number,
                batch_year,
                sack_amount,
                total_weight: outflow_total_weight.toLocaleString("pt-BR"),
                total_pureness_score: outflow_total_pureness_score.toLocaleString("pt-BR"),
                pureness_score: toFloat2(outflow_total_pureness_score / outflow_total_weight).toLocaleString("pt-BR"),
                usage,
                _searchIndex: normalizeText([
                    batch_number,
                    batch_year,
                    sack_amount,
                    outflow_total_weight.toLocaleString("pt-BR"),
                    outflow_total_pureness_score.toLocaleString("pt-BR"),
                    toFloat2(outflow_total_pureness_score / outflow_total_weight).toLocaleString("pt-BR"),
                    usage
                ].join(' '))
            };
            dataTableBatchOutflowsAux.push(batchOutflow);

            existingBatchNumbers.add(batch_number);
        }

        this.batches = batchesAux;
        this.batchOutflows = batchOutflowsAux;
        this.dataTableBatches = dataTableBatchesAux;
        this.dataTableBatchOutflows = dataTableBatchOutflowsAux;
    },
    async fetchBatches() {
        try {
            this.batches = await invoke('list_batches');

            this.dataTableBatches = this.batches.map((batch: BatchDB) => 
                formatBatchForTable(batch)
            );
        } catch (err) {
            console.error(err);
        }
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

function createDataTableBatchKey(batchNumber: number, batchYear: number): string {
    return `${batchNumber}${batchYear}`
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

function formatBatchForTable(batch: BatchDB): DataTableBatch {
    const batchForTable: DataTableBatch = {
        key: createDataTableBatchKey(batch.batch_number, batch.batch_year),
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
