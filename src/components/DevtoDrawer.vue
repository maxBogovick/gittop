<template>
  <div class="fixed inset-0 z-50 flex justify-end" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">
    <!-- Backdrop -->
    <div class="drawer-backdrop fixed inset-0 bg-slate-900/50 dark:bg-slate-950/70 backdrop-blur-sm transition-opacity" aria-hidden="true" @click="$emit('close')"></div>

    <!-- Panel -->
    <div class="drawer-panel w-screen max-w-2xl transform transition-transform bg-[var(--color-bg-secondary)] shadow-2xl flex flex-col h-full relative z-10">

      <!-- Header -->
      <div class="flex-shrink-0 px-6 py-4 border-b border-[var(--color-border-primary)] bg-[var(--color-bg-secondary)]/80 backdrop-blur-md sticky top-0 z-20 flex items-start justify-between">
        <div class="flex items-start gap-4 min-w-0 pr-4">
           <img v-if="article.cover_image" :src="article.cover_image" class="h-16 w-24 object-cover rounded-lg border border-[var(--color-border-primary)] shadow-sm hidden sm:block" />
           <div class="flex-1 min-w-0">
              <h2 class="text-lg font-bold text-[var(--color-text-primary)] leading-snug line-clamp-2" id="slide-over-title">{{ article.title }}</h2>
              <div class="flex flex-wrap items-center gap-2 text-sm text-[var(--color-text-tertiary)] mt-1">
                <span class="font-medium text-[var(--color-text-secondary)]">{{ article.user.name }}</span>
                <span class="text-[var(--color-text-muted)]">@{{ article.user.username }}</span>
                <span>•</span>
                <span>{{ formatDate(article.published_at) }}</span>
              </div>
           </div>
        </div>
        <button type="button" class="rounded-full p-2 text-[var(--color-text-muted)] hover:text-[var(--color-text-secondary)] hover:bg-[var(--color-bg-hover)] transition-colors focus:outline-none flex-shrink-0" @click="$emit('close')">
          <span class="sr-only">Close panel</span>
          <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Actions -->
      <div class="px-6 py-3 bg-[var(--color-bg-tertiary)] border-b border-[var(--color-border-primary)] flex flex-col gap-3">
         <div class="flex flex-wrap gap-2">
            <span v-for="tag in article.tag_list" :key="tag" class="px-2 py-0.5 rounded text-xs font-medium bg-[var(--color-bg-hover)] text-[var(--color-text-secondary)]">#{{ tag }}</span>
         </div>
         <div class="flex gap-2">
            <a :href="article.url" target="_blank" class="inline-flex items-center px-4 py-2 border border-[var(--color-border-primary)] text-sm font-medium rounded-lg text-[var(--color-text-secondary)] bg-[var(--color-bg-secondary)] hover:bg-[var(--color-bg-hover)] hover:text-[var(--color-accent-primary)] transition-colors shadow-sm">
               Read on Dev.to
            </a>
         </div>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto px-6 py-6 custom-scrollbar">
         <div v-if="loading" class="animate-pulse space-y-6">
            <div class="h-4 bg-[var(--color-bg-tertiary)] rounded w-3/4"></div>
            <div class="h-4 bg-[var(--color-bg-tertiary)] rounded w-full"></div>
            <div class="h-4 bg-[var(--color-bg-tertiary)] rounded w-5/6"></div>
            <div class="h-64 bg-[var(--color-bg-tertiary)] rounded-lg w-full"></div>
         </div>
         <div v-else-if="error" class="flex flex-col items-center justify-center h-64 text-red-500">
             <svg class="h-10 w-10 mb-2 opacity-50" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/></svg>
            {{ error }}
         </div>
         <div v-else class="prose prose-slate dark:prose-invert max-w-none text-[var(--color-text-secondary)] leading-relaxed">
            <div v-html="contentHtml"></div>
         </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { getDevtoArticleDetails, type DevtoArticle } from '../services/tauriApi';

const props = defineProps<{
  article: DevtoArticle
}>();

defineEmits<{
  (e: 'close'): void
}>();

const loading = ref(true);
const error = ref<string | null>(null);
const contentHtml = ref('');

function formatDate(dateStr: string): string {
  if (!dateStr) return '';
  const date = new Date(dateStr);
  return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(date);
}

async function fetchDetails() {
  loading.value = true;
  error.value = null;
  contentHtml.value = '';

  try {
    const details = await getDevtoArticleDetails(props.article.id);
    contentHtml.value = details.body_html || '<p>No content available.</p>';
  } catch (e: any) {
    error.value = "Failed to load article details.";
    console.error(e);
  } finally {
    loading.value = false;
  }
}

onMounted(() => {
  fetchDetails();
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
  background-color: var(--color-border-primary);
  border-radius: 20px;
}

:deep(img) {
    max-width: 100%;
    height: auto;
    border-radius: 0.5rem;
    margin: 1.5rem 0;
}
:deep(pre) {
    background-color: var(--color-bg-tertiary);
    padding: 1rem;
    border-radius: 0.5rem;
    overflow-x: auto;
    font-size: 0.875rem;
    border: 1px solid var(--color-border-primary);
}
:deep(a) {
    color: var(--color-accent-primary);
    text-decoration: none;
}
:deep(a:hover) {
    text-decoration: underline;
}
:deep(h2), :deep(h3) {
    color: var(--color-text-primary);
    font-weight: 700;
}
</style>
