<template>
  <div class="overflow-hidden bg-white shadow-sm border border-slate-200 rounded-xl">
    <table class="min-w-full divide-y divide-slate-200">
      <thead class="bg-slate-50/80">
        <tr>
          <th scope="col" class="px-6 py-4 text-left text-xs font-semibold text-slate-500 uppercase tracking-wider">Crate</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-semibold text-slate-500 uppercase tracking-wider">Downloads</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-semibold text-slate-500 uppercase tracking-wider">Version</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-semibold text-slate-500 uppercase tracking-wider">Updated</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-100 bg-white">
        <tr v-for="item in crates" :key="item.id" class="hover:bg-slate-50/80 transition-all duration-150 cursor-pointer group" @click="$emit('select', item)">
          <td class="px-6 py-4">
            <div class="min-w-0">
              <div class="text-sm font-black text-slate-900 group-hover:text-orange-600 transition-colors">{{ item.name }}</div>
              <div class="text-xs text-slate-500 mt-1 line-clamp-1">{{ item.description }}</div>
            </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-600 text-right font-mono font-medium">{{ formatNumber(item.downloads) }}</td>
          <td class="px-6 py-4 whitespace-nowrap text-right">
            <span class="px-2 py-1 bg-slate-100 text-slate-600 text-xs font-bold rounded-md border border-slate-200">v{{ item.max_version }}</span>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500 text-right">{{ formatDate(item.updated_at) }}</td>
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