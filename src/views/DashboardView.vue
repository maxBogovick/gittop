<template>
  <div class="h-full flex flex-col items-center justify-center p-6 py-20">
      <!-- Hero Section -->
      <div class="w-full max-w-4xl text-center space-y-10 animate-fade-in">
        <!-- Header with enhanced styling -->
        <div class="space-y-6">
          <div class="inline-flex items-center gap-2 px-4 py-2 bg-white/80 dark:bg-slate-800/80 backdrop-blur-sm rounded-full border border-slate-200 dark:border-slate-700 shadow-lg mb-6">
            <div class="w-2 h-2 bg-emerald-500 rounded-full animate-pulse"></div>
            <span class="text-sm font-semibold bg-gradient-to-r from-blue-600 to-indigo-600 dark:from-blue-400 dark:to-indigo-400 bg-clip-text text-transparent">
              Search Across 12+ Developer Platforms
            </span>
          </div>

          <h1 class="text-5xl sm:text-7xl font-black tracking-tight">
            <span class="bg-gradient-to-r from-slate-900 via-slate-700 to-slate-900 dark:from-white dark:via-slate-200 dark:to-white bg-clip-text text-transparent">
              GitTop
            </span>
            <span class="block mt-2 bg-gradient-to-r from-blue-600 via-indigo-600 to-purple-600 dark:from-blue-400 dark:via-indigo-400 dark:to-purple-400 bg-clip-text text-transparent animate-gradient">
              Search
            </span>
          </h1>

          <p class="text-xl text-slate-600 dark:text-slate-300 max-w-2xl mx-auto leading-relaxed font-medium">
            One unified search engine for all your favorite developer communities, forums, and resources. Find what you need, faster.
          </p>
        </div>

        <!-- Search Box Container with enhanced effects -->
        <div class="w-full relative group">
          <!-- Animated gradient border -->
          <div class="absolute -inset-1 bg-gradient-to-r from-blue-600 via-indigo-600 to-purple-600 rounded-2xl opacity-30 blur-lg group-hover:opacity-60 group-focus-within:opacity-60 transition-all duration-500 animate-gradient"></div>

          <!-- Main search container -->
          <div class="relative bg-white/90 dark:bg-slate-800/90 backdrop-blur-xl rounded-2xl shadow-2xl border border-slate-200 dark:border-slate-700 overflow-hidden">
            <div class="flex items-center p-3">
              <!-- Search icon -->
              <div class="pl-4 pr-3">
                <svg class="w-7 h-7 text-slate-400 dark:text-slate-500 group-focus-within:text-blue-600 dark:group-focus-within:text-blue-400 transition-colors" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
              </div>

              <!-- Input field -->
              <input
                  v-model="query"
                  @keydown.enter="handleSearch"
                  @focus="inputFocused = true"
                  @blur="inputFocused = false"
                  type="text"
                  class="flex-1 bg-transparent border-none focus:ring-0 focus:outline-none text-xl px-2 py-4 text-slate-900 dark:text-white placeholder-slate-400 dark:placeholder-slate-500"
                  placeholder="Search for 'rust async', 'react hooks', 'vue composition'..."
                  autofocus
              />

              <!-- Action buttons -->
              <div class="flex items-center gap-3 pr-3">
                <kbd v-if="!inputFocused" class="hidden sm:inline-flex px-3 py-1.5 bg-slate-100 dark:bg-slate-700 rounded-lg text-sm font-mono text-slate-600 dark:text-slate-400 border border-slate-300 dark:border-slate-600 shadow-sm">
                  Enter
                </kbd>
                <button
                    @click="handleSearch"
                    :disabled="selectedSources.length === 0"
                    :class="[
                    'p-3 rounded-xl transition-all duration-300 transform hover:scale-105 active:scale-95',
                    selectedSources.length === 0 
                      ? 'bg-slate-100 dark:bg-slate-700 text-slate-400 dark:text-slate-500 cursor-not-allowed'
                      : 'bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-700 hover:to-indigo-700 text-white shadow-lg shadow-blue-500/50 dark:shadow-blue-500/30'
                  ]"
                    title="Search"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-6 h-6">
                    <path fill-rule="evenodd" d="M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9z" clip-rule="evenodd" />
                  </svg>
                </button>
              </div>
            </div>

            <!-- Progress bar when searching -->
            <div v-if="inputFocused" class="h-1 w-full bg-gradient-to-r from-blue-600 via-indigo-600 to-purple-600 animate-shimmer"></div>
          </div>

          <!-- Error message with better styling -->
          <Transition
              enter-active-class="transition-all duration-300 ease-out"
              enter-from-class="opacity-0 translate-y-2"
              enter-to-class="opacity-100 translate-y-0"
              leave-active-class="transition-all duration-200 ease-in"
              leave-from-class="opacity-100 translate-y-0"
              leave-to-class="opacity-0 translate-y-2"
          >
            <div v-if="showError" class="absolute -bottom-12 left-0 right-0 flex justify-center">
              <div class="inline-flex items-center gap-2 px-4 py-2 bg-red-50 dark:bg-red-900/30 text-red-600 dark:text-red-400 text-sm font-semibold rounded-xl border border-red-200 dark:border-red-800 shadow-lg">
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
                </svg>
                Please select at least one source
              </div>
            </div>
          </Transition>
        </div>

        <!-- Source Selection with enhanced design -->
        <div class="space-y-6 pt-4">
          <!-- Stats and toggle -->
          <div class="flex items-center justify-center gap-6">
            <div class="flex items-center gap-2 text-sm text-slate-600 dark:text-slate-400">
              <div class="flex items-center gap-1.5 px-3 py-1.5 bg-white/80 dark:bg-slate-800/80 backdrop-blur-sm rounded-full border border-slate-200 dark:border-slate-700">
                <svg class="w-4 h-4 text-emerald-500" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                </svg>
                <span class="font-semibold">{{ selectedSources.length }}/{{ availableSources.length }}</span>
                <span>selected</span>
              </div>
            </div>

            <button
                @click="toggleAll"
                class="text-sm font-bold text-blue-600 dark:text-blue-400 hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors uppercase tracking-wider flex items-center gap-2 group"
            >
              <span>{{ isAllSelected ? 'Deselect All' : 'Select All' }}</span>
              <svg class="w-4 h-4 transition-transform group-hover:scale-110" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
              </svg>
            </button>
          </div>

          <!-- Source chips with enhanced design -->
          <div class="flex flex-wrap justify-center gap-2.5 max-w-4xl mx-auto">
            <button
                v-for="(source, index) in availableSources"
                :key="source.id"
                @click="toggleSource(source.id)"
                :style="{ animationDelay: `${index * 30}ms` }"
                :class="[
                'group relative px-4 py-2.5 rounded-xl text-sm font-semibold transition-all duration-300 border-2 flex items-center gap-2 animate-slide-up',
                'hover:shadow-lg hover:-translate-y-0.5 active:scale-95',
                selectedSources.includes(source.id)
                  ? 'bg-gradient-to-r from-blue-600 to-indigo-600 text-white border-transparent shadow-lg shadow-blue-500/30 dark:shadow-blue-500/20'
                  : 'bg-white/80 dark:bg-slate-800/80 backdrop-blur-sm text-slate-700 dark:text-slate-300 border-slate-200 dark:border-slate-700 hover:border-blue-400 dark:hover:border-blue-500'
              ]"
            >
              <component :is="source.icon" class="w-4 h-4 transition-transform group-hover:scale-110" />
              <span>{{ source.name }}</span>

              <!-- Selection indicator -->
              <Transition
                  enter-active-class="transition-all duration-200"
                  enter-from-class="scale-0 opacity-0"
                  enter-to-class="scale-100 opacity-100"
                  leave-active-class="transition-all duration-200"
                  leave-from-class="scale-100 opacity-100"
                  leave-to-class="scale-0 opacity-0"
              >
                <svg v-if="selectedSources.includes(source.id)" class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
              </Transition>
            </button>
          </div>
        </div>
      </div>

      <!-- Popular Searches with enhanced design -->
      <div class="mt-20 text-center space-y-6 animate-fade-in" style="animation-delay: 200ms;">
        <div class="flex items-center justify-center gap-3">
          <div class="h-px w-12 bg-gradient-to-r from-transparent to-slate-300 dark:to-slate-700"></div>
          <div class="flex items-center gap-2 px-4 py-2 bg-white/80 dark:bg-slate-800/80 backdrop-blur-sm rounded-full border border-slate-200 dark:border-slate-700">
            <svg class="w-4 h-4 text-orange-500" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M12.395 2.553a1 1 0 00-1.45-.385c-.345.23-.614.558-.822.88-.214.33-.403.713-.57 1.116-.334.804-.614 1.768-.84 2.734a31.365 31.365 0 00-.613 3.58 2.64 2.64 0 01-.945-1.067c-.328-.68-.398-1.534-.398-2.654A1 1 0 005.05 6.05 6.981 6.981 0 003 11a7 7 0 1011.95-4.95c-.592-.591-.98-.985-1.348-1.467-.363-.476-.724-1.063-1.207-2.03zM12.12 15.12A3 3 0 017 13s.879.5 2.5.5c0-1 .5-4 1.25-4.5.5 1 .786 1.293 1.371 1.879A2.99 2.99 0 0113 13a2.99 2.99 0 01-.879 2.121z" clip-rule="evenodd" />
            </svg>
            <span class="text-sm font-bold text-slate-700 dark:text-slate-300 uppercase tracking-wider">
              Trending Searches
            </span>
          </div>
          <div class="h-px w-12 bg-gradient-to-l from-transparent to-slate-300 dark:to-slate-700"></div>
        </div>

        <div class="flex flex-wrap justify-center items-center gap-3">
          <button
              v-for="(term, index) in popularSearches"
              :key="term"
              @click="searchFor(term)"
              :style="{ animationDelay: `${index * 50}ms` }"
              class="group px-5 py-2 bg-white/60 dark:bg-slate-800/60 backdrop-blur-sm hover:bg-white dark:hover:bg-slate-800 rounded-full border border-slate-200 dark:border-slate-700 hover:border-blue-400 dark:hover:border-blue-500 transition-all duration-300 hover:shadow-lg hover:-translate-y-0.5 animate-slide-up"
          >
            <span class="text-sm font-medium text-slate-700 dark:text-slate-300 group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors">
              {{ term }}
            </span>
          </button>
        </div>
      </div>

      <!-- Footer stats -->
      <div class="mt-16 flex flex-wrap justify-center gap-8 animate-fade-in" style="animation-delay: 400ms;">
        <div class="text-center">
          <div class="text-3xl font-bold bg-gradient-to-r from-blue-600 to-indigo-600 dark:from-blue-400 dark:to-indigo-400 bg-clip-text text-transparent">
            12+
          </div>
          <div class="text-sm text-slate-600 dark:text-slate-400 font-medium mt-1">Platforms</div>
        </div>
        <div class="text-center">
          <div class="text-3xl font-bold bg-gradient-to-r from-indigo-600 to-purple-600 dark:from-indigo-400 dark:to-purple-400 bg-clip-text text-transparent">
            ∞
          </div>
          <div class="text-sm text-slate-600 dark:text-slate-400 font-medium mt-1">Results</div>
        </div>
        <div class="text-center">
          <div class="text-3xl font-bold bg-gradient-to-r from-purple-600 to-pink-600 dark:from-purple-400 dark:to-pink-400 bg-clip-text text-transparent">
            1
          </div>
          <div class="text-sm text-slate-600 dark:text-slate-400 font-medium mt-1">Unified Search</div>
        </div>
      </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRouter } from 'vue-router';
