<template>
  <div class="relative inline-block text-left">
    <button 
      @click="exportData" 
      :disabled="loading || disabled"
      class="inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
    >
      <svg class="-ml-1 mr-2 h-5 w-5 text-gray-500" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd" />
      </svg>
      {{ loading ? 'Exporting...' : 'Export CSV' }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { exportRepositories, type Repository } from '../services/tauriApi';

const props = defineProps<{
  repositories: Repository[];
  disabled?: boolean;
}>();

const loading = ref(false);

async function exportData() {
  if (props.repositories.length === 0) return;
  
  loading.value = true;
  try {
    const path = await exportRepositories(props.repositories, 'csv');
    alert(`Exported successfully to: ${path}`);
  } catch (e: any) {
    alert(`Export failed: ${e}`);
  } finally {
    loading.value = false;
  }
}
</script>
