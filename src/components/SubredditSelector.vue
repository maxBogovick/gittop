<template>
  <div class="relative inline-block text-left">
    <select
      :value="modelValue"
      @input="updateValue"
      class="block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-orange-500 focus:border-orange-500 sm:text-sm rounded-md bg-white text-gray-700 border"
    >
      <option value="">All (r/all)</option>
      <option v-for="sub in subreddits" :key="sub" :value="sub">
        r/{{ sub }}
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

const subreddits = [
  "programming",
  "rust",
  "webdev",
  "javascript",
  "typescript",
  "python",
  "cpp",
  "csharp",
  "golang",
  "linux",
  "technology",
  "coding",
  "vuejs",
  "reactjs"
];

function updateValue(event: Event) {
  const target = event.target as HTMLSelectElement;
  emit('update:modelValue', target.value);
}
</script>
