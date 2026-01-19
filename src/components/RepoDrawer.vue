<template>
  <div class="fixed inset-0 z-50 flex justify-end" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">
    <!-- Backdrop -->
    <div class="drawer-backdrop fixed inset-0 bg-slate-900/30 backdrop-blur-sm transition-opacity" aria-hidden="true" @click="$emit('close')"></div>

    <!-- Panel -->
    <div class="drawer-panel w-screen max-w-2xl transform transition-transform bg-white shadow-2xl flex flex-col h-full relative z-10">
      
      <!-- Header -->
      <div class="flex-shrink-0 px-6 py-4 border-b border-slate-100 bg-white/80 backdrop-blur-md sticky top-0 z-20 flex items-center justify-between">
        <div class="flex items-center gap-3 overflow-hidden">
            <img v-if="repo.owner_avatar_url" :src="repo.owner_avatar_url" class="h-10 w-10 rounded-full ring-2 ring-slate-100" />
            <div class="min-w-0">
              <h2 class="text-lg font-bold text-slate-900 truncate" id="slide-over-title">
                {{ repo.owner }} <span class="text-slate-400 font-normal">/</span> {{ repo.name }}
              </h2>
              <div class="flex items-center gap-2 text-sm text-slate-500">
                <span v-if="repo.primary_language" class="flex items-center gap-1">
                  <span class="w-2 h-2 rounded-full bg-blue-500"></span>
                  {{ repo.primary_language }}
                </span>
                <span>•</span>
                <span class="flex items-center gap-1">
                  <svg class="h-3 w-3" fill="currentColor" viewBox="0 0 20 20"><path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/></svg>
                  {{ repo.stars_total }}
                </span>
              </div>
            </div>
        </div>
        <button type="button" class="rounded-full p-2 text-slate-400 hover:text-slate-600 hover:bg-slate-100 transition-colors focus:outline-none" @click="$emit('close')">
          <span class="sr-only">Close panel</span>
          <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Description & Link -->
      <div class="px-6 py-4 bg-slate-50 border-b border-slate-100">
         <p class="text-sm text-slate-600 leading-relaxed">{{ repo.description || 'No description provided.' }}</p>
         <div class="mt-3 flex gap-2">
            <a :href="repo.repository_url" target="_blank" class="inline-flex items-center px-4 py-2 border border-slate-200 text-sm font-medium rounded-lg text-slate-700 bg-white hover:bg-slate-50 hover:text-blue-600 transition-colors shadow-sm">
               <svg class="mr-2 h-4 w-4" fill="currentColor" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd"/></svg>
               View on GitHub
            </a>
         </div>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto px-6 py-6 custom-scrollbar">
         <div v-if="loading" class="animate-pulse space-y-4">
            <div class="h-4 bg-slate-200 rounded w-3/4"></div>
            <div class="h-4 bg-slate-200 rounded w-full"></div>
            <div class="h-4 bg-slate-200 rounded w-5/6"></div>
            <div class="space-y-2 mt-8">
               <div class="h-3 bg-slate-200 rounded w-full"></div>
               <div class="h-3 bg-slate-200 rounded w-full"></div>
               <div class="h-3 bg-slate-200 rounded w-4/5"></div>
            </div>
         </div>
         <div v-else-if="error" class="flex flex-col items-center justify-center h-64 text-red-500">
            <svg class="h-10 w-10 mb-2 opacity-50" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/></svg>
            {{ error }}
         </div>
         <div v-else class="prose prose-slate prose-sm max-w-none hover:prose-a:text-blue-600" v-html="readmeHtml"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import MarkdownIt from 'markdown-it';
import { getReadme, type Repository } from '../services/tauriApi';

const props = defineProps<{
  repo: Repository
}>();

defineEmits<{
  (e: 'close'): void
}>();

const loading = ref(true);
const error = ref<string | null>(null);
const readmeHtml = ref('');

const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true
});

async function fetchReadme() {
  loading.value = true;
  error.value = null;
  readmeHtml.value = '';
  
  try {
    const markdown = await getReadme(props.repo.owner, props.repo.name);
    readmeHtml.value = md.render(markdown);
  } catch (e: any) {
    error.value = "Failed to load README.";
    console.error(e);
  } finally {
    loading.value = false;
  }
}

onMounted(() => {
  fetchReadme();
});
</script>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background-color: #cbd5e1;
  border-radius: 20px;
}
</style>


