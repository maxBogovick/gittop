<template>
  <div class="fixed inset-0 z-50 flex justify-end" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">
    <div class="drawer-backdrop fixed inset-0 bg-slate-900/30 backdrop-blur-sm transition-opacity" aria-hidden="true" @click="$emit('close')"></div>

    <div class="drawer-panel w-screen max-w-2xl transform transition-transform bg-white shadow-2xl flex flex-col h-full relative z-10">
      <div class="flex-shrink-0 px-6 py-6 border-b border-slate-100 bg-white/80 backdrop-blur-md sticky top-0 z-20 flex items-start justify-between">
        <div class="min-w-0 pr-4">
           <div class="flex items-center gap-3">
              <div class="h-12 w-12 rounded-xl bg-gradient-to-br from-orange-500 to-red-600 flex items-center justify-center shadow-lg">
                <svg class="h-6 w-6 text-white" viewBox="0 0 512 512" fill="currentColor">
                  <path d="M239.1 6.3l-208 78c-18.7 7-31.1 25-31.1 45v225.1c0 18.2 10.3 34.8 26.5 42.9l208 104c13.5 6.8 29.4 6.8 42.9 0l208-104c16.3-8.1 26.5-24.8 26.5-42.9V129.3c0-20-12.4-37.9-31.1-44.9l-208-78c-9-3.4-18.8-3.4-27.8 0zM256 68.4l192 72v1.1l-192 78-192-78v-1.1l192-72zm32 356V275.5l160-65v133.9l-160 80z"/>
                </svg>
              </div>
              <div>
                <h2 class="text-xl font-bold text-slate-900 leading-tight" id="slide-over-title">{{ crate.name }}</h2>
                <p class="text-sm text-slate-500 mt-1">v{{ crate.max_version }}</p>
              </div>
           </div>
        </div>
        <button type="button" class="rounded-full p-2 text-slate-400 hover:text-slate-600 hover:bg-slate-100 transition-colors focus:outline-none flex-shrink-0" @click="$emit('close')">
          <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <div class="flex-1 overflow-y-auto custom-scrollbar">
         <div class="p-8">
             <div v-if="crate.description" class="bg-slate-50 border-l-4 border-orange-500 p-6 rounded-r-xl mb-8">
                 <p class="text-slate-700 leading-relaxed">{{ crate.description }}</p>
             </div>

             <div class="grid grid-cols-2 gap-4 mb-8">
                <div class="bg-slate-50 p-4 rounded-xl text-center">
                    <p class="text-2xl font-bold text-slate-900">{{ formatDownloads(crate.downloads) }}</p>
                    <p class="text-xs font-semibold text-slate-400 uppercase tracking-widest mt-1">Downloads</p>
                </div>
                <div class="bg-slate-50 p-4 rounded-xl text-center">
                    <p class="text-2xl font-bold text-slate-900">{{ crate.max_version }}</p>
                    <p class="text-xs font-semibold text-slate-400 uppercase tracking-widest mt-1">Latest Version</p>
                </div>
             </div>

             <div class="mb-8">
                <h3 class="text-sm font-semibold text-slate-400 uppercase tracking-widest mb-4">Installation</h3>
                <div class="bg-slate-900 text-slate-100 p-4 rounded-xl font-mono text-sm overflow-x-auto">
                    <code>cargo add {{ crate.name }}</code>
                </div>
             </div>

             <div class="mb-8">
                <h3 class="text-sm font-semibold text-slate-400 uppercase tracking-widest mb-4">Last Updated</h3>
                <p class="text-slate-700">{{ formatDate(crate.updated_at) }}</p>
             </div>

             <div class="pt-8 border-t border-slate-100 flex flex-col gap-3">
                <a :href="`https://crates.io/crates/${crate.name}`" target="_blank" class="bg-orange-600 text-white px-6 py-3 rounded-xl font-bold hover:bg-orange-700 transition-all shadow-lg hover:shadow-orange-500/20 active:scale-95 text-center">
                    View on Crates.io
                </a>
                <a v-if="crate.repository" :href="crate.repository" target="_blank" class="bg-slate-800 text-white px-6 py-3 rounded-xl font-bold hover:bg-slate-900 transition-all shadow-lg active:scale-95 text-center">
                    View Repository
                </a>
                <a v-if="crate.homepage && crate.homepage !== crate.repository" :href="crate.homepage" target="_blank" class="border border-slate-200 text-slate-700 px-6 py-3 rounded-xl font-bold hover:bg-slate-50 transition-all active:scale-95 text-center">
                    Visit Homepage
                </a>
                <a :href="`https://docs.rs/${crate.name}`" target="_blank" class="border border-slate-200 text-slate-700 px-6 py-3 rounded-xl font-bold hover:bg-slate-50 transition-all active:scale-95 text-center">
                    View Documentation
                </a>
             </div>
         </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type Crate } from '../services/tauriApi';

defineProps<{
  crate: Crate;
}>();

defineEmits<{
  (e: 'close'): void;
}>();

function formatDate(dateStr: string): string {
  if (!dateStr) return '';
  const date = new Date(dateStr);
  return new Intl.DateTimeFormat('en-US', {
    month: 'long',
    day: 'numeric',
    year: 'numeric'
  }).format(date);
}

function formatDownloads(num: number): string {
  if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
  if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
  return num.toString();
}
</script>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background-color: #cbd5e1;
  border-radius: 20px;
}
</style>
