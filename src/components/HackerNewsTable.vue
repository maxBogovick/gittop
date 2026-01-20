<template>
  <div class="overflow-hidden bg-white/70 dark:bg-slate-900/70 backdrop-blur-md shadow-lg border border-slate-200/50 dark:border-slate-700/50 rounded-2xl">
    <table class="min-w-full divide-y divide-slate-200/50 dark:divide-slate-700/50">
      <thead class="bg-slate-50/50 dark:bg-slate-800/50">
        <tr>
          <th scope="col" class="px-6 py-4 text-left text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Story</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Score</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Comments</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Time</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-200/50 dark:divide-slate-700/50">
        <tr
          v-for="story in stories"
          :key="story.id"
          class="hover:bg-white/50 dark:hover:bg-slate-800/50 transition-all duration-150 cursor-pointer group"
          @click="$emit('select', story)"
        >
          <td class="px-6 py-4">
            <div class="min-w-0">
              <div class="text-sm font-bold text-slate-900 dark:text-white group-hover:text-orange-600 dark:group-hover:text-orange-400 transition-colors line-clamp-2">
                {{ story.title }}
              </div>
              <div class="text-xs text-slate-500 dark:text-slate-400 mt-1 flex items-center gap-2">
                <span>by {{ story.by }}</span>
                <span v-if="getHost(story.url)" class="text-slate-400 dark:text-slate-500">({{ getHost(story.url) }})</span>
              </div>
            </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-700 dark:text-slate-300 text-right font-medium">
             <div class="flex items-center justify-end gap-1">
                {{ story.score }}
                <svg class="h-3 w-3 text-orange-400" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M14.707 12.707a1 1 0 01-1.414 0L10 9.414l-3.293 3.293a1 1 0 01-1.414-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 010 1.414z" clip-rule="evenodd" /></svg>
             </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-700 dark:text-slate-300 text-right">
            {{ story.descendants }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500 dark:text-slate-400 text-right">
            {{ formatTime(story.time) }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { type HackerNewsStory } from '../services/tauriApi';

defineProps<{
  stories: HackerNewsStory[];
}>();

defineEmits<{
  (e: 'select', story: HackerNewsStory): void;
}>();

function formatTime(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  const now = new Date();
  const diffHours = Math.floor((now.getTime() - date.getTime()) / (1000 * 60 * 60));

  if (diffHours < 1) return 'Just now';
  if (diffHours < 24) return `${diffHours}h ago`;
  return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(date);
}

function getHost(url?: string): string {
  if (!url) return '';
  try {
    return new URL(url).hostname.replace('www.', '');
  } catch {
    return '';
  }
}
</script>
