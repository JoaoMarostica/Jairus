import { defineStore } from 'pinia'
import type { RawBatch, BatchDB } from '@/types/batches'
import { invoke } from '@tauri-apps/api/core'
import * as XLSX from 'xlsx'

export const useSheetStore = defineStore('sheet', {
  state: () => ({}),
  actions: {
    async importBatchesFromSheet(file: File) {
      const data = await file.arrayBuffer()

      const workbook = XLSX.read(data, { type: 'array' })
      const sheet = workbook.Sheets['LOT']
      if (!sheet) throw new Error('formato inválido')

      // Como o cabeçalho está na linha 2, pulamos a linha 1
      const rawJson = XLSX.utils.sheet_to_json(sheet, { range: 1, defval: null })

      const toFloat2 = (value: any) => Math.round(parseFloat(value) * 100) / 100

      const dataFormatted: RawBatch[] = []

      for (const row of rawJson) {
        const rowData = row as Record<string, any>
        if (!rowData['LOTE']) continue
        if (rowData['LOTE'] === 'RESUMO SEMENTES FISCALIZADAS') break

        const batch: RawBatch = {
          batch_number: rowData['LOTE'],
          batch_year: parseInt(rowData['ANO']),
          batch_month: (new Date(rowData['VCTO']).getMonth() + 1) % 12,
          seed: rowData['VARIEDADE'],
          coating: rowData['TIPO'],
          brand: rowData['SC'],
          sack_weight: toFloat2(rowData['P.SC.']),
          sack_amount: parseInt(rowData['QT.SC.']),
          total_weight: toFloat2(rowData['P.TOT.']),
          pureness_score: toFloat2(rowData['PP']),
          total_pureness_score: toFloat2(rowData['TOTAL PP']),
          outflow_total_pureness_score: toFloat2(rowData['SAÍDAS PP']),
          outflow_total_weight: toFloat2(rowData['SAÍDAS KG']),
          usage: rowData['USO'],
          batch_status: 1,
          deleted_at: null,
          origin: null
        }

        dataFormatted.push(batch)
      }

      this.setBatchesFromSheetData(dataFormatted)
    },

    setBatchesFromSheetData(data: RawBatch[]) {
      this.$reset()

      data.forEach(async (batch) => {
        const batchForDB: BatchDB = formatBatchForDB(batch)

        await invoke('add_batch', {
          batch: batchForDB,
        }).catch(console.error)

        const batchOutflowForDB = {
          batch_number: batch.batch_number,
          batch_year: batch.batch_year,
          sack_amount: batch.sack_amount,
          total_weight: batch.total_weight,
          total_pureness_score: batch.total_pureness_score,
          usage: batch.usage,
        }

        await invoke('add_outflow', {
          new: batchOutflowForDB,
        }).catch(console.error)
      })
    },
  },
})

function formatBatchForDB(batch: RawBatch): BatchDB {
  return {
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
  }
}
