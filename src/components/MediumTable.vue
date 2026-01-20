<template>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    <div
        v-for="article in articles"
        :key="article.link"
        class="bg-white/70 dark:bg-slate-900/70 backdrop-blur-md rounded-2xl shadow-lg border border-slate-200/50 dark:border-slate-700/50 overflow-hidden hover:shadow-xl hover:scale-[1.02] transition-all duration-300 cursor-pointer flex flex-col group"
        @click="$emit('select', article)"
    >
      <div class="aspect-video w-full overflow-hidden bg-slate-100 dark:bg-slate-800 relative">
         <img v-if="article.thumbnail" :src="article.thumbnail" class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110" loading="lazy" />
         <div v-else class="w-full h-full flex items-center justify-center text-slate-400 dark:text-slate-500">
            <svg class="h-12 w-12" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 00-2 2z" />
            </svg>
         </div>
         <div class="absolute top-3 left-3 flex gap-1 flex-wrap">
            <span v-for="cat in article.categories.slice(0, 2)" :key="cat" class="bg-white/90 dark:bg-slate-800/90 backdrop-blur-sm text-[10px] font-bold uppercase tracking-wider px-2 py-1 rounded text-slate-600 dark:text-slate-300 shadow-sm">
                {{ cat }}
            </span>
         </div>
      </div>
      <div class="p-5 flex-1 flex flex-col">
        <h3 class="text-lg font-bold text-slate-900 dark:text-white group-hover:text-green-600 dark:group-hover:text-green-400 transition-colors line-clamp-2 leading-tight mb-2">
            {{ article.title }}
        </h3>
        <div class="mt-auto pt-4 flex items-center justify-between">
            <div class="flex items-center gap-2">
                <div class="h-6 w-6 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center text-[10px] font-bold text-slate-500 dark:text-slate-400">
                    {{ article.author.charAt(0) }}
                </div>
                <span class="text-xs font-medium text-slate-500 dark:text-slate-400">{{ article.author }}</span>
            </div>
            <span class="text-[10px] font-medium text-slate-400 dark:text-slate-500 uppercase tracking-widest">{{ formatDate(article.pub_date) }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type MediumArticle } from '../services/tauriApi';

defineProps<{
  articles: MediumArticle[];
}>();

defineEmits<{
  (e: 'select', article: MediumArticle): void;
}>();

function formatDate(dateStr: string): string {
  if (!dateStr) return '';
  const date = new Date(dateStr);
  return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(date);
}
</script>
