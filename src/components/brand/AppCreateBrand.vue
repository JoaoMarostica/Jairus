<template>
  <n-modal
    v-model:show="createBrandModal"
    style="width: 600px;"
    :mask-closable="false"
    preset="card"
    :closable="true"
    title="Nova Marca"
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
            label="Nome da Marca"
            path="brand_name"
            :validation-status="brandNameInputStatus"
            :feedback="brandNameInputFeedback"
          >
            <n-input 
              v-model:value="form.brand_name" 
              placeholder="Digite o nome da marca" 
              clearable 
            />
          </n-form-item>
          <n-form-item
            label="Lista de pesos"
            path="weights"
            :validation-status="weightsStatus"
            :feedback="weightsFeedback"
          >
            <div class="weights-container">
              <div class="weight-input-container">
                <n-form-item 
                  :validation-status="newWeightStatus" 
                  :feedback="newWeightFeedback"
                  size="small"
                  style="margin-bottom: 0; flex-grow: 1;"
                >
                  <n-input-number
                    v-model:value="newWeight"
                    placeholder="Novo peso (kg)"
                    :min="1"
                    
                    style="width: 130px;"
                  />
                </n-form-item>
                
                <n-button 
                  secondary 
                  size="small" 
                  @click="addWeight"
                  style="margin-left: 10px;"
                >
                  <template #icon>
                    <n-icon><plus-outlined /></n-icon>
                  </template>
                  Adicionar
                </n-button>
              </div>
              <div class="weights-scrollable-container">
                <div v-for="(_, index) in form.weights" :key="index" class="weight-item">
                  <n-input-number
                    v-model:value="form.weights[index]"
                    placeholder="Peso (Kg)"
                    :min="0.1"
                    style="width: 130px;"
                  />
                  <n-button quaternary circle type="error" @click="removeWeight(index)" class="remove-weight-btn">
                    <template #icon>
                      <n-icon><delete-outlined /></n-icon>
                    </template>
                  </n-button>
                </div>
              </div>
            </div>
          </n-form-item>
        </n-form>
      </n-gi>

      <n-gi :span="1">
        <n-descriptions
          label-placement="top"
          :column="1"
          title="Resumo da Marca"
          size="small"
          v-if="form.brand_name || form.weights.length > 0"
        >
          <n-descriptions-item label="Nome da Marca" v-if="form.brand_name">
            {{ form.brand_name }}
          </n-descriptions-item>
          
          <n-descriptions-item label="Pesos" v-if="form.weights.length > 0">
            <n-tag
              v-for="(weight, index) in form.weights"
              :key="index"
              type="info"
              size="small"
              style="margin-right: 8px; margin-bottom: 8px"
            >
              {{ weight }} kg
            </n-tag>
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
          :disabled="brandNameInputStatus === 'error' || !isFormValid"
        >
          Criar Marca
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
  NInputNumber,
  FormInst, 
  NButton, 
  NForm, 
  NFormItem, 
  NDescriptions, 
  NDescriptionsItem, 
  NGi, 
  NGrid,
  NTag,
  NIcon 
} from 'naive-ui'
import { PlusOutlined, DeleteOutlined } from '@vicons/material'
import type { BrandDB } from '@/types/brands'
import { useBrandsStore } from '@/stores/brandsStore'
import { useGlobalStore } from '@/stores/globalStore'
import type { FormValidationStatus } from 'naive-ui/es/form/src/interface'

const createBrandModal = defineModel('show', {
  type: Boolean,
  default: false
})

const globalStore = useGlobalStore()
const brandsStore = useBrandsStore()

const brandNameInputStatus = ref<FormValidationStatus | undefined>(undefined)
const brandNameInputFeedback = computed(() => {
  return brandNameInputStatus.value === 'error'
    ? 'Marca já existe'
    : undefined
})

const weightsStatus = ref<FormValidationStatus | undefined>(undefined)
const weightsFeedback = computed(() => {
  return weightsStatus.value === 'error'
    ? 'Adicione pelo menos um peso'
    : undefined
})

const formRef = ref<FormInst | null>(null)
const size = ref<'small' | 'medium' | 'large'>('medium')
const form = reactive({
  brand_name: '' as string,
  weights: [] as number[]
})

const newWeight = ref<number | null>(null)
const newWeightStatus = ref<FormValidationStatus | undefined>(undefined)
const newWeightFeedback = ref<string | undefined>(undefined)

const isFormValid = computed(() => {
  return form.brand_name.trim() !== '' && form.weights.length > 0
})

// Validação para verificar se a marca já existe
watchEffect(() => {
  brandNameInputStatus.value = undefined

  if (form.brand_name) {
    const existingBrand = brandsStore.brands.find(
      brand => brand.brand_name.toLowerCase() === form.brand_name.toLowerCase()
    )
    
    if (existingBrand) {
      brandNameInputStatus.value = 'error'
    }
  }
})

watchEffect(() => {
  weightsStatus.value = form.weights.length === 0 ? 'error' : undefined
})

watch(createBrandModal, () => {
  if (createBrandModal.value) {
    form.weights = [10, 15, 25, 30]
  } else {
    resetForm()
  }
})

const rules = {
  brand_name: {
    required: true,
    trigger: ['blur', 'input'],
    message: 'Nome da marca é obrigatório',
  },
  weights: {
    validator: () => {
      if (form.weights.length === 0) {
        return new Error('Adicione pelo menos um peso')
      }
      return true
    },
    trigger: ['change']
  }
}

function handleSubmit(e: MouseEvent) {
  e.preventDefault()
  formRef.value?.validate(async (errors) => {
    if (!errors) {
      const brand: BrandDB = {
        brand_name: form.brand_name.trim(),
        weights: form.weights
      }

      try {
        await brandsStore.createBrand(brand)
        
        globalStore.showMessage({
          content: 'Marca criada com sucesso.',
          type: 'success',
        })
        createBrandModal.value = false
        resetForm()
      } catch (error: any) {
        console.error(error);
        globalStore.showMessage({
          content: 'Erro ao criar marca.',
          type: 'error',
        })
      }
    }
  })
}

function addWeight() {
  if (!newWeight.value || newWeight.value <= 0) {
    newWeightStatus.value = 'error'
    newWeightFeedback.value = 'Digite um peso válido'
    return
  }
  
  form.weights.push(newWeight.value)
  newWeight.value = null
  newWeightStatus.value = undefined
  newWeightFeedback.value = undefined
}

function removeWeight(index: number) {
  form.weights.splice(index, 1)
}

function cancel() {
  createBrandModal.value = false
  resetForm()
}

function resetForm() {
  form.brand_name = ''
  form.weights = []
  newWeight.value = null
  newWeightStatus.value = undefined
  newWeightFeedback.value = undefined
  brandNameInputStatus.value = undefined
  weightsStatus.value = undefined
}


</script>

<style scoped>
.weights-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.weights-scrollable-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  max-height: 150px;
  overflow-y: auto;
  padding-right: 5px;
}

.weight-item {
  display: flex;
  align-items: center;
  gap: 10px;
}

.weight-input-container {
  display: flex;
  align-items: center; 
  margin-top: 8px;
  border-top: 1px solid #eee;
  padding-top: 8px;
}

.remove-weight-btn {
  flex-shrink: 0;
}
</style>