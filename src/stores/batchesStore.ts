import { defineStore } from 'pinia';
import type { RawBatch, BatchDB, BatchOutflowDB, DataTableBatch, DataTableBatchOutflow } from '@/types/batches';
import type { DataTableRowKey } from 'naive-ui';
import ExcelJS from 'exceljs';
import { invoke } from '@tauri-apps/api/core';

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
    async importBatchesFromSheet(file: File) {
        const buffer = await file.arrayBuffer()
        const workbook = new ExcelJS.Workbook()
        await workbook.xlsx.load(buffer)

        const sheet = workbook.getWorksheet('LOT')
        if (!sheet) {
            throw new Error('formato inválido')
        }

        // Pega os títulos da linha 2 (cabeçalho)
        const headers: string[] = []
        sheet.getRow(2).eachCell((cell, colNumber) => {
            headers[colNumber - 1] = (cell.value as string).trim()
        })

        const data = []
        for (let i = 3; i <= sheet.rowCount; i++) {
            const row = sheet.getRow(i)

            if (row.getCell(1).value === null) continue
            if (row.getCell(1).value === "RESUMO SEMENTES FISCALIZADAS") break

            // Monta um objeto que mapeia header -> valor da célula
            const rowData: Record<string, any> = {}
            row.eachCell((cell, colNumber) => {
            const header = headers[colNumber - 1]
            rowData[header] = cell.value
            })

            const toFloat2 = (value: any) => Math.round(parseFloat(value) * 100) / 100
            
            // Pegando os dados do lote que realmente interessam
            const batch = {
                batch_number: rowData['LOTE'],
                batch_year: parseInt(rowData['ANO']),
                // ex1.: set/2024 --> 7 + 1 = 8 % 12 = 8 (setembro em um array de [0..11])
                // ex2.: jan/2026 --> 11 + 1 = 12 % 12 = 0 (janeiro em um array de [0..11])
                batch_month: (new Date(rowData['VCTO']).getMonth() + 1) % 12,
                seed: rowData['VARIEDADE'],
                coating: rowData['TIPO'],
                brand: rowData['SC'],
                sack_weight: toFloat2(rowData['P.SC.']),
                sack_amount: parseInt(rowData['QT.SC.']),
                total_weight: toFloat2(rowData['P.TOT.'].result),
                pureness_score: toFloat2(rowData['PP']),
                total_pureness_score: toFloat2(rowData['TOTAL PP'].result),
                outflow_total_pureness_score: toFloat2(rowData['SAÍDAS PP']),
                outflow_total_weight: toFloat2(rowData['SAÍDAS KG']),
                usage: rowData['USO'],
                batch_status: 1, // 1 = ativo
                deleted_at: null,
                origin: null
            }
            data.push(batch)
        }
        this.setBatchesFromSheetData(data)
    },
    setBatchesFromSheetData(data: RawBatch[]) {
        this.$reset;

        const toFloat2 = (value: any) => Math.round(parseFloat(value) * 100) / 100;

        const existingBatchNumbers = new Set(this.dataTableBatches.map(b => b.batch_number));

        data.forEach((batch) => {
            if (!existingBatchNumbers.has(batch.batch_number)) {
                // Lotes

                // Para enviar para o DB
                const batchForDB: BatchDB = formatBatchDB(batch)
                this.batches.push(batchForDB);

                const batchForDataTable: DataTableBatch = formatBatchForTable(batchForDB)
                this.dataTableBatches.push(batchForDataTable);
                existingBatchNumbers.add(batch.batch_number);

                // Enviando para o DB
                invoke('create_batch', {
                    batch: batchForDB
                }).then((res) => {
                    console.log(res);
                }).catch(console.error);
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
    async fetchBatches() {
        try {
            this.$reset;

            this.batches = await invoke('list_batches');

            this.batches.forEach((batch: BatchDB) =>
                this.dataTableBatches.push(formatBatchForTable(batch))
            );
        } catch (err) {
            console.error(err);
        }
    },
    async createBatch(newBatch: BatchDB) {
        try {
            const createdBatch: BatchDB = await invoke('create_batch', {
                batch: newBatch
            });

            this.dataTableBatches.push(formatBatchForTable(createdBatch));
        } catch (err) {
            console.error('Erro ao criar batch:', err);
        }
    },
    getLastBatch() {
        const batchesNumber = this.dataTableBatches
            .filter(batch => batch.batch_status === 1)
            .map(batch => batch.batch_number);

        const lastBatchNumber = Math.max(...batchesNumber);

        return lastBatchNumber;
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
    },
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

function formatBatchDB(batch: RawBatch): BatchDB {
    const BatchForDB: BatchDB = {
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

    return BatchForDB
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
