<template>
  <div class="relative inline-block text-left">
    <select
      :value="modelValue"
      @input="updateValue"
      class="block w-full pl-3 pr-10 py-2 text-sm rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white border border-slate-200 dark:border-slate-700 focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 transition-colors appearance-none cursor-pointer"
    >
      <option value="" class="bg-white dark:bg-slate-800">All Languages</option>
      <option v-for="lang in languages" :key="lang" :value="lang" class="bg-white dark:bg-slate-800">
        {{ lang }}
      </option>
    </select>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  modelValue?: string
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>();

const languages = [
  "Rust",
  "TypeScript",
  "JavaScript",
  "Python",
  "Go",
  "Java",
  "C++",
  "C#",
  "Swift",
  "Kotlin",
  "PHP",
  "Ruby"
];

function updateValue(event: Event) {
  const target = event.target as HTMLSelectElement;
  emit('update:modelValue', target.value);
}
</script>
