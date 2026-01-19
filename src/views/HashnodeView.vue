<template>
  <div class="relative">
    <ViewHeader
      title="Hashnode"
      subtitle="Developer-first blogging platform for the modern web"
    >
      <template #actions>
        <div class="flex flex-col sm:flex-row items-center gap-4 w-full">
          <SearchBar
              v-model="store.tag"
              placeholder="Filter by tag (e.g. rust, javascript, web-development)..."
              @update:model-value="reload"
              class="w-full sm:w-80"
          />
          <div class="flex bg-white p-1 rounded-xl border border-slate-200 shadow-sm">
            <button
                v-for="type in feedTypes"
                :key="type.value"
                @click="store.feedType = type.value; reload()"
                :class="[
                    'px-4 py-2 text-xs font-bold uppercase tracking-wider rounded-lg transition-all',
                    store.feedType === type.value ? 'bg-blue-600 text-white shadow-md' : 'text-slate-500 hover:text-slate-700 hover:bg-slate-50'
                ]"
            >
                {{ type.label }}
            </button>
          </div>
        </div>
      </template>
    </ViewHeader>
    
    <div v-if="store.error" class="bg-red-50 border border-red-100 p-4 rounded-xl shadow-sm mb-6 flex items-center justify-between">
      <div class="flex items-center gap-3 text-red-700">
        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
        <p class="text-sm font-semibold">{{ store.error }}</p>
      </div>
      <button @click="reload()" class="text-sm font-bold text-red-600 hover:underline">Retry</button>
    </div>

    <LoadingSkeleton v-if="store.isLoading && !store.posts.length" />
    
    <div v-else-if="store.posts.length === 0 && !store.isLoading" class="flex flex-col items-center justify-center py-20 text-slate-400">
      <p class="font-medium">No posts found.</p>
    </div>
    
    <div v-else class="space-y-6">
      <HashnodeTable 
        :posts="store.posts" 
        @select="handleSelect" 
      />
      <LoadMoreButton 
        v-if="store.hasMore"
        :loading="store.isLoading" 
        :has-more="store.hasMore" 
        @click="loadMore" 
      />
    </div>

    <Transition name="slide-over">
        <HashnodeDrawer 
            v-if="selectedPost" 
            :post="selectedPost" 
            @close="selectedPost = null" 
        />
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useHashnodeStore } from '../stores/hashnode';
import ViewHeader from '../components/ViewHeader.vue';
import HashnodeTable from '../components/HashnodeTable.vue';
import HashnodeDrawer from '../components/HashnodeDrawer.vue';
import LoadingSkeleton from '../components/LoadingSkeleton.vue';
import LoadMoreButton from '../components/LoadMoreButton.vue';
import SearchBar from '../components/SearchBar.vue';
import { type HashnodePost } from '../services/tauriApi';

const store = useHashnodeStore();
const selectedPost = ref<HashnodePost | null>(null);

const feedTypes = [
  { value: 'FEATURED', label: 'Featured' },
  { value: 'RECENT', label: 'Recent' },
  { value: 'RELEVANT', label: 'Relevant' },
];

function reload() {
  store.loadPosts(false);
}

function loadMore() {
  store.loadPosts(true);
}

function handleSelect(post: HashnodePost) {
  selectedPost.value = post;
}

onMounted(() => {
  if (store.posts.length === 0) {
      reload();
  }
});
</script>