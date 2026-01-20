<template>
  <div class="flex space-x-2">
    <button
      v-for="m in metrics"
      :key="m"
      @click="select(m)"
      :class="[
        'px-3 py-1.5 rounded-lg text-sm font-medium cursor-pointer transition-colors',
        modelValue === m
          ? 'bg-emerald-600 dark:bg-emerald-500 text-white shadow-sm'
          : 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-700 hover:text-slate-900 dark:hover:text-white'
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
