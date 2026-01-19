<template>
  <div class="relative">
    <ViewHeader 
      title="Dev.to" 
      subtitle="Most popular articles from the developer community"
      has-toggle
      top-path="/devto/top"
      new-path="/devto/new"
    >
      <template #actions>
        <div class="flex flex-col sm:flex-row items-center gap-4 w-full">
          <SearchBar 
              v-model="store.tag" 
              placeholder="Filter by tag (e.g. rust, webdev)..." 
              @update:model-value="reload" 
              class="w-full sm:w-80" 
          />
          <div class="flex items-center gap-3 bg-white p-1.5 rounded-xl border border-slate-200 shadow-sm">
            <select 
                v-model="store.top" 
                @change="reload"
                class="block w-full sm:w-48 pl-3 pr-10 py-2 text-sm border-transparent focus:ring-0 sm:text-sm rounded-lg"
            >
                <option :value="1">Top today</option>
                <option :value="7">Top this week</option>
                <option :value="30">Top this month</option>
                <option :value="365">Top this year</option>
                <option :value="null">All time</option>
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
  // For "Top" view, we ensure state is either default or irrelevant, but usually 'rising' or just use 'top' param
  // If 'top' is set, 'state' param might be ignored by API or conflict. 
  // We'll set state to null to rely on 'top' param.
  store.state = null; 
  if (store.top === null) store.top = 7; // Default to week if not set
  store.loadArticles(false);
}

function loadMore() {
  store.loadArticles(true);
}

function handleSelect(article: DevtoArticle) {
  selectedArticle.value = article;
}

onMounted(() => {
  // Initialize with some default if empty
  if (store.articles.length === 0) {
      store.top = 7;
      reload();
  }
});
</script>
