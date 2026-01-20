<template>
  <div class="fixed inset-0 z-50 flex justify-end" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">
    <div class="drawer-backdrop fixed inset-0 bg-slate-900/50 dark:bg-slate-950/70 backdrop-blur-sm transition-opacity" aria-hidden="true" @click="$emit('close')"></div>

    <div class="drawer-panel w-screen max-w-2xl transform transition-transform bg-[var(--color-bg-secondary)] shadow-2xl flex flex-col h-full relative z-10">
      <div class="flex-shrink-0 px-6 py-6 border-b border-[var(--color-border-primary)] bg-[var(--color-bg-secondary)]/80 backdrop-blur-md sticky top-0 z-20 flex items-start justify-between">
        <div class="min-w-0 pr-4">
           <h2 class="text-xl font-bold text-[var(--color-text-primary)] leading-tight" id="slide-over-title">{{ article.title }}</h2>
           <div class="flex items-center gap-3 mt-3 text-sm text-[var(--color-text-tertiary)]">
              <span class="font-medium text-[var(--color-text-primary)]">{{ article.author }}</span>
              <span>•</span>
              <span>{{ formatDate(article.pub_date) }}</span>
           </div>
        </div>
        <button type="button" class="rounded-full p-2 text-[var(--color-text-muted)] hover:text-[var(--color-text-secondary)] hover:bg-[var(--color-bg-hover)] transition-colors focus:outline-none flex-shrink-0" @click="$emit('close')">
          <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <div class="flex-1 overflow-y-auto custom-scrollbar">
         <img v-if="article.thumbnail" :src="article.thumbnail" class="w-full h-64 object-cover" />
         <div class="p-8">
             <div class="prose prose-slate dark:prose-invert max-w-none text-[var(--color-text-secondary)] leading-relaxed" v-html="sanitizedContent"></div>

             <div class="mt-12 pt-8 border-t border-[var(--color-border-primary)] flex flex-col items-center">
                <p class="text-[var(--color-text-tertiary)] mb-4 font-medium">Read the full article on Medium</p>
                <a :href="article.link" target="_blank" class="bg-green-600 dark:bg-green-500 text-white px-8 py-3 rounded-xl font-bold hover:bg-green-700 dark:hover:bg-green-600 transition-colors shadow-lg">
                    Continue Reading ↗
                </a>
             </div>
         </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { type MediumArticle } from '../services/tauriApi';

const props = defineProps<{
  article: MediumArticle;
}>();

defineEmits<{
  (e: 'close'): void;
}>();

const sanitizedContent = computed(() => {
    // Remove iframely or other problematic tags from RSS if needed
    return props.article.content;
});

function formatDate(dateStr: string): string {
  if (!dateStr) return '';
  const date = new Date(dateStr);
  return new Intl.DateTimeFormat('en-US', {
    month: 'long',
    day: 'numeric',
    year: 'numeric'
  }).format(date);
}
</script>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background-color: var(--color-border-primary);
  border-radius: 20px;
}
:deep(img) {
    max-width: 100%;
    height: auto;
    border-radius: 0.75rem;
    margin: 2rem 0;
}
:deep(figure) {
    margin: 2rem 0;
}
:deep(figcaption) {
    text-align: center;
    color: var(--color-text-muted);
    font-size: 0.875rem;
    margin-top: 0.5rem;
}
</style>
