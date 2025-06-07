<template>
  <n-modal
    v-model:show="removeOutflowModal"
    style="width: 400px;"
    :mask-closable="false"
    :closable="true"
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
import { useOutflowsStore } from '@/stores/outflowsStore';
import { useGlobalStore } from '@/stores/globalStore'
import { NModal } from 'naive-ui'

const removeOutflowModal = defineModel('modal', {
  type: Boolean,
  default: false,
})

const emit = defineEmits<{
  (e: 'reloadData'): void
}>()

const props = defineProps<{
  selectedOutflow: any
}>()

const outflowsStore = useOutflowsStore();
const globalStore = useGlobalStore()

const selectedOutflow = computed(() => {
  console.log(`Selected Outflow:`, props.selectedOutflow);
  return props.selectedOutflow
})

const modalTitle = computed(() =>
  props.selectedOutflow?.usage
    ? `Remoção do Pedido ${props.selectedOutflow.usage}`
    : 'Remoção de Pedido'
)

async function confirmRemove() {
  try {
    await outflowsStore.removeOutflow(selectedOutflow.value)

    globalStore.showMessage({
      content: 'Pedido removido com sucesso!',
      type: 'success',
    })

    emit('reloadData')
    removeOutflowModal.value = false
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
