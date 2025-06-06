import { defineStore } from 'pinia';
import type { RawBatch, BatchDB } from '@/types/batches';
import ExcelJS from 'exceljs';
import { invoke } from '@tauri-apps/api/core';

export const useSheetStore = defineStore('sheet', {
  state: () => ({
  }),
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
                batch_status: 1,
                deleted_at: null,
                origin: null
            }
            data.push(batch)
        }
        this.setBatchesFromSheetData(data)
    },
    setBatchesFromSheetData(data: RawBatch[]) {
        this.$reset();

        data.forEach(async (batch) => {
            const batchForDB: BatchDB = formatBatchForDB(batch)

            await invoke('add_batch', {
                batch: batchForDB
            }).then((res) => {
                console.log(res);
            }).catch(console.error);

            const batchOutflowForDB = {
                batch_number: batch.batch_number,
                batch_year: batch.batch_year,
                sack_amount: batch.sack_amount,
                total_weight: batch.total_weight,
                total_pureness_score: batch.total_pureness_score,
                usage: batch.usage,
            };

            await invoke('add_outflow', {
                new: batchOutflowForDB
            }).then((res) => {
                console.log(res);
            }).catch(console.error);
        });
    },
  },
});

function formatBatchForDB(batch: RawBatch): BatchDB {
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
