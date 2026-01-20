<template>
  <div class="relative group">
    <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
      <svg class="h-5 w-5 text-slate-400 dark:text-slate-500 group-focus-within:text-blue-500 dark:group-focus-within:text-blue-400 transition-colors" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd" />
      </svg>
    </div>
    <input
      ref="inputRef"
      type="text"
      :value="modelValue"
      @input="onInput"
      class="block w-full pl-10 pr-20 py-2.5 sm:text-sm bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl shadow-sm placeholder-slate-400 dark:placeholder-slate-500 text-slate-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 transition-all duration-200"
      :placeholder="placeholder || 'Search repositories...'"
    />
    <div class="absolute inset-y-0 right-0 pr-3 flex items-center gap-2">
       <button
          v-if="modelValue"
          @click="clear"
          class="text-slate-400 dark:text-slate-500 hover:text-slate-600 dark:hover:text-slate-300 focus:outline-none p-1 rounded-full hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
       >
         <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
           <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
         </svg>
       </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

defineProps<{
  modelValue?: string,
  placeholder?: string
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>();

const inputRef = ref<HTMLInputElement | null>(null);
let timeout: number | null = null;

function onInput(event: Event) {
  const target = event.target as HTMLInputElement;
  const val = target.value;

  if (timeout) clearTimeout(timeout);

  timeout = setTimeout(() => {
    emit('update:modelValue', val);
  }, 500) as unknown as number;
}

function clear() {
  emit('update:modelValue', '');
  inputRef.value?.focus();
}
</script>
