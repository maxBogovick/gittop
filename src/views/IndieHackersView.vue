<template>
  <div class="relative">
    <ViewHeader title="Indie Hackers" subtitle="Learn from the developers behind profitable products">
      <template #actions>
        <div class="flex items-center gap-4 w-full">
          <SearchBar
              v-model="store.searchQuery"
              placeholder="Search posts..."
              class="w-full sm:w-80"
          />
          <div class="text-xs font-semibold text-slate-400">
            {{ store.posts.length }} posts
          </div>
        </div>
      </template>
    </ViewHeader>

    <div v-if="store.error" class="bg-red-50 border border-red-100 p-4 rounded-xl shadow-sm mb-6 flex items-center justify-between">
      <div class="flex items-center gap-3 text-red-700">
        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
        <p class="text-sm font-semibold">{{ store.error }}</p>
      </div>
      <button @click="store.loadPosts()" class="text-sm font-bold text-red-600 hover:underline">Retry</button>
    </div>

    <LoadingSkeleton v-if="store.isLoading && !store.allPosts.length" />
    <div v-else class="mt-6">
      <IndieHackersTable :posts="store.posts" @select="handleSelect" />
    </div>

    <Transition name="slide-over">
      <IndieHackersDrawer
        v-if="selectedPost"
        :post="selectedPost"
        @close="selectedPost = null"
      />
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useIndieHackersStore } from '../stores/indiehackers';
import ViewHeader from '../components/ViewHeader.vue';
import IndieHackersTable from '../components/IndieHackersTable.vue';
import IndieHackersDrawer from '../components/IndieHackersDrawer.vue';
import LoadingSkeleton from '../components/LoadingSkeleton.vue';
import SearchBar from '../components/SearchBar.vue';
import { type IndieHackersPost } from '../services/tauriApi';

const store = useIndieHackersStore();
const selectedPost = ref<IndieHackersPost | null>(null);

function handleSelect(post: IndieHackersPost) {
  selectedPost.value = post;
}

onMounted(() => { if (!store.allPosts.length) store.loadPosts(); });
</script>