import {
  CodeBracketIcon,
  ChatBubbleLeftRightIcon,
  QuestionMarkCircleIcon,
  NewspaperIcon,
  HashtagIcon,
  CubeIcon,
  ShoppingBagIcon,
  FireIcon,
  MegaphoneIcon
} from '@heroicons/vue/24/outline';

const router = useRouter();
const query = ref('');
const showError = ref(false);
const inputFocused = ref(false);

const availableSources = [
  { id: 'GitHub', name: 'GitHub', icon: CodeBracketIcon },
  { id: 'Crates.io', name: 'Crates.io', icon: CubeIcon },
  { id: 'StackOverflow', name: 'Stack Overflow', icon: QuestionMarkCircleIcon },
  { id: 'Reddit', name: 'Reddit', icon: ChatBubbleLeftRightIcon },
  { id: 'HackerNews', name: 'Hacker News', icon: NewspaperIcon },
  { id: 'Lobsters', name: 'Lobsters', icon: FireIcon },
  { id: 'Dev.to', name: 'Dev.to', icon: HashtagIcon },
  { id: 'Medium', name: 'Medium', icon: NewspaperIcon },
  { id: 'Hashnode', name: 'Hashnode', icon: HashtagIcon },
  { id: 'ProductHunt', name: 'Product Hunt', icon: MegaphoneIcon },
  { id: 'IndieHackers', name: 'Indie Hackers', icon: ChatBubbleLeftRightIcon },
  { id: 'Etsy', name: 'Etsy', icon: ShoppingBagIcon },
];

