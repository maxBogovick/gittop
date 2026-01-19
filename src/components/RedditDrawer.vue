<template>
  <div class="fixed inset-0 z-50 flex justify-end" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">
    <!-- Backdrop -->
    <div class="drawer-backdrop fixed inset-0 bg-slate-900/30 backdrop-blur-sm transition-opacity" aria-hidden="true" @click="$emit('close')"></div>

    <!-- Panel -->
    <div class="drawer-panel w-screen max-w-2xl transform transition-transform bg-white shadow-2xl flex flex-col h-full relative z-10">
      
      <!-- Header -->
      <div class="flex-shrink-0 px-6 py-4 border-b border-slate-100 bg-white/80 backdrop-blur-md sticky top-0 z-20 flex items-start justify-between">
        <div class="min-w-0 pr-4">
           <h2 class="text-lg font-bold text-slate-900 leading-snug" id="slide-over-title">{{ post.title }}</h2>
           <p class="text-sm text-slate-500 mt-1">
             <span class="font-medium text-orange-600">r/{{ post.subreddit }}</span> • Posted by u/{{ post.author }}
           </p>
        </div>
        <button type="button" class="rounded-full p-2 text-slate-400 hover:text-slate-600 hover:bg-slate-100 transition-colors focus:outline-none flex-shrink-0" @click="$emit('close')">
          <span class="sr-only">Close panel</span>
          <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Actions -->
      <div class="px-6 py-3 bg-slate-50 border-b border-slate-100 flex gap-2">
         <a :href="post.permalink" target="_blank" class="inline-flex items-center px-3 py-1.5 border border-slate-200 text-xs font-medium rounded-md text-slate-700 bg-white hover:bg-slate-50 hover:text-orange-600 transition-colors shadow-sm">
            View on Reddit
         </a>
         <a v-if="isExternalLink" :href="post.url" target="_blank" class="inline-flex items-center px-3 py-1.5 border border-blue-200 text-xs font-medium rounded-md text-blue-700 bg-blue-50 hover:bg-blue-100 transition-colors shadow-sm">
            Open Link ↗
         </a>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto px-6 py-6 custom-scrollbar">
         <div v-if="post.selftext" class="prose prose-slate prose-sm max-w-none hover:prose-a:text-blue-600" v-html="renderMarkdown(post.selftext)"></div>
         <div v-else-if="isImage" class="flex justify-center bg-slate-50 rounded-lg p-4">
            <img :src="decodeUrl(post.url)" referrerpolicy="no-referrer" class="max-w-full rounded shadow-sm object-contain" />
         </div>
         <div v-else class="text-center py-12 text-slate-500 bg-slate-50 rounded-lg border border-slate-100 dashed">
            <p class="mb-2 font-medium">This post is a link to:</p>
            <a :href="post.url" target="_blank" class="text-blue-600 hover:underline break-all block px-4">{{ post.url }}</a>
         </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import MarkdownIt from 'markdown-it';
import type { RedditPost } from '../services/tauriApi';

const props = defineProps<{
  post: RedditPost
}>();

defineEmits<{
  (e: 'close'): void
}>();

const md = new MarkdownIt({
    html: false,
    linkify: true,
    typographer: true
});

function renderMarkdown(text: string) {
  return md.render(text);
}

const isExternalLink = computed(() => {
  return !props.post.url.includes('reddit.com') && !props.post.url.startsWith('/');
});

const isImage = computed(() => {
  return /\.(jpg|jpeg|png|gif|webp)$/i.test(props.post.url);
});

function decodeUrl(url: string): string {
  return url.replace(/&amp;/g, '&');
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
  background-color: #cbd5e1;
  border-radius: 20px;
}
</style>
