<template>
  <div class="relative">
    <ViewHeader title="Lobste.rs" subtitle="Computing-focused community with serious technical discussion">
      <template #actions>
          <div class="flex bg-white p-1 rounded-xl border border-slate-200 shadow-sm w-fit">
              <button v-for="type in ['hottest', 'newest']" :key="type" @click="store.storyType = type; store.loadStories()" :class="['px-6 py-2 text-xs font-black uppercase tracking-wider rounded-lg transition-all', store.storyType === type ? 'bg-slate-900 text-white' : 'text-slate-500 hover:text-slate-700']">{{ type }}</button>
          </div>
      </template>
    </ViewHeader>
    
    <div v-if="store.error" class="bg-red-50 border border-red-100 p-4 rounded-xl shadow-sm mb-6 flex items-center justify-between">
      <div class="flex items-center gap-3 text-red-700">
        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
        <p class="text-sm font-semibold">{{ store.error }}</p>
      </div>
      <button @click="store.loadStories()" class="text-sm font-bold text-red-600 hover:underline">Retry</button>
    </div>

    <LoadingSkeleton v-if="store.isLoading && !store.stories.length" />
    <div v-else class="mt-6">
      <LobstersTable :stories="store.stories" @select="handleSelect" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useLobstersStore } from '../stores/lobsters';
import ViewHeader from '../components/ViewHeader.vue';
import LobstersTable from '../components/LobstersTable.vue';
import LoadingSkeleton from '../components/LoadingSkeleton.vue';
import { openUrl } from '@tauri-apps/plugin-opener';

const store = useLobstersStore();
const handleSelect = async (story: any) => await openUrl(story.url);
onMounted(() => { if (!store.stories.length) store.loadStories(); });
</script>