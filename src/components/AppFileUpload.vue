<template>
    <n-modal v-model:show="fileUploadModel" preset="card" title="Upload de Planilha" style="width: 500px">
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
    </n-modal>
</template>
  
<script setup lang="ts">
import { ArchiveOutlined } from '@vicons/material'
import { useGlobalStore } from '@/stores/globalStore';
import { storeToRefs } from 'pinia';
import { invoke } from '@tauri-apps/api/core'
import { UploadFileInfo } from 'naive-ui'
import {
    NModal,
    NUpload,
    NUploadDragger,
    NText,
    NP,
    NIcon,
} from 'naive-ui'

defineEmits(['toggle-sidebar']);

const globalStore = useGlobalStore();
const { fileUploadModel } = storeToRefs(globalStore);

function handleFileChange({ file }: { file: UploadFileInfo }) {
    const raw = file.file
    if (!raw) return

    const reader = new FileReader()

    reader.onload = async () => {
        const buffer = reader.result as ArrayBuffer
        const bytes = Array.from(new Uint8Array(buffer))

        try {
            const res = await invoke("process_file", {
            fileName: raw.name,
            fileBytes: bytes
            })
            console.log('Arquivo enviado com sucesso:', res)
        } catch (err) {
            console.error('Erro ao enviar o arquivo:', err)
        }
        fileUploadModel.value = false
    }

    reader.readAsArrayBuffer(raw)
}

</script>