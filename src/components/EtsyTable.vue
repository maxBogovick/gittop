<template>
  <div class="overflow-x-auto bg-white shadow rounded-lg">
    <table class="min-w-full divide-y divide-gray-200">
      <thead class="bg-gray-50">
        <tr>
          <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Item</th>
          <th scope="col" class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">Price</th>
          <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Shop</th>
          <th scope="col" class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">Favorites</th>
          <th scope="col" class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">Created</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-gray-200">
        <tr 
          v-for="listing in listings" 
          :key="listing.listing_id" 
          class="hover:bg-gray-50 transition-colors cursor-pointer group"
          @click="$emit('select', listing)"
        >
          <td class="px-6 py-4">
            <div class="flex items-center">
              <div class="flex-shrink-0 h-12 w-12">
                <img v-if="listing.images && listing.images.length > 0 && listing.images[0].url_170x135" class="h-12 w-12 rounded object-cover border border-gray-200" :src="listing.images[0].url_170x135" alt="" />
                <div v-else class="h-12 w-12 rounded bg-gray-200 flex items-center justify-center text-gray-500 font-bold text-xs">
                  No Img
                </div>
              </div>
              <div class="ml-4">
                <div class="text-sm font-medium text-orange-600 group-hover:underline line-clamp-2" :title="listing.title">
                  {{ listing.title }}
                </div>
                <div class="text-xs text-gray-400 mt-0.5" v-if="listing.tags && listing.tags.length">
                  {{ listing.tags.slice(0, 3).join(', ') }}
                </div>
              </div>
            </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-800 text-right font-medium">
             {{ formatPrice(listing.price) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-600">
            {{ listing.shop?.shop_name || 'Unknown' }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-600 text-right font-mono">
            {{ formatNumber(listing.num_favorers || 0) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 text-right">
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