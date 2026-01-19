<template>
  <div class="overflow-hidden bg-white shadow-sm border border-slate-200 rounded-xl">
    <table class="min-w-full divide-y divide-slate-200">
      <thead class="bg-slate-50/80">
        <tr>
          <th scope="col" class="px-6 py-4 text-left text-xs font-semibold text-slate-500 uppercase tracking-wider">Story</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-semibold text-slate-500 uppercase tracking-wider">Score</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-semibold text-slate-500 uppercase tracking-wider">Comments</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-semibold text-slate-500 uppercase tracking-wider">Time</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-100 bg-white">
        <tr 
          v-for="story in stories" 
          :key="story.id" 
          class="hover:bg-slate-50/80 transition-all duration-150 cursor-pointer group"
          @click="$emit('select', story)"
        >
          <td class="px-6 py-4">
            <div class="min-w-0">
              <div class="text-sm font-semibold text-slate-900 group-hover:text-orange-600 transition-colors line-clamp-2">
                {{ story.title }}
              </div>
              <div class="text-xs text-slate-500 mt-1 flex items-center gap-2">
                <span>by {{ story.by }}</span>
                <span v-if="getHost(story.url)" class="text-slate-400">({{ getHost(story.url) }})</span>
              </div>
            </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-600 text-right font-medium">
             <div class="flex items-center justify-end gap-1">
                {{ story.score }}
                <svg class="h-3 w-3 text-orange-400" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M14.707 12.707a1 1 0 01-1.414 0L10 9.414l-3.293 3.293a1 1 0 01-1.414-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 010 1.414z" clip-rule="evenodd" /></svg>
             </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-600 text-right">
            💬 {{ story.descendants }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500 text-right">
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