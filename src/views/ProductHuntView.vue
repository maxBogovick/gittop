<template>
  <div class="relative">
    <ViewHeader title="Product Hunt" subtitle="The best new products in tech, every day" />
    
    <div v-if="store.error" class="bg-red-50 border border-red-100 p-4 rounded-xl shadow-sm mb-6 flex items-center justify-between">
      <div class="flex items-center gap-3 text-red-700">
        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
        <p class="text-sm font-semibold">{{ store.error }}</p>
      </div>
      <button @click="store.loadPosts()" class="text-sm font-bold text-red-600 hover:underline">Retry</button>
    </div>

    <LoadingSkeleton v-if="store.isLoading && !store.posts.length" />
    <div v-else class="space-y-6">
      <ProductHuntTable :posts="store.posts" @select="handleSelect" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useProductHuntStore } from '../stores/producthunt';
import ViewHeader from '../components/ViewHeader.vue';
import ProductHuntTable from '../components/ProductHuntTable.vue';
import LoadingSkeleton from '../components/LoadingSkeleton.vue';
import { openUrl } from '@tauri-apps/plugin-opener';

const store = useProductHuntStore();
const handleSelect = async (post: any) => await openUrl(post.url);

onMounted(() => { if (!store.posts.length) store.loadPosts(); });
</script>