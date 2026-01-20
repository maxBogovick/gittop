<template>
  <div class="relative">
    <ViewHeader
      title="Hacker News"
      subtitle="The pulse of the tech world and Silicon Valley startup culture"
    >
      <template #actions>
        <div class="flex flex-col lg:flex-row items-center gap-4 w-full">
          <SearchBar
              v-model="store.searchQuery"
              placeholder="Search stories..."
              @update:model-value="reload"
              class="w-full lg:w-72"
          />

          <div v-if="!store.searchQuery" class="flex bg-white dark:bg-slate-800 p-1 rounded-xl border border-slate-200 dark:border-slate-700 shadow-sm">
            <button
                v-for="type in storyTypes"
                :key="type.value"
                @click="store.storyType = type.value; reload()"
                :class="[
                    'px-3 py-2 text-xs font-bold uppercase tracking-wider rounded-lg transition-all',
                    store.storyType === type.value ? 'bg-orange-500 text-white shadow-md' : 'text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:bg-slate-50 dark:hover:bg-slate-700'
                ]"
            >
                {{ type.label }}
            </button>
          </div>

          <div v-if="store.searchQuery" class="flex items-center gap-2">
            <select
                v-model="store.searchSort"
                @change="reload()"
                class="px-3 py-2 text-sm bg-white dark:bg-slate-800 text-slate-900 dark:text-white border border-slate-200 dark:border-slate-700 rounded-lg shadow-sm focus:ring-2 focus:ring-orange-500 focus:border-orange-500 cursor-pointer"
            >
              <option value="popularity" class="bg-white dark:bg-slate-800">By Popularity</option>
              <option value="date" class="bg-white dark:bg-slate-800">By Date</option>
            </select>

            <select
                v-model="store.searchTimeRange"
                @change="reload()"
                class="px-3 py-2 text-sm bg-white dark:bg-slate-800 text-slate-900 dark:text-white border border-slate-200 dark:border-slate-700 rounded-lg shadow-sm focus:ring-2 focus:ring-orange-500 focus:border-orange-500 cursor-pointer"
            >
              <option value="" class="bg-white dark:bg-slate-800">All Time</option>
              <option value="day" class="bg-white dark:bg-slate-800">Last 24h</option>
              <option value="week" class="bg-white dark:bg-slate-800">Last Week</option>
              <option value="month" class="bg-white dark:bg-slate-800">Last Month</option>
              <option value="year" class="bg-white dark:bg-slate-800">Last Year</option>
            </select>
          </div>
        </div>
      </template>
    </ViewHeader>
    
    <div v-if="store.error" class="bg-red-50 border border-red-100 p-4 rounded-xl shadow-sm mb-6 flex items-center gap-3">
        <div class="flex-1 text-sm font-semibold text-red-800">{{ store.error }}</div>
        <button @click="reload" class="text-sm font-bold text-red-600 hover:text-red-700 underline transition-colors">Retry</button>
    </div>

    <LoadingSkeleton v-if="store.isLoading && store.page === 1" />
    
    <div v-else-if="store.stories.length === 0 && !store.isLoading" class="flex flex-col items-center justify-center py-20 text-slate-400">
      <p class="font-medium">No stories found.</p>
    </div>
    
    <div v-else class="space-y-6">
      <HackerNewsTable 
        :stories="store.stories" 
        @select="handleSelect" 
      />
      <LoadMoreButton 
        :loading="store.isLoading" 
        :has-more="store.hasMore" 
        @click="loadMore" 
      />
    </div>

    <Transition name="slide-over">
        <HackerNewsDrawer 
            v-if="selectedStory" 
            :story="selectedStory" 
            @close="selectedStory = null" 
        />
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useHackerNewsStore } from '../stores/hackernews';
import ViewHeader from '../components/ViewHeader.vue';
import HackerNewsTable from '../components/HackerNewsTable.vue';
import HackerNewsDrawer from '../components/HackerNewsDrawer.vue';
import LoadingSkeleton from '../components/LoadingSkeleton.vue';
import LoadMoreButton from '../components/LoadMoreButton.vue';
import SearchBar from '../components/SearchBar.vue';
import { type HackerNewsStory } from '../services/tauriApi';

const store = useHackerNewsStore();
const selectedStory = ref<HackerNewsStory | null>(null);

const storyTypes = [
  { value: 'top', label: 'Top' },
  { value: 'new', label: 'New' },
  { value: 'best', label: 'Best' },
  { value: 'show', label: 'Show' },
  { value: 'ask', label: 'Ask' },
  { value: 'job', label: 'Jobs' },
];

function reload() {
  store.loadStories(false);
}

function loadMore() {
  store.loadStories(true);
}

function handleSelect(story: HackerNewsStory) {
  selectedStory.value = story;
}

onMounted(() => {
  if (store.stories.length === 0) {
      reload();
  }
});
</script>