<template>
  <n-modal
    v-model:show="removeOutflowModal"
    style="width: 400px;"
    :mask-closable="false"
    preset="dialog"
    type="error"
    :title="modalTitle"
    content="Tem certeza que deseja remover este pedido? Esta ação é irreversível e excluirá completamente todos os dados associados."
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

const removeOutflowModal = defineModel('modal', {
  type: Boolean,
  default: false,
})

const emit = defineEmits<{
  (e: 'close'): void
}>()

const props = defineProps<{
  selectedOutflow: any
}>()

const batchesStore = useBatchesStore()

const globalStore = useGlobalStore()

const selectedOutflow = computed(() => props.selectedOutflow)

const modalTitle = computed(() =>
  props.selectedOutflow?.usage
    ? `Remoção do Pedido ${props.selectedOutflow.usage}}`
    : 'Remoção de Pedido'
)

async function confirmRemove() {
  try {
    await batchesStore.removeOutflow(selectedOutflow.value)

    globalStore.showMessage({
      content: 'Pedido removido com sucesso!',
      type: 'success',
    })

    removeOutflowModal.value = false
    emit('close')
  } catch (error: any) {
    globalStore.showMessage({
      content: `Erro ao remover pedido: ${error?.message || error}`,
      type: 'error',
      keepAliveOnHover: true,
    })
  }
}

function cancelRemove() {
  removeOutflowModal.value = false
}

</script>
