<template>
  <div class="relative">
    <ViewHeader 
      title="Stack Overflow" 
      subtitle="Top programming questions and answers from the global developer community"
    >
      <template #actions>
        <div class="flex flex-col sm:flex-row items-center gap-4 w-full">
          <SearchBar 
              v-model="store.tag" 
              placeholder="Filter by tag (e.g. rust, typescript)..." 
              @update:model-value="reload" 
              class="w-full sm:w-80" 
          />
          
          <div class="flex items-center gap-2 bg-white dark:bg-slate-800 p-1.5 rounded-xl border border-slate-200 dark:border-slate-700 shadow-sm">
              <span class="text-xs font-bold text-slate-400 dark:text-slate-500 uppercase tracking-wider ml-2">Sort by:</span>
              <select
                  v-model="store.sort"
                  @change="reload"
                  class="block pl-3 pr-10 py-2 text-sm bg-transparent text-slate-900 dark:text-white border-transparent focus:ring-0 rounded-lg transition-all cursor-pointer"
              >
                  <option value="votes" class="bg-white dark:bg-slate-800">Top Rated</option>
                  <option value="activity" class="bg-white dark:bg-slate-800">Recent Activity</option>
                  <option value="creation" class="bg-white dark:bg-slate-800">Newest</option>
                  <option value="hot" class="bg-white dark:bg-slate-800">Hot</option>
              </select>
          </div>
        </div>
      </template>
    </ViewHeader>
    
    <div v-if="store.error" class="bg-red-50 border border-red-100 p-4 rounded-xl shadow-sm mb-6 flex items-center gap-3">
        <div class="p-2 bg-red-100 rounded-full text-red-600">
            <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
        </div>
        <div class="flex-1">
          <p class="text-sm font-semibold text-red-800">{{ store.error }}</p>
        </div>
        <button @click="reload" class="text-sm font-bold text-red-600 hover:text-red-700 underline px-2 py-1 transition-colors">
            Retry
        </button>
    </div>

    <LoadingSkeleton v-if="store.isLoading && store.page === 1" />
    
    <div v-else-if="store.questions.length === 0 && !store.isLoading" class="flex flex-col items-center justify-center py-20 text-slate-400">
      <svg class="h-12 w-12 mb-3 opacity-20" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
      <p class="font-medium">No questions found matching your criteria.</p>
    </div>
    
    <div v-else class="space-y-6">
      <StackOverflowTable 
        :questions="store.questions" 
        @select="handleSelect" 
      />
      <LoadMoreButton 
        :loading="store.isLoading" 
        :has-more="store.hasMore" 
        @click="loadMore" 
      />
    </div>

    <Transition name="slide-over">
        <StackOverflowDrawer 
            v-if="selectedQuestion" 
            :question="selectedQuestion" 
            @close="selectedQuestion = null" 
        />
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useStackOverflowStore } from '../stores/stackoverflow';
import ViewHeader from '../components/ViewHeader.vue';
import StackOverflowTable from '../components/StackOverflowTable.vue';
import StackOverflowDrawer from '../components/StackOverflowDrawer.vue';
import LoadingSkeleton from '../components/LoadingSkeleton.vue';
import LoadMoreButton from '../components/LoadMoreButton.vue';
import SearchBar from '../components/SearchBar.vue';
import { type StackOverflowQuestion } from '../services/tauriApi';

const store = useStackOverflowStore();
const selectedQuestion = ref<StackOverflowQuestion | null>(null);

function reload() {
  store.loadQuestions(false);
}

function loadMore() {
  store.loadQuestions(true);
}

function handleSelect(question: StackOverflowQuestion) {
  selectedQuestion.value = question;
}

onMounted(() => {
  if (store.questions.length === 0) {
      reload();
  }
});
</script>