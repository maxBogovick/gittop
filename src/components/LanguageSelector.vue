<template>
  <div class="relative inline-block text-left">
    <select
      :value="modelValue"
      @input="updateValue"
      class="block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm rounded-md bg-white text-gray-700 border"
    >
      <option value="">All Languages</option>
      <option v-for="lang in languages" :key="lang" :value="lang">
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
