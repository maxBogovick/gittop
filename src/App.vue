<template>
  <div class="min-h-screen relative bg-gradient-to-br from-slate-50 via-blue-50 to-indigo-100 dark:from-slate-950 dark:via-slate-900 dark:to-indigo-950 font-sans text-[var(--color-text-primary)] overflow-hidden">
    <!-- Animated Background Elements -->
    <div class="fixed inset-0 overflow-hidden pointer-events-none z-0">
      <div class="absolute top-1/4 -left-48 w-96 h-96 bg-blue-400/20 dark:bg-blue-600/10 rounded-full blur-3xl animate-pulse"></div>
      <div class="absolute bottom-1/4 -right-48 w-96 h-96 bg-indigo-400/20 dark:bg-indigo-600/10 rounded-full blur-3xl animate-pulse" style="animation-delay: 1s;"></div>
      <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-purple-400/10 dark:bg-purple-600/5 rounded-full blur-3xl animate-pulse" style="animation-delay: 2s;"></div>
    </div>

    <!-- Desktop Sidebar -->
    <AppSidebar
      :collapsed="themeStore.sidebarCollapsed"
      @toggle="themeStore.toggleSidebar"
    />

    <!-- Mobile Sidebar Drawer -->
    <Transition name="mobile-sidebar">
      <MobileSidebarDrawer
        v-if="isMobileMenuOpen"
        @close="closeMobileMenu"
      />
    </Transition>

    <!-- Global Search Modal -->
    <SearchModal
      :is-open="isSearchOpen"
      @close="isSearchOpen = false"
    />

    <!-- Main Content Area -->
    <div
      :class="[
        'relative z-10 min-h-screen flex flex-col transition-all duration-300',
        themeStore.sidebarCollapsed
          ? 'lg:pl-[var(--sidebar-collapsed-width)]'
          : 'lg:pl-[var(--sidebar-width)]'
      ]"
    >
      <!-- Header -->
      <AppHeader
        :is-refreshing="isRefreshing"
        @open-mobile-menu="openMobileMenu"
        @open-search="handleSearch"
        @refresh="handleRefresh"
        @open-settings="handleSettings"
      />

      <!-- Content -->
      <main class="flex-grow px-4 sm:px-6 lg:px-8 py-6 animate-fade-in-up">
        <div class="max-w-7xl mx-auto">
          <router-view v-slot="{ Component }">
            <transition name="page" mode="out-in">
              <component :is="Component" />
            </transition>
          </router-view>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue';
import { useRoute } from 'vue-router';
import { useThemeStore } from './stores/theme';
import { refreshCache } from './services/tauriApi';
import AppSidebar from './components/layout/AppSidebar.vue';
import AppHeader from './components/layout/AppHeader.vue';
import MobileSidebarDrawer from './components/layout/MobileSidebarDrawer.vue';
import SearchModal from './components/SearchModal.vue';

const themeStore = useThemeStore();
const route = useRoute();

const isRefreshing = ref(false);
const isMobileMenuOpen = ref(false);
const isSearchOpen = ref(false);

// Initialize theme on mount
onMounted(() => {
  themeStore.applyTheme();
  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});

function handleKeydown(e: KeyboardEvent) {
  if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
    e.preventDefault();
    isSearchOpen.value = !isSearchOpen.value;
  }
}

// Close mobile menu on route change
watch(() => route.path, () => {
  isMobileMenuOpen.value = false;
  // Also close search if navigating (e.g. from search modal itself, but modal handles it)
});

function openMobileMenu() {
  isMobileMenuOpen.value = true;
}

function closeMobileMenu() {
  isMobileMenuOpen.value = false;
}

async function handleRefresh() {
  isRefreshing.value = true;
  try {
    await refreshCache();
  } finally {
    setTimeout(() => isRefreshing.value = false, 1000);
  }
}

function handleSearch() {
  isSearchOpen.value = true;
}

function handleSettings() {
  // TODO: Implement settings modal
  console.log('Open settings...');
}
</script>

<style>
.no-scrollbar::-webkit-scrollbar {
  display: none;
}
.no-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
