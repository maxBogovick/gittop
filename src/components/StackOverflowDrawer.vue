<template>
  <div class="fixed inset-0 z-50 flex justify-end" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">
    <div class="drawer-backdrop fixed inset-0 bg-slate-900/50 dark:bg-slate-950/70 backdrop-blur-sm transition-opacity" aria-hidden="true" @click="$emit('close')"></div>

    <div class="drawer-panel w-screen max-w-2xl transform transition-transform bg-[var(--color-bg-secondary)] shadow-2xl flex flex-col h-full relative z-10">
      <div class="flex-shrink-0 px-6 py-4 border-b border-[var(--color-border-primary)] bg-[var(--color-bg-secondary)]/80 backdrop-blur-md sticky top-0 z-20 flex items-start justify-between">
        <div class="min-w-0 pr-4">
           <h2 class="text-lg font-bold text-[var(--color-text-primary)] leading-snug" id="slide-over-title" v-html="question.title"></h2>
           <div class="flex items-center gap-2 mt-2">
              <img v-if="question.owner.profile_image" :src="question.owner.profile_image" class="h-6 w-6 rounded-full" />
              <p class="text-sm text-[var(--color-text-tertiary)]">
                Asked by <span class="font-medium text-orange-600 dark:text-orange-400">{{ question.owner.display_name }}</span> • {{ formatDate(question.creation_date) }}
              </p>
           </div>
        </div>
        <button type="button" class="rounded-full p-2 text-[var(--color-text-muted)] hover:text-[var(--color-text-secondary)] hover:bg-[var(--color-bg-hover)] transition-colors focus:outline-none flex-shrink-0" @click="$emit('close')">
          <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <div class="px-6 py-3 bg-[var(--color-bg-tertiary)] border-b border-[var(--color-border-primary)] flex gap-4 text-sm">
         <div class="flex items-center gap-1 text-[var(--color-text-secondary)]">
            <span class="font-bold">{{ question.score }}</span> score
         </div>
         <div class="flex items-center gap-1 text-[var(--color-text-secondary)]">
            <span class="font-bold">{{ question.answer_count }}</span> answers
         </div>
         <div class="flex items-center gap-1 text-[var(--color-text-secondary)]">
            <span class="font-bold">{{ question.view_count }}</span> views
         </div>
         <a :href="question.link" target="_blank" class="ml-auto text-orange-600 dark:text-orange-400 hover:underline font-medium">
            View on Stack Overflow ↗
         </a>
      </div>

      <div class="flex-1 overflow-y-auto px-6 py-6 custom-scrollbar">
         <div v-if="question.body" class="prose prose-slate dark:prose-invert prose-sm max-w-none text-[var(--color-text-secondary)] overflow-x-auto" v-html="question.body"></div>
         <div v-else class="text-center py-12 text-[var(--color-text-tertiary)] bg-[var(--color-bg-tertiary)] rounded-lg border border-[var(--color-border-primary)] dashed">
            <p>Full content not available in preview.</p>
            <a :href="question.link" target="_blank" class="text-orange-600 dark:text-orange-400 hover:underline mt-2 inline-block">Read full question on Stack Overflow</a>
         </div>

         <div v-if="question.tags && question.tags.length" class="mt-8 pt-6 border-t border-[var(--color-border-primary)]">
             <div class="flex flex-wrap gap-2">
                 <span v-for="tag in question.tags" :key="tag" class="px-2.5 py-1 bg-orange-50 dark:bg-orange-900/30 text-orange-600 dark:text-orange-400 text-xs font-medium rounded-md border border-orange-100 dark:border-orange-800">
                     {{ tag }}
                 </span>
             </div>
         </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type StackOverflowQuestion } from '../services/tauriApi';

defineProps<{
  question: StackOverflowQuestion;
}>();

defineEmits<{
  (e: 'close'): void;
}>();

function formatDate(timestamp: number): string {
  if (!timestamp) return '';
  return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric', year: 'numeric' }).format(new Date(timestamp * 1000));
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
:deep(pre) {
    background-color: var(--color-bg-tertiary);
    padding: 1rem;
    border-radius: 0.5rem;
    border: 1px solid var(--color-border-primary);
}
:deep(code) {
    color: var(--color-text-secondary);
}
</style>
