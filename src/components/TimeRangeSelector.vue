<template>
  <div class="flex space-x-2">
    <button
      v-for="range in ranges"
      :key="range.value"
      @click="select(range.value)"
      :class="[
        'px-3 py-1 rounded text-sm font-medium cursor-pointer',
        modelValue === range.value ? 'bg-blue-600 text-white' : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
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
