<template>
  <div class="overflow-x-auto bg-white/70 dark:bg-slate-900/70 backdrop-blur-md shadow-lg border border-slate-200/50 dark:border-slate-700/50 rounded-2xl">
    <table class="min-w-full divide-y divide-slate-200/50 dark:divide-slate-700/50">
      <thead class="bg-slate-50/50 dark:bg-slate-800/50">
        <tr>
          <th scope="col" class="px-6 py-4 text-left text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Post</th>
          <th scope="col" class="px-6 py-4 text-left text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Subreddit</th>
          <th
            scope="col"
            class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider cursor-pointer hover:text-slate-700 dark:hover:text-slate-200 transition-colors"
            @click="$emit('sort', 'score')"
          >
            Score
            <span v-if="currentSort === 'score'" class="ml-1 text-blue-500">▼</span>
          </th>
          <th
            scope="col"
            class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider cursor-pointer hover:text-slate-700 dark:hover:text-slate-200 transition-colors"
            @click="$emit('sort', 'comments')"
          >
            Comments
            <span v-if="currentSort === 'comments'" class="ml-1 text-blue-500">▼</span>
          </th>
          <th
            scope="col"
            class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider cursor-pointer hover:text-slate-700 dark:hover:text-slate-200 transition-colors"
            @click="$emit('sort', 'date')"
          >
            Posted
            <span v-if="currentSort === 'date'" class="ml-1 text-blue-500">▼</span>
          </th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-200/50 dark:divide-slate-700/50">
        <tr
          v-for="post in posts"
          :key="post.id"
          class="hover:bg-white/50 dark:hover:bg-slate-800/50 transition-all duration-150 cursor-pointer group"
          @click="$emit('select', post)"
        >
          <td class="px-6 py-4">
            <div class="flex items-start">
              <div class="flex-shrink-0 h-12 w-12 mr-4" v-if="post.thumbnail && post.thumbnail !== 'self' && post.thumbnail !== 'default'">
                 <img :src="decodeUrl(post.thumbnail)" referrerpolicy="no-referrer" class="h-12 w-12 rounded-lg object-cover border border-slate-200 dark:border-slate-700" @error="($event.target as HTMLImageElement).style.display='none'" />
              </div>
              <div>
                <div class="text-sm font-bold text-slate-900 dark:text-white group-hover:text-orange-600 dark:group-hover:text-orange-400 transition-colors line-clamp-2">
                  {{ post.title }}
                </div>
                <div class="text-xs text-slate-500 dark:text-slate-400 mt-1">
                  by u/{{ post.author }}
                </div>
              </div>
            </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap">
             <span class="px-2.5 py-0.5 inline-flex text-xs leading-5 font-bold rounded-md bg-orange-50 dark:bg-orange-900/20 text-orange-700 dark:text-orange-300 ring-1 ring-inset ring-orange-600/10 dark:ring-orange-400/20">
               r/{{ post.subreddit }}
             </span>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-700 dark:text-slate-300 text-right font-medium">
            {{ formatNumber(post.score) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-700 dark:text-slate-300 text-right font-medium">
            {{ formatNumber(post.num_comments) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500 dark:text-slate-400 text-right">
            {{ formatDate(post.created_utc) }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import type { RedditPost } from '../services/tauriApi';

defineProps<{
  posts: RedditPost[];
  currentSort?: string;
}>();

defineEmits<{
  (e: 'select', post: RedditPost): void;
  (e: 'sort', column: string): void;
}>();

function formatNumber(num: number): string {
  return new Intl.NumberFormat('en-US', { notation: "compact", compactDisplay: "short" }).format(num);
}

function formatDate(dateStr: string): string {
  if (!dateStr) return '';
  const date = new Date(dateStr);
  return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(date);
}

function decodeUrl(url: string): string {
  return url.replace(/&amp;/g, '&');
}
</script>