const popularSearches = ['Rust', 'Vue 3', 'Tauri', 'React', 'TypeScript', 'Next.js'];

const selectedSources = ref<string[]>(availableSources.map(s => s.id));

const isAllSelected = computed(() => selectedSources.value.length === availableSources.length);

const toggleAll = () => {
  if (isAllSelected.value) {
    selectedSources.value = [];
  } else {
    selectedSources.value = availableSources.map(s => s.id);
    showError.value = false;
  }
};

const toggleSource = (id: string) => {
  if (selectedSources.value.includes(id)) {
    selectedSources.value = selectedSources.value.filter(s => s !== id);
  } else {
    selectedSources.value.push(id);
  }
  if (selectedSources.value.length > 0) {
    showError.value = false;
  }
};

const handleSearch = () => {
  if (selectedSources.value.length === 0) {
    showError.value = true;
    setTimeout(() => showError.value = false, 3000);
    return;
  }
  if (!query.value.trim()) return;

  router.push({
    name: 'search',
    query: {
      q: query.value,
      sources: selectedSources.value.join(',')
    }
  });
};

const searchFor = (term: string) => {
  query.value = term;
  handleSearch();
};
</script>

<style scoped>
@keyframes fade-in {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes slide-up {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes gradient {
  0%, 100% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
}

@keyframes shimmer {
  0% {
    background-position: -200% 0;
  }
  100% {
    background-position: 200% 0;
  }
}

.animate-fade-in {
  animation: fade-in 0.6s ease-out forwards;
}

.animate-slide-up {
  animation: slide-up 0.4s ease-out forwards;
  opacity: 0;
}

.animate-gradient {
  background-size: 200% auto;
  animation: gradient 3s ease infinite;
}

.animate-shimmer {
  background-size: 200% 100%;
  animation: shimmer 2s linear infinite;
}
</style>