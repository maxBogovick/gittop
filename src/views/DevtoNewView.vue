<template>
  <div class="relative">
    <ViewHeader 
      title="Dev.to" 
      subtitle="Fresh content hot off the press from developer authors"
      has-toggle
      top-path="/devto/top"
      new-path="/devto/new"
    >
      <template #actions>
        <div class="flex flex-col sm:flex-row items-center gap-4 w-full">
          <SearchBar 
              v-model="store.tag" 
              placeholder="Filter by tag..." 
              @update:model-value="reload" 
              class="w-full sm:w-80" 
          />
          <div class="flex items-center gap-3 bg-white p-1.5 rounded-xl border border-slate-200 shadow-sm">
            <select 
                v-model="store.state" 
                @change="reload"
                class="block w-full sm:w-48 pl-3 pr-10 py-2 text-sm border-transparent focus:ring-0 sm:text-sm rounded-lg"
            >
                <option value="fresh">Fresh</option>
                <option value="rising">Rising</option>
                <option value="all">All</option>
            </select>
          </div>
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

    <LoadingSkeleton v-if="store.isLoading && store.page === 1" />
    
    <div v-else-if="store.articles.length === 0 && !store.isLoading" class="text-center py-10 text-gray-500">
      No articles found.
    </div>
    
    <div v-else>
      <DevtoTable 
        :articles="store.articles" 
        @select="handleSelect" 
      />
      <LoadMoreButton 
        :loading="store.isLoading" 
        :has-more="store.hasMore" 
        @click="loadMore" 
      />
    </div>

    <Transition name="slide-over">
        <DevtoDrawer 
            v-if="selectedArticle" 
            :article="selectedArticle" 
            @close="selectedArticle = null" 
        />
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useDevtoStore } from '../stores/devto';
import ViewHeader from '../components/ViewHeader.vue';
import DevtoTable from '../components/DevtoTable.vue';
import DevtoDrawer from '../components/DevtoDrawer.vue';
import LoadingSkeleton from '../components/LoadingSkeleton.vue';
import LoadMoreButton from '../components/LoadMoreButton.vue';
import SearchBar from '../components/SearchBar.vue';
import { type DevtoArticle } from '../services/tauriApi';

const store = useDevtoStore();
const selectedArticle = ref<DevtoArticle | null>(null);

function reload() {
  // For "New" view, we assume top is null
  store.top = null;
  // Default state to 'fresh' if not set
  if (!store.state) store.state = 'fresh';
  store.loadArticles(false);
}

function loadMore() {
  store.loadArticles(true);
}

function handleSelect(article: DevtoArticle) {
  selectedArticle.value = article;
}

onMounted(() => {
  store.state = 'fresh';
  store.top = null;
  if (store.articles.length === 0) {
      reload();
  }
});
</script>
