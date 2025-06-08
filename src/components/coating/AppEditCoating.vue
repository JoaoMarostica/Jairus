<template>
  <n-modal
    v-model:show="editCoatingModal"
    style="width: 500px;"
    :mask-closable="false"
    preset="card"
    :closable="true"
    title="Editar Revestimento"
  >
    <n-grid :cols="2" x-gap="24px">
      <n-gi :span="1">
        <n-form
          ref="formRef"
          :model="form"
          :rules="rules"
          :size="size"
          label-placement="top"
        >
          <n-form-item
            label="Nome do Revestimento"
            path="coating_name"
            :validation-status="coatingNameInputStatus"
            :feedback="coatingNameInputFeedback"
          >
            <n-input 
              v-model:value="form.coating_name" 
              placeholder="Digite o nome do revestimento" 
              clearable 
            />
          </n-form-item>
        </n-form>
      </n-gi>

      <n-gi :span="1">
        <n-descriptions
          label-placement="top"
          :column="1"
          title="Resumo do Revestimento"
          size="small"
          v-if="form.coating_name"
        >
          <n-descriptions-item label="Nome" v-if="form.coating_name">
            {{ form.coating_name }}
          </n-descriptions-item>
        </n-descriptions>
      </n-gi>
    </n-grid>
    
    <template #footer>
      <div style="display: flex; justify-content: space-between; margin-top: 16px">
        <n-button @click="cancelEdit">
          Cancelar
        </n-button>
        <n-button 
          type="primary" 
          @click="updateCoating" 
          :disabled="coatingNameInputStatus === 'error'"
        >
          Salvar Alterações
        </n-button>
      </div>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { computed, ref, reactive, watchEffect, watch } from 'vue'
import { 
  NModal, 
  NInput, 
  FormInst, 
  NButton, 
  NForm, 
  NFormItem, 
  NDescriptions, 
  NDescriptionsItem, 
  NGi, 
  NGrid 
} from 'naive-ui'
import type { CoatingDB, DataTableCoating } from '@/types/coatings'
import { useCoatingsStore } from '@/stores/coatingsStore'
import { useGlobalStore } from '@/stores/globalStore'
import type { FormValidationStatus } from 'naive-ui/es/form/src/interface'

interface Props {
  coating?: DataTableCoating | null
}

const props = withDefaults(defineProps<Props>(), {
  coating: null
})

const editCoatingModal = defineModel('show', {
  type: Boolean,
  default: false
})

const globalStore = useGlobalStore()
const coatingsStore = useCoatingsStore()

const coatingNameInputStatus = ref<FormValidationStatus | undefined>(undefined)
const coatingNameInputFeedback = computed(() => {
  return coatingNameInputStatus.value === 'error'
    ? 'Já existe um revestimento com este nome'
    : undefined
})

const formRef = ref<FormInst | null>(null)
const size = ref<'small' | 'medium' | 'large'>('medium')
const form = reactive({
  coating_name: '' as string
})

// Guarda o nome original para identificar no backend
const originalName = ref<string>('')

// Validação para verificar se o revestimento já existe (exceto ele mesmo)
watchEffect(() => {
  coatingNameInputStatus.value = undefined

  if (form.coating_name && form.coating_name !== originalName.value) {
    const existingCoating = coatingsStore.coatings.find(
      coating => coating.coating_name.toLowerCase() === form.coating_name?.toLowerCase()
    )
    
    if (existingCoating) {
      coatingNameInputStatus.value = 'error'
    }
  }
})

watch([editCoatingModal, () => props.coating], () => {
  if (editCoatingModal.value && props.coating) {
    form.coating_name = props.coating.coating_name
    originalName.value = props.coating.coating_name
  }
})

watch(editCoatingModal, () => {
  if (!editCoatingModal.value) {
    resetForm()
  }
})

const rules = {
  coating_name: {
    required: true,
    trigger: ['blur', 'input'],
    message: 'Campo obrigatório'
  }
}

function updateCoating(e: MouseEvent) {
  e.preventDefault()
  formRef.value?.validate(async (errors) => {
    if (!errors) {
      const updatedCoating: CoatingDB = {
        coating_name: form.coating_name.trim()
      }
      try {
          
        // Ajuste aqui: passar o original name e o objeto atualizado
        await coatingsStore.editCoating(updatedCoating,
          originalName.value
        );
        
        globalStore.showMessage({
          content: 'Revestimento editado com sucesso.',
          type: 'success',
        })
        editCoatingModal.value = false
        resetForm()
      } catch (error: any) {
        console.error(error);
        globalStore.showMessage({
          content: 'Erro ao editar revestimento.',
          type: 'error',
        })
      }
    }
  })
}

function cancelEdit() {
  editCoatingModal.value = false
  resetForm()
}

function resetForm() {
  form.coating_name = ''
  originalName.value = ''
  coatingNameInputStatus.value = undefined
}
</script>