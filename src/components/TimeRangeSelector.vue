<template>
  <div class="flex space-x-2">
    <button
      v-for="range in ranges"
      :key="range.value"
      @click="select(range.value)"
      :class="[
        'px-3 py-1.5 rounded-lg text-sm font-medium cursor-pointer transition-colors',
        modelValue === range.value
          ? 'bg-blue-600 dark:bg-blue-500 text-white shadow-sm'
          : 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-700 hover:text-slate-900 dark:hover:text-white'
      ]"
    >
      {{ range.label }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { TimeRange } from '../services/tauriApi';

defineProps<{
  modelValue: TimeRange
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: TimeRange): void
}>();

const ranges = [
  { label: 'Day', value: TimeRange.Day },
  { label: 'Week', value: TimeRange.Week },
  { label: 'Month', value: TimeRange.Month },
  { label: 'Year', value: TimeRange.Year },
];

function select(value: TimeRange) {
  emit('update:modelValue', value);
}
</script>
