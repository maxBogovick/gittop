<template>
  <div class="relative">
    <ViewHeader 
      title="GitHub" 
      subtitle="Trending projects and active community repositories"
      has-toggle
      top-path="/top"
      new-path="/new"
    >
      <template #actions>
        <div class="flex flex-col sm:flex-row items-center gap-4 w-full">
          <SearchBar v-model="store.keyword" @update:model-value="reload" class="w-full sm:w-80" />
          <div class="flex items-center gap-3 bg-white p-1.5 rounded-xl border border-slate-200 shadow-sm">
            <LanguageSelector v-model="store.language" @update:model-value="reload" />
            <TimeRangeSelector v-model="store.timeRange" @update:model-value="reload" />
            <MetricSelector v-model="store.metric" @update:model-value="reload" />
          </div>
          <div class="sm:ml-auto">
            <ExportButton :repositories="store.repositories" :disabled="store.isLoading || store.repositories.length === 0" />
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

    <!-- Show skeleton only on initial load, not append -->
    <LoadingSkeleton v-if="store.isLoading && store.page === 1" />
    
    <div v-else-if="store.repositories.length === 0 && !store.isLoading" class="text-center py-10 text-gray-500">
      No repositories found matching your criteria.
    </div>
    
    <div v-else>
      <RepoTable 
        :repositories="store.repositories" 
        :current-metric="store.metric"
        @select="handleSelect" 
        @sort="handleSort"
      />
      <LoadMoreButton 
        :loading="store.isLoading" 
        :has-more="store.hasMore" 
        @click="loadMore" 
      />
    </div>

    <Transition name="slide-over">
        <RepoDrawer 
            v-if="selectedRepo" 
            :repo="selectedRepo" 
            @close="selectedRepo = null" 
        />
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useRepositoryStore } from '../stores/repositories';
import ViewHeader from '../components/ViewHeader.vue';
import TimeRangeSelector from '../components/TimeRangeSelector.vue';
import MetricSelector from '../components/MetricSelector.vue';
import LanguageSelector from '../components/LanguageSelector.vue';
import SearchBar from '../components/SearchBar.vue';
import RepoTable from '../components/RepoTable.vue';
import LoadingSkeleton from '../components/LoadingSkeleton.vue';
import RepoDrawer from '../components/RepoDrawer.vue';
import LoadMoreButton from '../components/LoadMoreButton.vue';
import ExportButton from '../components/ExportButton.vue';
import { type Repository, Metric } from '../services/tauriApi';

const store = useRepositoryStore();
const selectedRepo = ref<Repository | null>(null);

function reload() {
  store.loadTopRepositories(false);
}

function loadMore() {
  store.loadTopRepositories(true);
}

function handleSelect(repo: Repository) {
  selectedRepo.value = repo;
}

function handleSort(metric: string) {
  store.metric = metric as Metric;
  reload();
}

onMounted(() => {
  if (store.repositories.length === 0) {
    reload();
  }
});
</script>