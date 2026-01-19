<template>
  <div class="fixed inset-0 z-50 flex justify-end" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">
    <!-- Backdrop -->
    <div class="drawer-backdrop fixed inset-0 bg-slate-900/30 backdrop-blur-sm transition-opacity" aria-hidden="true" @click="$emit('close')"></div>

    <!-- Panel -->
    <div class="drawer-panel w-screen max-w-2xl transform transition-transform bg-white shadow-2xl flex flex-col h-full relative z-10">
      
      <!-- Header -->
      <div class="flex-shrink-0 px-6 py-4 border-b border-slate-100 bg-white/80 backdrop-blur-md sticky top-0 z-20 flex items-start justify-between">
        <div class="flex items-start gap-4 min-w-0 pr-4">
           <div class="flex-shrink-0">
              <img v-if="listing.images && listing.images.length > 0 && listing.images[0].url_75x75" :src="listing.images[0].url_75x75" class="h-16 w-16 rounded-lg object-cover border border-slate-200 shadow-sm" />
           </div>
           <div class="flex-1 min-w-0">
              <h2 class="text-lg font-bold text-slate-900 leading-snug line-clamp-2" id="slide-over-title">{{ listing.title }}</h2>
              <div class="flex flex-wrap items-center gap-x-3 gap-y-1 text-sm text-slate-500 mt-1">
                 <span class="font-medium text-slate-700">{{ listing.shop?.shop_name }}</span>
                 <span>•</span>
                 <span class="font-semibold text-green-600">{{ formatPrice(listing.price) }}</span>
                 <span>•</span>
                 <span class="flex items-center gap-1 text-red-500">
                    <svg class="h-3 w-3 fill-current" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M3.172 5.172a4 4 0 015.656 0L10 6.343l1.172-1.171a4 4 0 115.656 5.656L10 17.657l-6.828-6.829a4 4 0 010-5.656z" clip-rule="evenodd" /></svg>
                    {{ listing.num_favorers }}
                 </span>
              </div>
           </div>
        </div>
        <button type="button" class="rounded-full p-2 text-slate-400 hover:text-slate-600 hover:bg-slate-100 transition-colors focus:outline-none flex-shrink-0" @click="$emit('close')">
          <span class="sr-only">Close panel</span>
          <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Actions -->
      <div class="px-6 py-3 bg-slate-50 border-b border-slate-100 flex gap-2">
         <a :href="listing.url" target="_blank" class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-lg shadow-sm text-white bg-orange-600 hover:bg-orange-700 focus:outline-none transition-colors">
            View on Etsy
         </a>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto px-6 py-6 custom-scrollbar">
         <!-- Image Gallery Preview -->
         <div v-if="listing.images && listing.images.length > 1" class="mb-8">
             <h3 class="text-sm font-semibold text-slate-900 mb-3">Gallery</h3>
             <div class="grid grid-cols-4 gap-3">
                 <div 
                     v-for="(img, idx) in listing.images.slice(0, 8)" 
                     :key="idx" 
                     class="aspect-square rounded-lg overflow-hidden border border-slate-200 cursor-zoom-in hover:ring-2 hover:ring-orange-500 transition-all"
                     @click="openImage(img.url_fullxfull || img.url_570xN)"
                 >
                    <img :src="img.url_170x135" class="w-full h-full object-cover" loading="lazy" />
                 </div>
             </div>
         </div>

         <div class="prose prose-slate prose-sm max-w-none text-slate-600 leading-relaxed whitespace-pre-line">
             {{ listing.description || 'No description available.' }}
         </div>
         
         <div v-if="listing.tags && listing.tags.length" class="mt-8 pt-6 border-t border-slate-100">
             <h4 class="text-xs font-semibold text-slate-400 uppercase tracking-wider mb-3">Tags</h4>
             <div class="flex flex-wrap gap-2">
                 <span v-for="tag in listing.tags" :key="tag" class="px-2.5 py-1 bg-slate-100 text-slate-600 text-xs font-medium rounded-full">
                     {{ tag }}
                 </span>
             </div>
         </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type EtsyListing, type EtsyPrice } from '../services/tauriApi';
import { openUrl } from '@tauri-apps/plugin-opener';

defineProps<{
  listing: EtsyListing
}>();

defineEmits<{
  (e: 'close'): void
}>();

function formatPrice(price?: EtsyPrice): string {
    if (!price) return 'N/A';
    try {
        const amount = price.amount / price.divisor;
        return new Intl.NumberFormat('en-US', { style: 'currency', currency: price.currency_code }).format(amount);
    } catch {
        return `${price.amount / price.divisor} ${price.currency_code}`;
    }
}

async function openImage(url?: string) {
    if (url) {
        await openUrl(url);
    }
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
