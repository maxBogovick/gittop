<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 overflow-y-auto" role="dialog" aria-modal="true">
    <!-- Backdrop -->
    <div class="fixed inset-0 bg-black/50 backdrop-blur-sm transition-opacity" @click="$emit('close')"></div>

    <!-- Modal Panel -->
    <div class="flex min-h-full items-start justify-center p-4 pt-16 sm:p-6 sm:pt-24">
      <div 
        class="relative w-full max-w-2xl transform overflow-hidden rounded-2xl bg-white/90 dark:bg-slate-800/90 backdrop-blur-xl shadow-2xl transition-all border border-slate-200/50 dark:border-slate-700/50"
        @click.stop
      >
        <!-- Search Input -->
        <div class="relative flex items-center border-b border-slate-200/50 dark:border-slate-700/50">
          <svg class="absolute left-4 h-5 w-5 text-slate-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd" />
          </svg>
          <input
            ref="inputRef"
            type="text"
            v-model="query"
            @keydown.enter="handleSearch"
            @keydown.esc="$emit('close')"
            class="h-14 w-full bg-transparent pl-12 pr-4 text-slate-900 dark:text-white placeholder-slate-400 focus:outline-none sm:text-lg"
            placeholder="Search across GitHub, Reddit, StackOverflow..."
            autocomplete="off"
          />
          <div class="absolute right-4 flex items-center gap-2">
            <kbd class="hidden sm:inline-flex items-center gap-1 rounded border border-slate-200 dark:border-slate-700 bg-slate-100 dark:bg-slate-900/50 px-2 py-0.5 text-xs font-medium text-slate-500">
              ESC
            </kbd>
          </div>
        </div>

        <!-- Quick Links / Recent (Placeholder) -->
        <div class="px-4 py-3 bg-slate-50/50 dark:bg-slate-900/30 text-xs text-slate-500 flex justify-between">
            <span>Press Enter to search</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue';
import { useRouter } from 'vue-router';

const props = defineProps<{
  isOpen: boolean;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
}>();

const router = useRouter();
const query = ref('');
const inputRef = ref<HTMLInputElement | null>(null);

watch(() => props.isOpen, async (newVal) => {
  if (newVal) {
    query.value = '';
    await nextTick();
    inputRef.value?.focus();
  }
});

const handleSearch = () => {
  if (query.value.trim()) {
    router.push({ name: 'search', query: { q: query.value.trim() } });
    emit('close');
  }
};

// Handle global keydown for Cmd+K here? 
// No, let parent handle visibility, but we can prevent propagation if open.
</script>
