<template>
  <div v-if="isOpen" class="fixed inset-0 overflow-hidden z-50" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">
    <div class="absolute inset-0 overflow-hidden">
      <div class="absolute inset-0 bg-slate-900/50 dark:bg-slate-950/70 backdrop-blur-sm transition-opacity" @click="close"></div>
      <div class="pointer-events-none fixed inset-y-0 right-0 flex max-w-full pl-10">
        <div class="pointer-events-auto w-screen max-w-2xl transform transition-transform duration-300 ease-in-out">
          <div class="flex h-full flex-col overflow-y-scroll bg-[var(--color-bg-secondary)] shadow-xl">
            <!-- Header -->
            <div class="px-6 py-6 border-b border-[var(--color-border-primary)] bg-[var(--color-bg-secondary)] sticky top-0 z-10">
              <div class="flex items-start justify-between">
                <h2 class="text-lg font-bold text-[var(--color-text-primary)] leading-snug">
                    {{ story?.title || 'Loading...' }}
                </h2>
                <div class="ml-3 flex h-7 items-center">
                  <button type="button" class="rounded-md bg-[var(--color-bg-secondary)] text-[var(--color-text-muted)] hover:text-[var(--color-text-secondary)] focus:outline-none" @click="close">
                    <span class="sr-only">Close panel</span>
                    <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>
              </div>
              <div class="mt-4 flex flex-wrap gap-2 text-xs">
                <a :href="story?.url" target="_blank" class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md bg-red-50 dark:bg-red-900/30 text-red-700 dark:text-red-400 hover:bg-red-100 dark:hover:bg-red-900/50 transition-colors font-medium">
                  Read Article
                  <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" /></svg>
                </a>
                <a :href="story?.comments_url" target="_blank" class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md bg-[var(--color-bg-tertiary)] text-[var(--color-text-secondary)] hover:bg-[var(--color-bg-hover)] transition-colors font-medium">
                  Lobste.rs Link
                  <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1" /></svg>
                </a>
                <span class="inline-flex items-center gap-1 px-2.5 py-1.5 rounded-md bg-orange-50 dark:bg-orange-900/30 text-orange-700 dark:text-orange-400 font-medium">
                    <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 15l7-7 7 7" /></svg>
                    {{ story?.score }} points
                </span>
                <span class="inline-flex items-center gap-1 px-2.5 py-1.5 rounded-md bg-[var(--color-bg-tertiary)] text-[var(--color-text-tertiary)]">
                    by {{ story?.submitter_user }}
                </span>
              </div>
              <div v-if="story?.tags && story.tags.length" class="mt-3 flex flex-wrap gap-1">
                 <span v-for="tag in story.tags" :key="tag" class="px-2 py-0.5 text-[10px] uppercase font-bold tracking-wider rounded border border-[var(--color-border-primary)] text-[var(--color-text-tertiary)]">
                     {{ tag }}
                 </span>
              </div>
            </div>

            <!-- Content -->
            <div class="relative mt-2 flex-1 px-4 sm:px-6">
                <div v-if="isLoading" class="py-12 flex justify-center">
                    <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-[var(--color-text-primary)]"></div>
                </div>

                <div v-else-if="story?.comments" class="pb-10">
                    <div class="px-2 py-3 border-b border-[var(--color-border-primary)] mb-4">
                        <h3 class="text-sm font-bold text-[var(--color-text-primary)] uppercase tracking-wide">{{ story.comments.length }} Comments</h3>
                    </div>

                    <div v-for="comment in story.comments" :key="comment.short_id"
                         :style="{ paddingLeft: `${Math.min(comment.depth * 20, 200)}px` }"
                         class="mb-4 group relative">

                        <!-- Thread line for nested comments -->
                        <div v-if="comment.depth > 0" class="absolute left-0 top-0 bottom-0 border-l border-[var(--color-border-secondary)]"
                             :style="{ left: `${Math.min(comment.depth * 20, 200) - 10}px` }"></div>

                        <div class="bg-[var(--color-bg-secondary)] p-3 rounded-lg border border-transparent hover:border-[var(--color-border-primary)] transition-colors">
                            <div class="flex items-center gap-2 mb-2 text-xs text-[var(--color-text-tertiary)]">
                                <span class="font-bold text-[var(--color-text-secondary)]">{{ comment.commenting_user }}</span>
                                <span>&bull;</span>
                                <span>{{ formatDate(comment.created_at) }}</span>
                                <span>&bull;</span>
                                <span class="text-orange-600 dark:text-orange-400 font-medium" v-if="comment.score > 0">+{{ comment.score }}</span>
                            </div>

                            <div class="prose prose-sm prose-slate dark:prose-invert max-w-none text-[var(--color-text-primary)] break-words" v-html="comment.comment"></div>
                        </div>
                    </div>

                    <div v-if="story.comments.length === 0" class="text-center py-12 text-[var(--color-text-tertiary)]">
                        No comments yet.
                    </div>
                </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useLobstersStore } from '../stores/lobsters';

const store = useLobstersStore();
const props = defineProps<{
    modelValue: boolean
}>();

const emit = defineEmits<{
    (e: 'update:modelValue', value: boolean): void
}>();

const isOpen = computed({
    get: () => props.modelValue,
    set: (val) => emit('update:modelValue', val)
});

const story = computed(() => store.selectedStory);
const isLoading = computed(() => store.isLoadingDetails);

function close() {
    isOpen.value = false;
}

function formatDate(dateStr: string): string {
    if (!dateStr) return '';
    const date = new Date(dateStr);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffSec = Math.floor(diffMs / 1000);
    const diffMin = Math.floor(diffSec / 60);
    const diffHour = Math.floor(diffMin / 60);
    const diffDay = Math.floor(diffHour / 24);

    if (diffSec < 60) return 'just now';
    if (diffMin < 60) return `${diffMin}m ago`;
    if (diffHour < 24) return `${diffHour}h ago`;
    if (diffDay < 7) return `${diffDay}d ago`;

    return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(date);
}
</script>

<style scoped>
/* Scoped styles for comment HTML content */
:deep(.prose p) {
    margin-bottom: 0.5em;
}
:deep(.prose p:last-child) {
    margin-bottom: 0;
}
:deep(.prose a) {
    color: var(--color-accent-primary);
    text-decoration: none;
}
:deep(.prose a:hover) {
    text-decoration: underline;
}
:deep(.prose pre) {
    background-color: var(--color-bg-tertiary);
    padding: 0.5rem;
    border-radius: 0.375rem;
    overflow-x: auto;
    font-size: 0.875em;
    margin-top: 0.5em;
    margin-bottom: 0.5em;
}
:deep(.prose blockquote) {
    border-left: 3px solid var(--color-border-primary);
    padding-left: 1rem;
    font-style: italic;
    color: var(--color-text-tertiary);
    margin-top: 0.5em;
    margin-bottom: 0.5em;
}
</style>
