<template>
  <div class="overflow-hidden bg-white/70 dark:bg-slate-900/70 backdrop-blur-md shadow-lg border border-slate-200/50 dark:border-slate-700/50 rounded-2xl">
    <table class="min-w-full divide-y divide-slate-200/50 dark:divide-slate-700/50">
      <thead class="bg-slate-50/50 dark:bg-slate-800/50">
        <tr>
          <th scope="col" class="px-6 py-4 text-left text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Item</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Price</th>
          <th scope="col" class="px-6 py-4 text-left text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Shop</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Favorites</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Created</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-200/50 dark:divide-slate-700/50">
        <tr
          v-for="listing in listings"
          :key="listing.listing_id"
          class="hover:bg-white/50 dark:hover:bg-slate-800/50 transition-all duration-150 cursor-pointer group"
          @click="$emit('select', listing)"
        >
          <td class="px-6 py-4">
            <div class="flex items-center">
              <div class="flex-shrink-0 h-12 w-12">
                <img v-if="listing.images && listing.images.length > 0 && listing.images[0].url_170x135" class="h-12 w-12 rounded-lg object-cover ring-2 ring-white dark:ring-slate-800 shadow-sm" :src="listing.images[0].url_170x135" alt="" />
                <div v-else class="h-12 w-12 rounded-lg bg-slate-100 dark:bg-slate-800 flex items-center justify-center text-slate-400 dark:text-slate-500 font-bold text-xs">
                  No Img
                </div>
              </div>
              <div class="ml-4">
                <div class="text-sm font-bold text-slate-900 dark:text-white group-hover:text-orange-600 dark:group-hover:text-orange-400 transition-colors line-clamp-2" :title="listing.title">
                  {{ listing.title }}
                </div>
                <div class="text-xs text-slate-400 dark:text-slate-500 mt-0.5" v-if="listing.tags && listing.tags.length">
                  {{ listing.tags.slice(0, 3).join(', ') }}
                </div>
              </div>
            </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-900 dark:text-white text-right font-bold">
             {{ formatPrice(listing.price) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-600 dark:text-slate-300">
            {{ listing.shop?.shop_name || 'Unknown' }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-600 dark:text-slate-300 text-right font-mono">
            {{ formatNumber(listing.num_favorers || 0) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500 dark:text-slate-400 text-right">
            {{ formatDate(listing.creation_tsz) }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { type EtsyListing, type EtsyPrice } from '../services/tauriApi';

defineProps<{
  listings: EtsyListing[];
}>();

defineEmits<{
  (e: 'select', listing: EtsyListing): void;
}>();

function formatNumber(num: number): string {
  return new Intl.NumberFormat('en-US', { notation: "compact", compactDisplay: "short" }).format(num);
}

function formatDate(ts?: number): string {
  if (!ts) return '';
  const date = new Date(ts * 1000);
  return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric', year: 'numeric' }).format(date);
}

function formatPrice(price?: EtsyPrice): string {
    if (!price) return 'N/A';
    try {
        const amount = price.amount / price.divisor;
        return new Intl.NumberFormat('en-US', { style: 'currency', currency: price.currency_code }).format(amount);
    } catch {
        return `${price.amount / price.divisor} ${price.currency_code}`;
    }
}
</script>
