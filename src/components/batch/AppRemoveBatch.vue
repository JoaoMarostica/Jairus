<template>
  <n-spin :show="loading">
    <n-modal
      v-model:show="removeBatchModal"
      style="width: 400px;"
      :mask-closable="false"
      preset="dialog"
      type="error"
      :title="modalTitle"
      :content=removeContent
      positive-text="Confirmar"
      negative-text="Cancelar"
      @positive-click="confirmRemove"
      @negative-click="cancelRemove"
    />
  </n-spin>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useBatchesStore } from '@/stores/batchesStore'
import { useGlobalStore } from '@/stores/globalStore'
import { NModal, NSpin } from 'naive-ui'

const removeBatchModal = defineModel('modal', {
  type: Boolean,
  default: false,
})

const props = defineProps<{
  selectedBatch: any,
  option: string,
}>()

const loading = ref(false);

const batchesStore = useBatchesStore()
const globalStore = useGlobalStore()

const selectedBatch = computed(() => props.selectedBatch)

const modalTitle = computed(() =>
  props.selectedBatch?.batch_number && props.option === 'one'
    ? `Remoção do Lote ${props.selectedBatch.batch_number}/${String(props.selectedBatch.batch_year).slice(-2)}`
    : 'Remoção de Lote'
)

const removeContent = computed(() => {
  if (props.option === 'multiple') {
    return "Tem certeza que deseja remover os lotes selecionados? Esta ação é irreversível e excluirá completamente todos os dados associados.";
  } else if (props.option === 'all') {
    return "Tem certeza que deseja remover todos os lotes? Esta ação é irreversível e excluirá completamente todos os dados associados.";
  } else {
    return "Tem certeza que deseja remover este lote? Esta ação é irreversível e excluirá completamente todos os dados associados.";
  }
});

function confirmRemove() {
  if (props.option === 'multiple') {
    removeSelectedBatches()
  } else if (props.option === 'all') {
    removeAllBatches()
  } else {
    removeBatch()
  }
}

async function removeBatch() {
  try {
    loading.value = true
    await batchesStore.removeBatch(selectedBatch.value)

    globalStore.showMessage({
      content: 'Lote removido com sucesso.',
      type: 'success',
    })
  } catch (error: any) {
    console.error(error);
    globalStore.showMessage({
      content: 'Erro ao remover lote.',
      type: 'error',
      keepAliveOnHover: true,
    })
  } finally {
    loading.value = false
    removeBatchModal.value = false
  }
}

async function removeSelectedBatches() {
  try {
    loading.value = true
    await batchesStore.removeSelectedBatches()

    globalStore.showMessage({
      content: 'Lotes selecionados removidos com sucesso.',
      type: 'success',
    })
  } catch (error: any) {
    console.error(error);
    globalStore.showMessage({
      content: 'Erro ao remover lotes selecionados.',
      type: 'error',
    })
  } finally {
    loading.value = false
    removeBatchModal.value = false
  }
}

async function removeAllBatches() {
  try {
    loading.value = true
    await batchesStore.removeAllBatches()

    globalStore.showMessage({
      content: 'Todos os lotes removidos com sucesso.',
      type: 'success',
    })
  } catch (error: any) {
    console.error(error);
    globalStore.showMessage({
      content: 'Erro ao remover todos os lotes.',
      type: 'error',
    })
  } finally {
    loading.value = false
    removeBatchModal.value = false
  }
}

function cancelRemove() {
  removeBatchModal.value = false
}

</script>
