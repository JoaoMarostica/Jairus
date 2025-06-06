<template>
  <n-modal
    v-model:show="createSeedModal"
    style="width: 600px;"
    :mask-closable="false"
    preset="card"
    :closable="true"
    title="Nova Cultivar"
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
            label="Nome Popular"
            path="popular_name"
            :validation-status="seedNameInputStatus"
            :feedback="seedNameInputFeedback"
          >
            <n-input 
              v-model:value="form.popular_name" 
              placeholder="Digite o nome popular da cultivar" 
              clearable 
            />
          </n-form-item>
          
          <n-form-item
            label="Nome Científico"
            path="scientific_name"
          >
            <n-input 
              v-model:value="form.scientific_name" 
              placeholder="Digite o nome científico da cultivar" 
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
          title="Resumo da Cultivar"
          size="small"
          v-if="form.popular_name || form.scientific_name"
        >
          <n-descriptions-item label="Nome Popular" v-if="form.popular_name">
            {{ form.popular_name }}
          </n-descriptions-item>
          <n-descriptions-item label="Nome Científico" v-if="form.scientific_name">
            {{ form.scientific_name }}
          </n-descriptions-item>
        </n-descriptions>
      </n-gi>
    </n-grid>
    
    <template #footer>
      <div style="display: flex; justify-content: space-between; margin-top: 16px">
        <n-button @click="cancel">
          Cancelar
        </n-button>
        <n-button 
          type="primary" 
          @click="handleSubmit" 
          :disabled="seedNameInputStatus === 'error'"
        >
          Criar Cultivar
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
import type { SeedDB } from '@/types/seeds'
import { useSeedsStore } from '@/stores/seedsStore'
import { useGlobalStore } from '@/stores/globalStore'
import type { FormValidationStatus } from 'naive-ui/es/form/src/interface'

const createSeedModal = defineModel('modal', {
  type: Boolean,
  default: false
})

const globalStore = useGlobalStore()
const seedsStore = useSeedsStore()

const seedNameInputStatus = ref<FormValidationStatus | undefined>(undefined)
const seedNameInputFeedback = computed(() => {
  return seedNameInputStatus.value === 'error'
    ? 'Cultivar já existe'
    : undefined
})

const formRef = ref<FormInst | null>(null)
const size = ref<'small' | 'medium' | 'large'>('medium')
const form = reactive({
  popular_name: null as string | null,
  scientific_name: null as string | null,
})

// Validação para verificar se a cultivar já existe
watchEffect(() => {
  seedNameInputStatus.value = undefined

  if (form.popular_name) {
    const existingSeed = seedsStore.seeds.find(
      seed => seed.popular_name.toLowerCase() === form.popular_name?.toLowerCase()
    )
    
    if (existingSeed) {
      seedNameInputStatus.value = 'error'
    }
  }
})


watch(createSeedModal, () => {
  if (!createSeedModal.value) {
    resetForm()
  }
})

const rules = {
  popular_name: {
    required: true,
    trigger: ['blur', 'input'],
    message: 'Campo obrigatório',
  },
  scientific_name: {
    required: true,
    trigger: ['blur', 'input'],
    message: 'Campo obrigatório'
  }
}

function handleSubmit(e: MouseEvent) {
  e.preventDefault()
  formRef.value?.validate(async (errors) => {
    if (!errors) {
      const seed: SeedDB = {
        popular_name: form.popular_name || '',
        scientific_name: form.scientific_name || ''
      }

      try {
        await seedsStore.createSeed(seed)
        
        globalStore.showMessage({
          content: 'Cultivar criada com sucesso!',
          type: 'success',
        })
        createSeedModal.value = false
        resetForm()
      } catch (error: any) {
        globalStore.showMessage({
          content: `Erro ao criar cultivar: ${error?.message || error}`,
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

function cancel() {
  createSeedModal.value = false
  resetForm()
}

function resetForm() {
  form.popular_name = null
  form.scientific_name = null
  seedNameInputStatus.value = undefined
}
</script>