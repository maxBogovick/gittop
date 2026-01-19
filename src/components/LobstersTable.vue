<template>
  <div class="overflow-hidden bg-white shadow-sm border border-slate-200 rounded-xl">
    <table class="min-w-full divide-y divide-slate-200">
      <thead class="bg-slate-50/80">
        <tr>
          <th scope="col" class="px-6 py-4 text-left text-xs font-semibold text-slate-500 uppercase tracking-wider">Story</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-semibold text-slate-500 uppercase tracking-wider">Score</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-semibold text-slate-500 uppercase tracking-wider">Comments</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-semibold text-slate-500 uppercase tracking-wider">Created</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-100 bg-white">
        <tr v-for="story in stories" :key="story.short_id" class="hover:bg-slate-50/80 transition-all duration-150 cursor-pointer group" @click="$emit('select', story)">
          <td class="px-6 py-4">
            <div class="min-w-0">
              <div class="text-sm font-semibold text-slate-900 group-hover:text-red-600 transition-colors line-clamp-2">{{ story.title }}</div>
              <div class="flex flex-wrap gap-1 mt-2">
                <span v-for="tag in story.tags" :key="tag" class="px-2 py-0.5 bg-slate-100 text-slate-500 text-[10px] font-black uppercase rounded border border-slate-200">{{ tag }}</span>
              </div>
            </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-600 text-right font-bold">{{ story.score }}</td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-600 text-right">💬 {{ story.comment_count }}</td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500 text-right">{{ formatDate(story.created_at) }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { type LobstersStory } from '../services/tauriApi';
defineProps<{ stories: LobstersStory[] }>();
defineEmits<{ (e: 'select', story: LobstersStory): void }>();

function formatDate(dateStr: string): string {
  const date = new Date(dateStr);
  return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(date);
}
</script>