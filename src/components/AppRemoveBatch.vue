<template>
  <n-modal
    v-model:show="removeBatchModal"
    style="width: 400px;"
    :mask-closable="false"
    preset="dialog"
    type="error"
    :title="modalTitle"
    content="Tem certeza que deseja remover este lote? Esta ação é irreversível e excluirá completamente todos os dados associados."
    positive-text="Confirmar"
    negative-text="Cancelar"
    @positive-click="confirmRemove"
    @negative-click="cancelRemove"
  />
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useBatchesStore } from '@/stores/batchesStore'
import { useGlobalStore } from '@/stores/globalStore'
import { NModal } from 'naive-ui'

const removeBatchModal = defineModel('modal', {
  type: Boolean,
  default: false,
})

const props = defineProps<{
  selectedBatch: any,
  multiple: boolean,
}>()

const batchesStore = useBatchesStore()

const globalStore = useGlobalStore()

const selectedBatch = computed(() => props.selectedBatch)

const modalTitle = computed(() =>
  props.selectedBatch?.batch_number
    ? `Remoção do Lote ${props.selectedBatch.batch_number}/${String(props.selectedBatch.batch_year).slice(-2)}`
    : 'Remoção de Lote'
)

function confirmRemove() {
  if (props.multiple) {
    removeSelectedBatches()
  } else {
    removeBatch()
  }
}

async function removeBatch() {
  try {
    await batchesStore.removeBatch(selectedBatch.value)

    globalStore.showMessage({
      content: 'Lote removido com sucesso!',
      type: 'success',
    })

    removeBatchModal.value = false
  } catch (error: any) {
    globalStore.showMessage({
      content: `Erro ao remover lote: ${error?.message || error}`,
      type: 'error',
      keepAliveOnHover: true,
    })
  }
}

async function removeSelectedBatches() {
  try {
    await batchesStore.removeSelectedBatches()

    globalStore.showMessage({
      content: 'Lotes selecionados removidos com sucesso!',
      type: 'success',
    })

    removeBatchModal.value = false
  } catch (error: any) {
    globalStore.showMessage({
      content: `Erro ao remover lotes selecionados: ${error?.message || error}`,
      type: 'error',
      keepAliveOnHover: true,
    })
  }
}

function cancelRemove() {
  removeBatchModal.value = false
}

</script>
