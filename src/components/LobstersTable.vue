<template>
  <div class="overflow-hidden bg-white/70 dark:bg-slate-900/70 backdrop-blur-md shadow-lg border border-slate-200/50 dark:border-slate-700/50 rounded-2xl">
    <table class="min-w-full divide-y divide-slate-200/50 dark:divide-slate-700/50">
      <thead class="bg-slate-50/50 dark:bg-slate-800/50">
        <tr>
          <th scope="col" class="pl-6 pr-3 py-4 text-left text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider w-12">#</th>
          <th scope="col" class="px-3 py-4 text-left text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Story</th>
          <th scope="col" class="px-3 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider w-24">Score</th>
          <th scope="col" class="px-3 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider w-24">Comments</th>
          <th scope="col" class="pl-3 pr-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider w-32">Posted</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-200/50 dark:divide-slate-700/50">
        <tr v-for="(story, index) in stories" :key="story.short_id"
            class="hover:bg-white/50 dark:hover:bg-slate-800/50 transition-all duration-150 cursor-pointer group"
            @click.stop="$emit('select', story)">

          <!-- Rank / Index -->
          <td class="pl-6 pr-3 py-4 whitespace-nowrap text-xs font-medium text-slate-400 dark:text-slate-500">
              {{ index + 1 }}
          </td>

          <!-- Story Details -->
          <td class="px-3 py-4">
            <div class="min-w-0">
              <div class="flex items-baseline gap-2">
                  <div class="text-sm font-bold text-slate-900 dark:text-white group-hover:text-red-600 dark:group-hover:text-red-400 transition-colors line-clamp-2 leading-snug">
                    {{ story.title }}
                  </div>
                  <a :href="story.url" target="_blank" @click.stop class="text-[10px] text-slate-400 dark:text-slate-500 hover:text-slate-600 dark:hover:text-slate-300 hover:underline shrink-0 truncate max-w-[150px]">
                      ({{ getDomain(story.url) }})
                  </a>
              </div>

              <div class="flex flex-wrap gap-y-1 gap-x-2 mt-1.5 items-center">
                <!-- User -->
                <div class="flex items-center gap-1 text-xs text-slate-500 dark:text-slate-400">
                    <div class="w-4 h-4 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center text-[9px] font-bold text-slate-500 dark:text-slate-400 border border-slate-200 dark:border-slate-700">
                        {{ story.submitter_user.charAt(0).toUpperCase() }}
                    </div>
                    <span>{{ story.submitter_user }}</span>
                </div>

                <span class="text-slate-400 dark:text-slate-500 text-[10px]">&bull;</span>

                <!-- Tags -->
                <div class="flex flex-wrap gap-1">
                    <span v-for="tag in story.tags" :key="tag"
                          class="px-1.5 py-0.5 bg-red-50 dark:bg-red-900/20 text-red-700 dark:text-red-300 text-[10px] font-bold uppercase rounded border border-red-200 dark:border-red-800 hover:bg-red-100 dark:hover:bg-red-900/30 transition-colors">
                        {{ tag }}
                    </span>
                </div>
              </div>
            </div>
          </td>

          <!-- Score -->
          <td class="px-3 py-4 whitespace-nowrap text-right">
             <div class="inline-flex items-center gap-1 text-sm font-bold text-orange-600 dark:text-orange-400 bg-orange-50 dark:bg-orange-900/30 px-2 py-0.5 rounded-md border border-orange-100 dark:border-orange-800">
                 {{ story.score }}
             </div>
          </td>

          <!-- Comments -->
          <td class="px-3 py-4 whitespace-nowrap text-right">
            <div class="text-sm font-medium text-slate-600 dark:text-slate-300 flex items-center justify-end gap-1">
                <svg class="w-3.5 h-3.5 text-slate-400 dark:text-slate-500" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" /></svg>
                {{ story.comment_count }}
            </div>
          </td>

          <!-- Date -->
          <td class="pl-3 pr-6 py-4 whitespace-nowrap text-right text-xs text-slate-400 dark:text-slate-500">
              {{ formatDate(story.created_at) }}
          </td>
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
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffHours = diffMs / (1000 * 60 * 60);

  if (diffHours < 24) {
      if (diffHours < 1) {
          const diffMin = Math.floor(diffMs / (1000 * 60));
          return `${diffMin}m`;
      }
      return `${Math.floor(diffHours)}h`;
  }
  return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(date);
}

function getDomain(url: string): string {
    try {
        return new URL(url).hostname.replace('www.', '');
    } catch {
        return '';
    }
}
</script>
