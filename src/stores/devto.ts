import { defineStore } from 'pinia';
import { ref } from 'vue';
import { type DevtoArticle, getDevtoArticles } from '../services/tauriApi';

export const useDevtoStore = defineStore('devto', () => {
  const articles = ref<DevtoArticle[]>([]);
  const isLoading = ref(false);
  const error = ref<string | null>(null);
  const page = ref(1);
  const hasMore = ref(true);
  
  // Filters
  const tag = ref<string>(''); // Acts as "Language" or "Topic"
  const top = ref<number | null>(null); // Acts as "Time Range" (7, 30, 365, etc.)
  const state = ref<string | null>('rising'); // 'fresh', 'rising', 'all'

  async function loadArticles(append: boolean = false) {
    if (isLoading.value) return;
    
    if (!append) {
      page.value = 1;
      articles.value = [];
      hasMore.value = true;
    }
    
    isLoading.value = true;
    error.value = null;
    
    try {
      // If we are in "New" mode (state=fresh), top should probably be null or ignored
      // If we are in "Top" mode, top should be set (e.g. 7 days) and state can be default or 'rising'
      
      const fetched = await getDevtoArticles(
        tag.value || null,
        top.value,
        state.value || null,
        page.value
      );
      
      if (fetched.length === 0) {
        hasMore.value = false;
      } else {
        if (append) {
          articles.value.push(...fetched);
        } else {
          articles.value = fetched;
        }
        page.value++;
      }
    } catch (e: any) {
      error.value = e.message || "Failed to load articles";
    } finally {
      isLoading.value = false;
    }
  }

  return {
    articles,
    isLoading,
    error,
    page,
    hasMore,
    tag,
    top,
    state,
    loadArticles
  };
});
