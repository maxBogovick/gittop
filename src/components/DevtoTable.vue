<template>
  <div class="overflow-hidden bg-white/70 dark:bg-slate-900/70 backdrop-blur-md shadow-lg border border-slate-200/50 dark:border-slate-700/50 rounded-2xl">
    <table class="min-w-full divide-y divide-slate-200/50 dark:divide-slate-700/50">
      <thead class="bg-slate-50/50 dark:bg-slate-800/50">
        <tr>
          <th scope="col" class="px-6 py-4 text-left text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Article</th>
          <th scope="col" class="px-6 py-4 text-left text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Tags</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Reactions</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Comments</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Reading Time</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Published</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-200/50 dark:divide-slate-700/50">
        <tr
          v-for="article in articles"
          :key="article.id"
          class="hover:bg-white/50 dark:hover:bg-slate-800/50 transition-all duration-150 cursor-pointer group"
          @click="$emit('select', article)"
        >
          <td class="px-6 py-4">
            <div class="flex items-center">
              <div class="flex-shrink-0 h-10 w-10">
                <img v-if="article.user.profile_image" class="h-10 w-10 rounded-full object-cover ring-2 ring-white dark:ring-slate-800 shadow-sm" :src="article.user.profile_image" alt="" />
                <div v-else class="h-10 w-10 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center text-slate-500 dark:text-slate-400 font-bold ring-2 ring-white dark:ring-slate-800 shadow-sm">
                  {{ article.user.username.charAt(0).toUpperCase() }}
                </div>
              </div>
              <div class="ml-4">
                <div class="text-sm font-bold text-slate-900 dark:text-white group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors line-clamp-2" :title="article.title">
                  {{ article.title }}
                </div>
                <div class="text-sm text-slate-500 dark:text-slate-400">
                  by {{ article.user.name }}
                </div>
              </div>
            </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap">
             <div class="flex gap-1 flex-wrap max-w-xs">
                <span v-for="tag in article.tag_list" :key="tag" class="px-2 py-0.5 inline-flex text-xs leading-5 font-medium rounded-md bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-300 ring-1 ring-inset ring-blue-700/10 dark:ring-blue-400/20">
                  #{{ tag }}
                </span>
             </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-700 dark:text-slate-300 text-right font-medium">
            {{ formatNumber(article.public_reactions_count) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-700 dark:text-slate-300 text-right font-medium">
            {{ formatNumber(article.comments_count) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-700 dark:text-slate-300 text-right">
            {{ article.reading_time_minutes }} min
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500 dark:text-slate-400 text-right">
            {{ formatDate(article.published_at) }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { type DevtoArticle } from '../services/tauriApi';

defineProps<{
  articles: DevtoArticle[];
}>();

defineEmits<{
  (e: 'select', article: DevtoArticle): void;
}>();

function formatNumber(num: number): string {
  return new Intl.NumberFormat('en-US', { notation: "compact", compactDisplay: "short" }).format(num);
}

function formatDate(dateStr: string): string {
  if (!dateStr) return '';
  const date = new Date(dateStr);
  return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(date);
}
</script>
