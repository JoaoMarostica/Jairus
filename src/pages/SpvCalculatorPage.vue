<template>
  <n-flex vertical gap="16px" class="p-6">
    <n-card title="Análise Técnica Sementes Puras e Viáveis / m²" size="medium">
      <n-form
        ref="formRef"
        :model="formModel"
        label-placement="top"
        require-mark-placement="right-hanging"
        :size="size"
        label-width="auto"
      >
        <n-form-item label="Cultivar">
          <n-select
            v-model:value="formModel.cultivarName"
            :options="cultivars"
            :menu-size="size"
            filterable
            clearable
            placeholder="Escolha a cultivar"
          />
        </n-form-item>

        <n-form-item label="Kilogramas por hectare (Kg/ha)">
          <n-input-number
            v-model:value="formModel.weightPerHectare"
            placeholder="Digite o Kg/ha" 
          >
            <template #suffix>
              <span>Kg/ha</span>
            </template>
          </n-input-number>
        </n-form-item>

        <n-form-item label="Linha da cultivar">
          <n-select
            v-model:value="formModel.cultivarLine"
            :options="cultivarLines"
            :menu-size="size"
            filterable
            clearable
            placeholder="Escolha a linha da cultivar"
          />
        </n-form-item>

        <n-form-item label="Método de plantio">
          <n-select
            v-model:value="formModel.plantationMethod"
            :options="plantationMethods"
            :menu-size="size"
            filterable
            clearable
            placeholder="Ecolha o método de plantio"
          />
        </n-form-item>

        <n-form-item
          label="Espaçamento entre linhas (cm)"
          v-if="formModel.plantationMethod === 'inLine'"
        >
          <n-input-number 
            v-model:value="formModel.plantationLineSpacing" 
            placeholder="Digite o espaçamento entre linhas" 
          />
        </n-form-item>

        <n-form-item label="Ponto de pureza (%)">
          <n-input-number 
            v-model:value="formModel.purityPoint" 
            placeholder="Digite o ponto de pureza" 
          >
            <template #suffix>
              <span>%</span>
            </template>
          </n-input-number>
        </n-form-item>

        <n-form-item label="Viabilidade em tetrazólio (%)">
          <n-input-number 
            v-model:value="formModel.viability" 
            placeholder="Digite a viabilidade" 
          >
            <template #suffix>
              <span>%</span>
            </template>
          </n-input-number>
        </n-form-item>

        <n-form-item label="Pegamento de plantas (%)">
          <n-input-number 
            v-model:value="formModel.germination" 
            placeholder="Digite o pegamento" 
          >
            <template #suffix>
              <span>%</span>
            </template>
          </n-input-number>
        </n-form-item>

        <n-form-item label="Preço por kilograma (R$)">
          <n-input-number 
            v-model:value="formModel.pricePerKg" 
            placeholder="Digite o preço/Kg"
          >
            <template #prefix>
              <span>R$</span>
            </template>
          </n-input-number>
        </n-form-item>

        <div style="display: flex; justify-content: center">
          <n-button round type="primary" @click="calculate">
            Calcular
          </n-button>
        </div>
      </n-form>

      <n-card v-if="resultAvailable" title="Resultados" size="small">
        <p v-if="formModel.plantationMethod === 'throw'">
          <strong>Sementes puras e viáveis por m²:</strong> {{ result.pureAndViableSeedsPerM2.toFixed(2) }}
        </p>
        <p v-if="formModel.plantationMethod === 'inLine'">
          <strong>Sementes puras e viáveis por metro linear:</strong> {{ result.pureAndViableSeedsPerLinearM.toFixed(2) }}
        </p>
        <p v-if="formModel.plantationMethod === 'throw'">
          <strong>Estimativa de plantas estabelecidas por m²:</strong> {{ result.establishedPlantsPerM2.toFixed(2) }}
        </p>
        <p v-if="formModel.plantationMethod === 'inLine'">
          <strong>Estimativa de plantas estabelecidas por metro linear:</strong> {{ result.establishedPlantsPerLinearM.toFixed(2) }}
        </p>
        <p>
          <strong>Investmento por hectare:</strong> R$ {{ result.investmentPerHectare.toFixed(2) }}
        </p>
      </n-card>
    </n-card>
  </n-flex>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useCultivarsStore } from '@/stores/cultivarsStore'
import { 
  NForm, NFormItem, NSelect, NInputNumber, NButton, NFlex, NCard 
} from 'naive-ui'
import { storeToRefs } from 'pinia'

// Form model
const formModel = ref({
  cultivarName: null as string | null,
  weightPerHectare: null as number | null,
  cultivarLine: null as string | null,
  plantationMethod: null as string | null,
  plantationLineSpacing: null as number | null,
  purityPoint: null as number | null,
  viability: null as number | null,
  germination: null as number | null,
  pricePerKg: null as number | null
})

const cultivarsStore = useCultivarsStore()
const { cultivars } = storeToRefs(cultivarsStore)

const cultivarLines = ref([
  { label: 'Golden', value: 'golden' },
  { label: 'Podium', value: 'podium' },
  { label: 'Conventional', value: 'conventional' }
])

const plantationMethods = ref([
  { label: 'À lanço', value: 'throw' },
  { label: 'Em linha', value: 'inLine' }
])

const result = ref({
  pureAndViableSeedsPerM2: 0,
  pureAndViableSeedsPerLinearM: 0,
  establishedPlantsPerM2: 0,
  establishedPlantsPerLinearM: 0,
  investmentPerHectare: 0,
})

const resultAvailable = ref(false)

const size = ref<'small' | 'medium' | 'large'>('medium')

onMounted(() => {
  cultivarsStore.fetchCultivars()
})

function calculate() {
  if (
    formModel.value.cultivarName != null &&
    formModel.value.weightPerHectare != null &&
    formModel.value.cultivarLine != null &&
    formModel.value.plantationMethod != null &&
    formModel.value.plantationLineSpacing != null &&
    formModel.value.purityPoint != null &&
    formModel.value.viability != null &&
    formModel.value.germination != null &&
    formModel.value.pricePerKg != null
  ) {
    const pms = formModel.value.plantationMethod === 'throw' ? 0.5 : 1
    // const pureSeedsPerGram = (1000 / pms) * (formModel.value.purityPoint / 100)
    // const pureAndViableSeedsPerGram = pureSeedsPerGram * (formModel.value.viability / 100)
    const pureAndViableSeedsPerM2 = ((1000 / pms) * (formModel.value.viability / 100)) * ((formModel.value.weightPerHectare * 1000) / 10000)
    const pureAndViableSeedsPerLinearM = pureAndViableSeedsPerM2 / formModel.value.plantationLineSpacing
    const establishedPlantsPerM2 = pureAndViableSeedsPerM2 * (formModel.value.germination / 100)
    const establishedPlantsPerLinearM = pureAndViableSeedsPerLinearM * (formModel.value.germination / 100)
    const investmentPerHectare = formModel.value.pricePerKg * formModel.value.weightPerHectare

    result.value.pureAndViableSeedsPerM2 = pureAndViableSeedsPerM2
    result.value.pureAndViableSeedsPerLinearM = pureAndViableSeedsPerLinearM
    result.value.establishedPlantsPerM2 = establishedPlantsPerM2
    result.value.establishedPlantsPerLinearM = establishedPlantsPerLinearM
    result.value.investmentPerHectare = investmentPerHectare
    resultAvailable.value = true
  } else {
    resultAvailable.value = false
  }
}

</script>

<style scoped>
</style>
