<template>
  <div class="fixed inset-0 z-50 flex justify-end" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">
    <div class="drawer-backdrop fixed inset-0 bg-slate-900/50 dark:bg-slate-950/70 backdrop-blur-sm transition-opacity" aria-hidden="true" @click="$emit('close')"></div>

    <div class="drawer-panel w-screen max-w-2xl transform transition-transform bg-[var(--color-bg-secondary)] shadow-2xl flex flex-col h-full relative z-10">
      <div class="flex-shrink-0 px-6 py-6 border-b border-[var(--color-border-primary)] bg-[var(--color-bg-secondary)]/80 backdrop-blur-md sticky top-0 z-20 flex items-start justify-between">
        <div class="min-w-0 pr-4">
           <h2 class="text-xl font-bold text-[var(--color-text-primary)] leading-tight" id="slide-over-title">{{ post.title }}</h2>
           <div class="flex items-center gap-3 mt-4">
              <div class="h-10 w-10 rounded-full bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white font-bold text-lg">
                {{ post.author.charAt(0).toUpperCase() }}
              </div>
              <div class="text-sm">
                 <p class="font-bold text-[var(--color-text-primary)]">{{ post.author }}</p>
                 <p class="text-[var(--color-text-tertiary)]">{{ formatDate(post.pub_date) }}</p>
              </div>
           </div>
        </div>
        <button type="button" class="rounded-full p-2 text-[var(--color-text-muted)] hover:text-[var(--color-text-secondary)] hover:bg-[var(--color-bg-hover)] transition-colors focus:outline-none flex-shrink-0" @click="$emit('close')">
          <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <div class="flex-1 overflow-y-auto custom-scrollbar">
         <img v-if="post.thumbnail" :src="post.thumbnail" class="w-full h-72 object-cover" />
         <div class="p-8">
             <div v-if="post.content" class="prose prose-slate dark:prose-invert max-w-none text-[var(--color-text-secondary)] leading-relaxed mb-8">
                 <p>{{ post.content }}</p>
             </div>

             <div class="pt-8 border-t border-[var(--color-border-primary)] flex flex-col items-center">
                <p class="text-[var(--color-text-tertiary)] mb-6 font-medium text-center">Read the full discussion on Indie Hackers.</p>
                <a :href="post.link" target="_blank" class="bg-blue-600 dark:bg-blue-500 text-white px-10 py-4 rounded-2xl font-bold hover:bg-blue-700 dark:hover:bg-blue-600 transition-all shadow-lg hover:shadow-blue-500/20 active:scale-95">
                    Open on Indie Hackers
                </a>
             </div>
         </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type IndieHackersPost } from '../services/tauriApi';

defineProps<{
  post: IndieHackersPost;
}>();

defineEmits<{
  (e: 'close'): void;
}>();

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
</style>
