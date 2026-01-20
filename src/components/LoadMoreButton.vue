<template>
  <div class="flex justify-center py-6">
    <button
      @click="$emit('click')"
      :disabled="loading || !hasMore"
      class="px-6 py-2.5 border border-slate-200 dark:border-slate-700 shadow-sm text-sm font-medium rounded-xl text-slate-600 dark:text-slate-300 bg-white dark:bg-slate-800 hover:bg-slate-50 dark:hover:bg-slate-700 hover:text-slate-900 dark:hover:text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
    >
      <span v-if="loading">Loading...</span>
      <span v-else-if="!hasMore">No more items</span>
      <span v-else>Load More</span>
    </button>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  loading: boolean;
  hasMore: boolean;
}>();

defineEmits<{
  (e: 'click'): void
}>();
</script>
