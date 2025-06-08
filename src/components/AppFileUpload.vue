<template>
  <n-modal
    v-model:show="fileUploadModal"
    preset="card"
    :mask-closable="false"
    draggable
    title="Importar Planilha Excel"
    style="width: 500px; position: fixed;  top: 70px; right: 122px"
  >
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
            Clique ou arraste a planilha para esta área para importar
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
import { useSheetStore } from '@/stores/sheetStore';
import { useBatchesStore } from '@/stores/batchesStore';
import { useOutflowsStore } from '@/stores/outflowsStore';
import { UploadFileInfo } from 'naive-ui'
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

const batchesStore = useBatchesStore();
const outflowsStore = useOutflowsStore();
const sheetStore = useSheetStore();

const loading = ref(false)

async function fetchData() {
  await batchesStore.fetchBatchesData();
  await outflowsStore.fetchOutflowsData();
}

async function handleFileChange({ file }: { file: UploadFileInfo }) {
  if (loading.value) return
  loading.value = true

  const rawFile = file.file
  
  if (!rawFile) {
    loading.value = false
    return
  }

  sheetStore.importBatchesFromSheet(rawFile)
    .then(async () => {
      globalStore.showMessage({
        content: 'Planilha lida com sucesso.',
        type: 'success',
      })
      await fetchData();
      fileUploadModal.value = false
    })
    .catch((error) => {
      console.error(error);
      globalStore.showMessage({
        content: 'Erro ao ler a planilha.',
        type: 'error',
      })
      fileUploadModal.value = false
    })
    .finally(() => {
      loading.value = false
    })
}

</script>