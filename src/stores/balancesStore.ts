import { defineStore } from 'pinia';
import type { DataTableBatch } from '@/types/batches';
import type { BalanceDB, DataTableBalanceOutflow } from '@/types/balance';
import { parsePtBrNumber } from '@/utils/parsing';
import { invoke } from '@tauri-apps/api/core';

export const useBalancesStore = defineStore('balances', {
  state: () => ({
  }),
  actions: {
    async getBatchBalance(batch: DataTableBatch): Promise<DataTableBalanceOutflow> {
        try {
            const outflowTotals: BalanceDB = await invoke('get_total_outflow', {
                batchNumber: batch.batch_number,
                batchYear: batch.batch_year
            })

            // Calculate totalPP and totalWeight from batch properties
            const totalPP = parsePtBrNumber(batch.total_pureness_score);
            const totalWeight = parsePtBrNumber(batch.total_weight);
            const sackAmount = batch.sack_amount;

            const balancePP = totalPP - outflowTotals.total_pureness_score;
            const balanceWeight = totalWeight - outflowTotals.total_weight;
            const balanceSackAmount = sackAmount - outflowTotals.sack_amount;

            const balance: DataTableBalanceOutflow = {
                key: createDataTableKey(batch.batch_number, batch.batch_number),
                sack_amount: Math.max(balanceSackAmount, 0),
                total_weight: parseFloat(Math.max(balanceWeight, 0).toFixed(2)).toLocaleString("pt-BR"),
                total_pureness_score: parseFloat(Math.max(balancePP, 0).toFixed(2)).toLocaleString("pt-BR"),
                _searchIndex: normalizeText([
                    batch.batch_number,
                    batch.batch_number,
                    (balanceSackAmount).toString(),
                    parseFloat(Math.max(balanceWeight, 0).toFixed(2)).toLocaleString("pt-BR"),
                    parseFloat(Math.max(balancePP, 0).toFixed(2)).toLocaleString("pt-BR")
                ].join(' '))
            }

            return balance
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
