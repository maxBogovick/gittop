<template>
  <div class="overflow-hidden bg-white/70 dark:bg-slate-900/70 backdrop-blur-md shadow-lg border border-slate-200/50 dark:border-slate-700/50 rounded-2xl">
    <table class="min-w-full divide-y divide-slate-200/50 dark:divide-slate-700/50">
      <thead class="bg-slate-50/50 dark:bg-slate-800/50">
        <tr>
          <th scope="col" class="px-6 py-4 text-left text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Repository</th>
          <th scope="col" class="px-6 py-4 text-left text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Language</th>
          <th
            scope="col"
            class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider cursor-pointer hover:text-slate-700 dark:hover:text-slate-200 transition-colors group"
            @click="$emit('sort', 'Stars')"
          >
            <div class="flex items-center justify-end gap-1">
              <span>Stars</span>
              <svg v-if="currentMetric === 'Stars'" class="h-4 w-4 text-blue-600 dark:text-blue-400" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M14.707 12.707a1 1 0 01-1.414 0L10 9.414l-3.293 3.293a1 1 0 01-1.414-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 010 1.414z" clip-rule="evenodd" transform="rotate(180 10 10)" />
              </svg>
              <svg v-else class="h-4 w-4 text-slate-400 opacity-0 group-hover:opacity-100" viewBox="0 0 20 20" fill="currentColor">
                 <path d="M5 10a1 1 0 011-1h8a1 1 0 110 2H6a1 1 0 01-1-1z" />
              </svg>
            </div>
          </th>
          <th
            scope="col"
            class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider cursor-pointer hover:text-slate-700 dark:hover:text-slate-200 transition-colors group"
            @click="$emit('sort', 'Forks')"
          >
             <div class="flex items-center justify-end gap-1">
              <span>Forks</span>
              <svg v-if="currentMetric === 'Forks'" class="h-4 w-4 text-blue-600 dark:text-blue-400" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M14.707 12.707a1 1 0 01-1.414 0L10 9.414l-3.293 3.293a1 1 0 01-1.414-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 010 1.414z" clip-rule="evenodd" transform="rotate(180 10 10)" />
              </svg>
            </div>
          </th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Issues</th>
          <th
            scope="col"
            class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider cursor-pointer hover:text-slate-700 dark:hover:text-slate-200 transition-colors group"
            @click="$emit('sort', 'Updated')"
          >
            <div class="flex items-center justify-end gap-1">
              <span>Activity</span>
              <svg v-if="currentMetric === 'Updated'" class="h-4 w-4 text-blue-600 dark:text-blue-400" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M14.707 12.707a1 1 0 01-1.414 0L10 9.414l-3.293 3.293a1 1 0 01-1.414-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 010 1.414z" clip-rule="evenodd" transform="rotate(180 10 10)" />
              </svg>
            </div>
          </th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-200/50 dark:divide-slate-700/50">
        <tr
          v-for="repo in repositories"
          :key="repo.owner + '/' + repo.name"
          class="hover:bg-white/50 dark:hover:bg-slate-800/50 transition-all duration-150 cursor-pointer group"
          @click="$emit('select', repo)"
        >
          <td class="px-6 py-4">
            <div class="flex items-start">
              <div class="flex-shrink-0 h-10 w-10">
                <img v-if="repo.owner_avatar_url" class="h-10 w-10 rounded-full object-cover ring-2 ring-white dark:ring-slate-800 shadow-sm" :src="repo.owner_avatar_url" alt="" />
                <div v-else class="h-10 w-10 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center text-slate-500 dark:text-slate-400 font-bold ring-2 ring-white dark:ring-slate-800 shadow-sm">
                  {{ repo.owner.charAt(0).toUpperCase() }}
                </div>
              </div>
              <div class="ml-4">
                <div class="text-sm font-bold text-slate-900 dark:text-white group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors">
                  {{ repo.owner }} / {{ repo.name }}
                </div>
                <div class="text-sm text-slate-500 dark:text-slate-400 line-clamp-1 max-w-xs mt-0.5" :title="repo.description || ''">
                  {{ repo.description || 'No description' }}
                </div>
                <div v-if="repo.license" class="flex items-center gap-1 text-xs text-slate-400 dark:text-slate-500 mt-1">
                  <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 6l3 1m0 0l-3 9a5.002 5.002 0 006.001 0M6 7l3 9M6 7l6-2m6 2l3-1m-3 1l-3 9a5.002 5.002 0 006.001 0M18 7l3 9m-3-9l-6-2m0-2v2m0 16V5m0 16H9m3 0h3" />
                  </svg>
                  {{ repo.license }}
                </div>
              </div>
            </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap">
             <span v-if="repo.primary_language" class="inline-flex items-center px-2.5 py-0.5 rounded-md text-xs font-medium bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-300 ring-1 ring-inset ring-blue-700/10 dark:ring-blue-400/20">
               {{ repo.primary_language }}
             </span>
             <span v-else class="text-slate-400 text-xs italic">N/A</span>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-700 dark:text-slate-300 text-right font-medium">
            <div class="flex items-center justify-end gap-1">
               {{ formatNumber(repo.stars_total) }}
               <svg class="h-4 w-4 text-amber-400" viewBox="0 0 20 20" fill="currentColor">
                 <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
               </svg>
            </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-700 dark:text-slate-300 text-right font-mono">
            {{ formatNumber(repo.forks) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-700 dark:text-slate-300 text-right font-mono">
            {{ formatNumber(repo.open_issues) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500 dark:text-slate-400 text-right">
            {{ formatDate(repo.last_activity_at || repo.updated_at) }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { type Repository, Metric } from '../services/tauriApi';

defineProps<{
  repositories: Repository[];
  currentMetric?: Metric;
}>();

defineEmits<{
  (e: 'select', repo: Repository): void;
  (e: 'sort', metric: string): void;
}>();

function formatNumber(num: number): string {
  return new Intl.NumberFormat('en-US', { notation: "compact", compactDisplay: "short" }).format(num);
}

function formatDate(dateStr: string): string {
  if (!dateStr) return '';
  const date = new Date(dateStr);
  const now = new Date();
  const diffTime = Math.abs(now.getTime() - date.getTime());
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));

  if (diffDays < 1) return 'Today';
  if (diffDays === 1) return 'Yesterday';
  if (diffDays < 7) return `${diffDays} days ago`;

  return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(date);
}
</script>
