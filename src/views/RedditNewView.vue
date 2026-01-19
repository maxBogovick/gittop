<template>
  <div class="relative">
    <ViewHeader 
      title="Reddit" 
      subtitle="Fresh content streaming in from around the web"
      has-toggle
      top-path="/reddit/top"
      new-path="/reddit/new"
    >
      <template #actions>
        <div class="flex flex-col sm:flex-row items-center gap-4 w-full">
          <SearchBar v-model="store.keyword" @update:model-value="reload" class="w-full sm:w-80" />
          <div class="flex items-center gap-3 bg-white p-1.5 rounded-xl border border-slate-200 shadow-sm">
            <SubredditSelector v-model="store.subreddit" @update:model-value="reload" />
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

    <LoadingSkeleton v-if="store.isLoading && !store.posts.length" />
    
    <div v-else-if="store.posts.length === 0 && !store.isLoading" class="text-center py-10 text-gray-500">
      No posts found.
    </div>
    
    <div v-else>
      <RedditTable 
        :posts="store.sortedPosts" 
        :current-sort="store.sortBy"
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
        <RedditDrawer 
            v-if="selectedPost" 
            :post="selectedPost" 
            @close="selectedPost = null" 
        />
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useRedditStore } from '../stores/reddit';
import ViewHeader from '../components/ViewHeader.vue';
import SubredditSelector from '../components/SubredditSelector.vue';
import SearchBar from '../components/SearchBar.vue';
import RedditTable from '../components/RedditTable.vue';
import LoadingSkeleton from '../components/LoadingSkeleton.vue';
import RedditDrawer from '../components/RedditDrawer.vue';
import LoadMoreButton from '../components/LoadMoreButton.vue';
import type { RedditPost } from '../services/tauriApi';

const store = useRedditStore();
const selectedPost = ref<RedditPost | null>(null);

function reload() {
  store.loadNewPosts(false);
}

function loadMore() {
  store.loadNewPosts(true);
}

function handleSelect(post: RedditPost) {
  selectedPost.value = post;
}

function handleSort(column: string) {
  store.sort(column);
}

onMounted(() => {
  if (store.posts.length === 0) {
    store.sortBy = 'date'; // Default for New
    reload();
  }
});
</script>