<template>
  <div class="relative">
    <ViewHeader 
      title="Etsy" 
      subtitle="Trending handmade items and unique discoveries"
      has-toggle
      top-path="/etsy/top"
      new-path="/etsy/new"
    >
      <template #actions>
        <div class="flex flex-col sm:flex-row items-center gap-4 w-full">
          <SearchBar v-model="store.keyword" @update:model-value="reload" class="w-full sm:w-80" placeholder="Search Etsy..." />
        </div>
      </template>
    </ViewHeader>
    
    <div v-if="store.error" class="bg-red-50 border border-red-100 p-4 rounded-xl shadow-sm mb-6 flex items-center justify-between">
      <div class="flex items-center gap-3 text-red-700">
        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
        <p class="text-sm font-semibold">{{ store.error }}</p>
      </div>
      <button @click="reload" class="text-sm font-bold text-red-600 hover:underline">Retry</button>
    </div>

    <LoadingSkeleton v-if="store.isLoading && store.offset === 0" />
    
    <div v-else-if="store.listings.length === 0 && !store.isLoading" class="text-center py-10 text-gray-500">
      No listings found.
    </div>
    
    <div v-else>
      <EtsyTable 
        :listings="store.listings" 
        @select="handleSelect" 
      />
      <LoadMoreButton 
        :loading="store.isLoading" 
        :has-more="store.hasMore" 
        @click="loadMore" 
      />
    </div>

    <Transition name="slide-over">
        <EtsyDrawer 
            v-if="selectedListing" 
            :listing="selectedListing" 
            @close="selectedListing = null" 
        />
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useEtsyStore } from '../stores/etsy';
import ViewHeader from '../components/ViewHeader.vue';
import SearchBar from '../components/SearchBar.vue';
import EtsyTable from '../components/EtsyTable.vue';
import LoadingSkeleton from '../components/LoadingSkeleton.vue';
import EtsyDrawer from '../components/EtsyDrawer.vue';
import LoadMoreButton from '../components/LoadMoreButton.vue';
import { type EtsyListing } from '../services/tauriApi';

const store = useEtsyStore();
const selectedListing = ref<EtsyListing | null>(null);

function reload() {
  store.loadTopListings(false);
}

function loadMore() {
  store.loadTopListings(true);
}

function handleSelect(listing: EtsyListing) {
  selectedListing.value = listing;
}

onMounted(() => {
  reload();
});
</script>