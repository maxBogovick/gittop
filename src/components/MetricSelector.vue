<template>
  <div class="flex space-x-2">
    <button
      v-for="m in metrics"
      :key="m"
      @click="select(m)"
      :class="[
        'px-3 py-1 rounded text-sm font-medium cursor-pointer',
        modelValue === m ? 'bg-green-600 text-white' : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
      ]"
    >
      {{ m }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { Metric } from '../services/tauriApi';

defineProps<{
  modelValue: Metric
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: Metric): void
}>();

const metrics = Object.values(Metric);

function select(value: Metric) {
  emit('update:modelValue', value);
}
</script>
