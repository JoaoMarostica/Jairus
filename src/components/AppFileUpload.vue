<template>
  <n-modal v-model:show="fileUploadModal" preset="card" title="Upload de Planilha" style="width: 500px">
    <n-spin :show="loading">
      <n-upload
        :default-upload="false"
        :on-change="handleFileChange"
        :max="1"
        directory-dnd
      >
        <n-upload-dragger>
          <div style="margin-bottom: 12px">
            <n-icon size="48" :depth="3">
            <ArchiveOutlined />
            </n-icon>
          </div>
          <n-text style="font-size: 16px">
            Clique ou arraste arquivos para esta área para fazer upload
          </n-text>
          <n-p depth="3" style="margin: 8px 0 0 0">
            Não envie informações sensíveis como senhas ou dados bancários.
          </n-p>
        </n-upload-dragger>
      </n-upload>
      <template #description>
        Lendo planilha...
      </template>
    </n-spin>
  </n-modal>
</template>
  
<script setup lang="ts">
import { ArchiveOutlined } from '@vicons/material'
import { useGlobalStore } from '@/stores/globalStore';
import { storeToRefs } from 'pinia';
import { ref } from 'vue';
import { useBatchesStore } from '@/stores/batchesStore';
// import { invoke } from '@tauri-apps/api/core'
import { UploadFileInfo } from 'naive-ui'
import ExcelJS from 'exceljs'
import {
  NModal,
  NUpload,
  NUploadDragger,
  NText,
  NP,
  NIcon,
  NSpin,
} from 'naive-ui'

const globalStore = useGlobalStore();
const { fileUploadModal } = storeToRefs(globalStore);
const batchStore = useBatchesStore();

const loading = ref(false)

function handleFileChange({ file }: { file: UploadFileInfo }) {
  loading.value = true
  const raw = file.file
  if (!raw) return

  readExcelFile(raw)
    .then((data) => {
      // função para enviar para o backend...
      
      batchStore.setBatches(data)
      globalStore.showMessage({
        content: 'Planilha lida com sucesso!',
        type: 'success',
      })
      fileUploadModal.value = false
      loading.value = false
    })
    .catch((err) => {
      globalStore.showMessage({
        content: `Erro ao ler a planilha: ${err.message}`,
        type: 'error',
      })
      fileUploadModal.value = false
      loading.value = false
    })
}

async function readExcelFile(file: File) {
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
      number: rowData['LOTE'],
      year: parseInt(rowData['ANO']),
      // ex1.: set/2024 --> 7 + 1 = 8 % 12 = 8 (setembro em um array de [0..11])
      // ex2.: jan/2026 --> 11 + 1 = 12 % 12 = 0 (janeiro em um array de [0..11])
      expireDate: (new Date(rowData['VCTO']).getMonth() + 1) % 12,
      seed: rowData['VARIEDADE'],
      coating: rowData['TIPO'],
      sackBrand: rowData['SC'],
      sackQuantity: parseInt(rowData['QT.SC.']),
      sackWeight: toFloat2(rowData['P.SC.']),
      purenessScore: toFloat2(rowData['PP']),
      outflowPP: toFloat2(rowData['SAÍDAS PP']),
      outflowKg: toFloat2(rowData['SAÍDAS KG']),
      usage: rowData['USO'],
      status: "active",
      deletedAt: null,
    }
    data.push(batch)
  }
  return data
}

</script>