<template>
  <div class="overflow-hidden bg-white/70 dark:bg-slate-900/70 backdrop-blur-md shadow-lg border border-slate-200/50 dark:border-slate-700/50 rounded-2xl">
    <table class="min-w-full divide-y divide-slate-200/50 dark:divide-slate-700/50">
      <thead class="bg-slate-50/50 dark:bg-slate-800/50">
        <tr>
          <th scope="col" class="px-6 py-4 text-left text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Crate</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Downloads</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Version</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Updated</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-200/50 dark:divide-slate-700/50">
        <tr v-for="item in crates" :key="item.id" class="hover:bg-white/50 dark:hover:bg-slate-800/50 transition-all duration-150 cursor-pointer group" @click="$emit('select', item)">
          <td class="px-6 py-4">
            <div class="min-w-0">
              <div class="text-sm font-black text-slate-900 dark:text-white group-hover:text-orange-600 dark:group-hover:text-orange-400 transition-colors">{{ item.name }}</div>
              <div class="text-xs text-slate-500 dark:text-slate-400 mt-1 line-clamp-1">{{ item.description }}</div>
            </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-700 dark:text-slate-300 text-right font-mono font-medium">{{ formatNumber(item.downloads) }}</td>
          <td class="px-6 py-4 whitespace-nowrap text-right">
            <span class="px-2 py-1 bg-orange-50 dark:bg-orange-900/20 text-orange-700 dark:text-orange-300 text-xs font-bold rounded-md border border-orange-200 dark:border-orange-800">v{{ item.max_version }}</span>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500 dark:text-slate-400 text-right">{{ formatDate(item.updated_at) }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { type Crate } from '../services/tauriApi';
defineProps<{ crates: Crate[] }>();
defineEmits<{ (e: 'select', item: Crate): void }>();

function formatNumber(num: number): string {
  return new Intl.NumberFormat('en-US', { notation: "compact", compactDisplay: "short" }).format(num);
}

function formatDate(dateStr: string): string {
  const date = new Date(dateStr);
  return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(date);
}
</script>
