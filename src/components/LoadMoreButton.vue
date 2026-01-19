<template>
  <div class="flex justify-center py-6">
    <button 
      @click="$emit('click')" 
      :disabled="loading || !hasMore"
      class="px-6 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
    >
      <span v-if="loading">Loading...</span>
      <span v-else-if="!hasMore">No more repositories</span>
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
