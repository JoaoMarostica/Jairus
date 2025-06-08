<template>
  <n-modal
    v-model:show="editSeedModal"
    style="width: 600px;"
    :mask-closable="false"
    preset="card"
    :closable="true"
    title="Editar Cultivar"
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
        <n-button @click="cancelEdit">
          Cancelar
        </n-button>
        <n-button 
          type="primary" 
          @click="updateSeed" 
          :disabled="seedNameInputStatus === 'error'"
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
import type { SeedDB, DataTableSeed } from '@/types/seeds'
import { useSeedsStore } from '@/stores/seedsStore'
import { useGlobalStore } from '@/stores/globalStore'
import type { FormValidationStatus } from 'naive-ui/es/form/src/interface'

interface Props {
  seed?: DataTableSeed | null
}

const props = withDefaults(defineProps<Props>(), {
  seed: null
})

const editSeedModal = defineModel('show', {
  type: Boolean,
  default: false
})

const globalStore = useGlobalStore()
const seedsStore = useSeedsStore()

const seedNameInputStatus = ref<FormValidationStatus | undefined>(undefined)
const seedNameInputFeedback = computed(() => {
  return seedNameInputStatus.value === 'error'
    ? 'Já existe uma cultivar com este nome'
    : undefined
})

const formRef = ref<FormInst | null>(null)
const size = ref<'small' | 'medium' | 'large'>('medium')
const form = reactive({
  popular_name: '' as string,
  scientific_name: '' as string,
})

// Guarda o nome original para identificar no backend
const originalName = ref<string>('')

// Validação para verificar se a cultivar já existe (exceto ela mesma)
watchEffect(() => {
  seedNameInputStatus.value = undefined

  if (form.popular_name && form.popular_name !== originalName.value) {
    const existingSeed = seedsStore.seeds.find(
      seed => seed.popular_name.toLowerCase() === form.popular_name?.toLowerCase()
    )
    
    if (existingSeed) {
      seedNameInputStatus.value = 'error'
    }
  }
})

watch([editSeedModal, () => props.seed], () => {
  if (editSeedModal.value && props.seed) {
    form.popular_name = props.seed.popular_name
    form.scientific_name = props.seed.scientific_name
    originalName.value = props.seed.popular_name
  }
})

watch(editSeedModal, () => {
  if (!editSeedModal.value) {
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

function updateSeed(e: MouseEvent) {
  e.preventDefault()
  formRef.value?.validate(async (errors) => {
    if (!errors) {

      const updatedSeed: SeedDB = {
        popular_name: form.popular_name,
        scientific_name: form.scientific_name
      }

      try {
        await seedsStore.editSeed( updatedSeed,originalName.value)
        
        globalStore.showMessage({
          content: 'Cultivar editada com sucesso.',
          type: 'success',
        })
        editSeedModal.value = false
        resetForm()
      } catch (error: any) {
        console.error(error);
        globalStore.showMessage({
          content: 'Erro ao editar cultivar.',
          type: 'error',
        })
      }
    }
  })
}

function cancelEdit() {
  editSeedModal.value = false
  resetForm()
}

function resetForm() {
  form.popular_name = ''
  form.scientific_name = ''
  originalName.value = ''
  seedNameInputStatus.value = undefined
}
</script>