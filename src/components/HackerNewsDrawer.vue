<template>
  <div class="fixed inset-0 z-50 flex justify-end" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">
    <div class="drawer-backdrop fixed inset-0 bg-slate-900/50 dark:bg-slate-950/70 backdrop-blur-sm transition-opacity" aria-hidden="true" @click="$emit('close')"></div>

    <div class="drawer-panel w-screen max-w-2xl transform transition-transform bg-[var(--color-bg-secondary)] shadow-2xl flex flex-col h-full relative z-10">
      <div class="flex-shrink-0 px-6 py-6 border-b border-[var(--color-border-primary)] bg-[var(--color-bg-secondary)]/80 backdrop-blur-md sticky top-0 z-20 flex items-start justify-between">
        <div class="min-w-0 pr-4">
           <h2 class="text-xl font-bold text-[var(--color-text-primary)] leading-tight" id="slide-over-title">{{ story.title }}</h2>
           <div class="flex items-center gap-3 mt-3 text-sm text-[var(--color-text-tertiary)]">
              <span class="font-medium text-orange-600 dark:text-orange-400">{{ story.score }} points</span>
              <span>•</span>
              <span>by {{ story.by }}</span>
              <span>•</span>
              <span>{{ formatDate(story.time) }}</span>
           </div>
        </div>
        <button type="button" class="rounded-full p-2 text-[var(--color-text-muted)] hover:text-[var(--color-text-secondary)] hover:bg-[var(--color-bg-hover)] transition-colors focus:outline-none flex-shrink-0" @click="$emit('close')">
          <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <div class="px-6 py-6 flex flex-col gap-6 flex-1 overflow-y-auto custom-scrollbar">
         <div v-if="story.url" class="bg-[var(--color-bg-tertiary)] rounded-xl p-6 border border-[var(--color-border-primary)]">
            <h3 class="text-sm font-semibold text-[var(--color-text-muted)] uppercase tracking-wider mb-2">Original Link</h3>
            <div class="flex items-start justify-between gap-4">
                <div class="min-w-0">
                    <p class="text-[var(--color-text-primary)] font-medium break-all">{{ story.url }}</p>
                    <p class="text-[var(--color-text-tertiary)] text-sm mt-1">{{ getHost(story.url) }}</p>
                </div>
                <a :href="story.url" target="_blank" class="flex-shrink-0 bg-orange-600 text-white px-4 py-2 rounded-lg text-sm font-bold shadow-sm hover:bg-orange-700 transition-colors">
                    Open Link ↗
                </a>
            </div>
         </div>

         <div v-if="story.text" class="prose prose-slate dark:prose-invert prose-sm max-w-none text-[var(--color-text-secondary)] bg-[var(--color-bg-tertiary)] rounded-xl p-6 border border-[var(--color-border-primary)]" v-html="story.text"></div>

         <div class="flex flex-col items-center justify-center py-12 border-2 border-dashed border-[var(--color-border-primary)] rounded-xl">
            <svg class="h-12 w-12 text-[var(--color-text-muted)] mb-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.812-1.244L3 20l1.314-3.574A9.944 9.944 0 013 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
            </svg>
            <p class="text-[var(--color-text-muted)] font-medium">Comments are hosted on Hacker News</p>
            <a :href="'https://news.ycombinator.com/item?id=' + story.id" target="_blank" class="mt-4 text-orange-600 dark:text-orange-400 hover:text-orange-700 dark:hover:text-orange-300 font-bold text-sm">
                View {{ story.descendants }} comments on HN ↗
            </a>
         </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type HackerNewsStory } from '../services/tauriApi';

defineProps<{
  story: HackerNewsStory;
}>();

defineEmits<{
  (e: 'close'): void;
}>();

function formatDate(timestamp: number): string {
  if (!timestamp) return '';
  return new Intl.DateTimeFormat('en-US', {
    month: 'short',
    day: 'numeric',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(timestamp * 1000));
}

function getHost(url?: string): string {
  if (!url) return '';
  try {
    return new URL(url).hostname;
  } catch {
    return '';
  }
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
