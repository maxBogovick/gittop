<template>
  <header class="sticky top-0 z-30 h-16 flex items-center justify-between px-4 lg:px-6 bg-white/70 dark:bg-slate-900/70 backdrop-blur-xl border-b border-slate-200/50 dark:border-slate-700/50 transition-all duration-300">
    <!-- Left side: Mobile menu + Breadcrumbs -->
    <div class="flex items-center gap-4">
      <!-- Mobile menu button -->
      <button
        @click="$emit('openMobileMenu')"
        class="lg:hidden p-2 rounded-lg text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-slate-900 dark:hover:text-white transition-colors"
        aria-label="Open menu"
      >
        <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
        </svg>
      </button>

      <!-- Breadcrumbs -->
      <Breadcrumbs />
    </div>

    <!-- Right side: Actions -->
    <div class="flex items-center gap-2">
      <!-- Search shortcut (Cmd+K) -->
      <button
        @click="$emit('openSearch')"
        class="hidden sm:flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm text-slate-500 bg-slate-100/50 dark:bg-slate-800/50 border border-slate-200/50 dark:border-slate-700/50 hover:border-blue-400 dark:hover:border-blue-500 transition-colors"
      >
        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
        <span>Search</span>
        <kbd class="hidden md:inline-flex items-center gap-0.5 px-1.5 py-0.5 bg-white/50 dark:bg-slate-900/50 rounded text-xs font-medium border border-slate-200 dark:border-slate-700">
          <span class="text-[10px]">⌘</span>K
        </kbd>
      </button>

      <!-- Divider -->
      <div class="hidden sm:block w-px h-6 bg-slate-200 dark:bg-slate-700 mx-1"></div>

      <!-- Refresh button -->
      <button
        @click="$emit('refresh')"
        :disabled="isRefreshing"
        class="p-2 rounded-lg text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-blue-600 dark:hover:text-blue-400 transition-colors disabled:opacity-50"
        title="Refresh cache"
      >
        <svg
          :class="['w-5 h-5', isRefreshing ? 'animate-spin' : '']"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.001 0 01-15.357-2m15.357 2H15" />
        </svg>
      </button>

      <!-- Theme toggle -->
      <ThemeToggle />

      <!-- Settings button -->
      <button
        @click="$emit('openSettings')"
        class="p-2 rounded-lg text-[var(--color-text-secondary)] hover:bg-[var(--color-bg-hover)] hover:text-[var(--color-text-primary)] transition-colors"
        title="Settings"
      >
        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
import Breadcrumbs from '../ui/Breadcrumbs.vue';
import ThemeToggle from '../ui/ThemeToggle.vue';

defineProps<{
  isRefreshing: boolean;
}>();

defineEmits<{
  (e: 'openMobileMenu'): void;
  (e: 'openSearch'): void;
  (e: 'refresh'): void;
  (e: 'openSettings'): void;
}>();
</script>
