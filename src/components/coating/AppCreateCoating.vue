<template>
  <n-modal
    v-model:show="createCoatingModal"
    style="width: 600px;"
    :mask-closable="false"
    preset="card"
    :closable="true"
    title="Novo Tratamento"
  >
    <n-grid :cols="2" x-gap="24px">
      <!-- Coluna do formulário -->
      <n-gi :span="1">
        <n-form
          ref="formRef"
          :model="form"
          :rules="rules"
          :size="size"
          label-placement="top"
        >
          <n-form-item
            label="Nome do Tratamento"
            path="coating_name"
            :validation-status="coatingNameInputStatus"
            :feedback="coatingNameInputFeedback"
          >
            <n-input 
              v-model:value="form.coating_name" 
              placeholder="Digite o nome do tratamento" 
              clearable 
            />
          </n-form-item>
        </n-form>
      </n-gi>

      <!-- Coluna das descrições -->
      <n-gi :span="1">
        <n-descriptions
          label-placement="top"
          :column="1"
          title="Resumo do Tratamento"
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
        <n-button @click="resetForm">
          Cancelar
        </n-button>
        <n-button 
          type="primary" 
          @click="createCoating" 
          :disabled="coatingNameInputStatus === 'error'"
        >
          Criar Tratamento
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
import type { CoatingDB } from '@/types/coatings'
import { useCoatingsStore } from '@/stores/coatingsStore'
import { useGlobalStore } from '@/stores/globalStore'
import type { FormValidationStatus } from 'naive-ui/es/form/src/interface'

// Model
const createCoatingModal = defineModel('show', {
  type: Boolean,
  default: false
})

const globalStore = useGlobalStore()
const coatingsStore = useCoatingsStore()

const coatingNameInputStatus = ref<FormValidationStatus | undefined>(undefined)
const coatingNameInputFeedback = computed(() => {
  return coatingNameInputStatus.value === 'error'
    ? 'Já existe um tratamento com este nome'
    : undefined
})

const formRef = ref<FormInst | null>(null)
const size = ref<'small' | 'medium' | 'large'>('medium')
const form = reactive({
  coating_name: '' as string,
})

// Validação para verificar se o Tratamento já existe
watchEffect(() => {
  coatingNameInputStatus.value = undefined

  if (form.coating_name) {
    const existingCoating = coatingsStore.coatings.find(
      coating => coating.coating_name.toLowerCase() === form.coating_name?.toLowerCase()
    )
    
    if (existingCoating) {
      coatingNameInputStatus.value = 'error'
    }
  }
})

// Limpa o formulário quando o modal fecha
watch(createCoatingModal, () => {
  if (!createCoatingModal.value) {
    resetForm()
  }
})

const rules = {
  coating_name: {
    required: true,
    trigger: ['blur', 'input'],
    message: 'Campo obrigatório',
  }
}



function createCoating(e: MouseEvent) {
  e.preventDefault()
  formRef.value?.validate(async (errors) => {
    if (!errors) {
      const newCoating: CoatingDB = {
        coating_name: form.coating_name.trim()
      }

      try {
       
        await coatingsStore.createCoating(newCoating)
        
       
        globalStore.showMessage({
          content: 'Tratamento criado com sucesso!',
          type: 'success',
        })
        
        
        resetForm()
        createCoatingModal.value = false
        
      } catch (error: any) {
        globalStore.showMessage({
          content: `Erro ao criar tratamento: ${error?.message || error}`,
          type: 'error',
        })
      }
    } else {
      globalStore.showMessage({
        content: 'Preencha todos os campos obrigatórios.',
        type: 'error',
      })
    }
  })
}

function resetForm() {
  form.coating_name = ''
  coatingNameInputStatus.value = undefined
  
}
</script>