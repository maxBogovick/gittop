<template>
  <nav class="flex items-center gap-2 text-sm">
    <span class="font-bold text-slate-900 dark:text-white">{{ currentSection }}</span>
    <template v-if="currentPage">
      <svg class="w-4 h-4 text-slate-400 dark:text-slate-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
      <span class="text-slate-600 dark:text-slate-300 font-medium">{{ currentPage }}</span>
    </template>
  </nav>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();

// Route to breadcrumb mapping
const breadcrumbMap: Record<string, { section: string; page?: string }> = {
  '/top': { section: 'GitHub', page: 'Top Repositories' },
  '/new': { section: 'GitHub', page: 'New Repositories' },
  '/reddit/top': { section: 'Reddit', page: 'Top Posts' },
  '/reddit/new': { section: 'Reddit', page: 'New Posts' },
  '/devto/top': { section: 'Dev.to', page: 'Top Articles' },
  '/devto/new': { section: 'Dev.to', page: 'New Articles' },
  '/etsy/top': { section: 'Etsy', page: 'Top Products' },
  '/etsy/new': { section: 'Etsy', page: 'New Products' },
  '/stackoverflow': { section: 'StackOverflow' },
  '/hackernews': { section: 'Hacker News' },
  '/medium': { section: 'Medium' },
  '/hashnode': { section: 'Hashnode' },
  '/producthunt': { section: 'Product Hunt' },
  '/lobsters': { section: 'Lobsters' },
  '/crates': { section: 'Crates.io' },
  '/indiehackers': { section: 'Indie Hackers' },
};

const currentSection = computed(() => {
  const crumb = breadcrumbMap[route.path];
  return crumb?.section || 'Dashboard';
});

const currentPage = computed(() => {
  const crumb = breadcrumbMap[route.path];
  return crumb?.page || null;
});
</script>
