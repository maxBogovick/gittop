<template>
  <div class="animate-pulse space-y-4">
    <div v-for="i in count" :key="i" class="flex items-center space-x-4 bg-white/50 dark:bg-slate-800/50 p-4 rounded-xl border border-slate-200/50 dark:border-slate-700/50">
      <div class="rounded-full bg-slate-200 dark:bg-slate-700 h-10 w-10"></div>
      <div class="flex-1 space-y-2">
        <div class="h-4 bg-slate-200 dark:bg-slate-700 rounded w-1/4"></div>
        <div class="h-3 bg-slate-200 dark:bg-slate-700 rounded w-1/2"></div>
      </div>
      <div class="h-4 bg-slate-200 dark:bg-slate-700 rounded w-16"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  count?: number
}>(), {
  count: 5
});
</script>
