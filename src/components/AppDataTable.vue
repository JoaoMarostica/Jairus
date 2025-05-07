<template>
  <n-grid cols="1 m:6" responsive="screen" x-gap="16" y-gap="16">
    <n-grid-item span="m:3">
      <n-input v-model:value="search" placeholder="Buscar..." clearable />
    </n-grid-item>
    <n-grid-item span="m:1.5" v-if="categoryKey">
      <n-select
        v-model:value="selectedCategory"
        :options="categoryOptions"
        placeholder="Filtro"
        clearable
      />
    </n-grid-item>
  </n-grid>

  <div class="table-wrapper">
    <n-data-table
      :columns="columns"
      :data="filteredData"
      :pagination="{ pageSize }"
      scroll-x="max-content"
    />
  </div>
</template>

<script setup lang="ts">
import { NGrid, NGridItem, NInput, NSelect, NDataTable } from 'naive-ui'
import { ref, computed } from 'vue'

const props = defineProps<{
  columns: any[],
  data: Record<string, any>[],
  categoryKey?: string,
  pageSize?: number
}>()

const pageSize = ref(props.pageSize ?? 12)

const search = ref('')
const selectedCategory = ref<string | null>(null)

const categoryOptions = computed(() => {
  if (!props.categoryKey) return []
  const unique = [...new Set(props.data.map(d => d[props.categoryKey!]))]
  return unique.map(c => ({ label: c, value: c }))
})

const normalizeText = (text: string | null) => {
  if (!text) return ''

  // Normaliza o texto: remove espaços extras, converte para minúsculas, remove acentos
  return text.trim().toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, "");
}

const filteredData = computed(() => {
  let result = [...props.data];

  // Filtro por categoria
  if (props.categoryKey && selectedCategory.value) {
    result = result.filter(item =>
      normalizeText(String(item[props.categoryKey!])) === normalizeText(selectedCategory.value)
    );
  }

  // Filtro de busca por qualquer valor nos campos relevantes
  if (search.value) {
    const searchText = normalizeText(search.value);

    result = result.filter(item =>
      Object.values(item).some(val =>
        normalizeText(String(val)).includes(searchText)
      )
    );
  }

  return result;
})
</script>

<style scoped>
.table-wrapper {
  margin-top: 30px;
}
</style>
