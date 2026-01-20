<template>
  <div class="fixed inset-0 z-50 flex justify-end" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">
    <div class="drawer-backdrop fixed inset-0 bg-slate-900/50 dark:bg-slate-950/70 backdrop-blur-sm transition-opacity" aria-hidden="true" @click="$emit('close')"></div>

    <div class="drawer-panel w-screen max-w-2xl transform transition-transform bg-[var(--color-bg-secondary)] shadow-2xl flex flex-col h-full relative z-10">
      <div class="flex-shrink-0 px-6 py-6 border-b border-[var(--color-border-primary)] bg-[var(--color-bg-secondary)]/80 backdrop-blur-md sticky top-0 z-20 flex items-start justify-between">
        <div class="min-w-0 pr-4">
           <h2 class="text-xl font-bold text-[var(--color-text-primary)] leading-tight" id="slide-over-title">{{ post.title }}</h2>
           <div class="flex items-center gap-3 mt-4">
              <img v-if="post.author.profilePicture" :src="post.author.profilePicture" class="h-8 w-8 rounded-full" />
              <div class="text-sm">
                 <p class="font-bold text-[var(--color-text-primary)]">{{ post.author.name }}</p>
                 <p class="text-[var(--color-text-tertiary)]">@{{ post.author.username }} • {{ formatDate(post.publishedAt) }}</p>
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
         <img v-if="post.coverImage" :src="post.coverImage.url" class="w-full h-72 object-cover" />
         <div class="p-8">
             <div class="bg-blue-50 dark:bg-blue-900/30 border-l-4 border-blue-500 p-6 rounded-r-xl mb-8">
                 <p class="text-[var(--color-text-secondary)] italic leading-relaxed">{{ post.brief }}</p>
             </div>

             <div class="grid grid-cols-3 gap-4 mb-12">
                <div class="bg-[var(--color-bg-tertiary)] p-4 rounded-xl text-center">
                    <p class="text-2xl font-bold text-[var(--color-text-primary)]">{{ post.reactionCount }}</p>
                    <p class="text-xs font-semibold text-[var(--color-text-muted)] uppercase tracking-widest mt-1">Reactions</p>
                </div>
                <div class="bg-[var(--color-bg-tertiary)] p-4 rounded-xl text-center">
                    <p class="text-2xl font-bold text-[var(--color-text-primary)]">{{ post.responseCount }}</p>
                    <p class="text-xs font-semibold text-[var(--color-text-muted)] uppercase tracking-widest mt-1">Comments</p>
                </div>
                <div class="bg-[var(--color-bg-tertiary)] p-4 rounded-xl text-center">
                    <p class="text-2xl font-bold text-[var(--color-text-primary)]">{{ post.readTimeInMinutes }}m</p>
                    <p class="text-xs font-semibold text-[var(--color-text-muted)] uppercase tracking-widest mt-1">Reading Time</p>
                </div>
             </div>

             <div class="pt-8 border-t border-[var(--color-border-primary)] flex flex-col items-center">
                <p class="text-[var(--color-text-tertiary)] mb-6 font-medium text-center">Hashnode provides a full reading experience for this post.</p>
                <a :href="post.url" target="_blank" class="bg-blue-600 dark:bg-blue-500 text-white px-10 py-4 rounded-2xl font-bold hover:bg-blue-700 dark:hover:bg-blue-600 transition-all shadow-lg hover:shadow-blue-500/20 active:scale-95">
                    Read Full Post on Hashnode ↗
                </a>
             </div>
         </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type HashnodePost } from '../services/tauriApi';

defineProps<{
  post: HashnodePost;
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
