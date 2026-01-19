import { defineStore } from 'pinia';
import { ref } from 'vue';
import { type HackerNewsStory, getHackerNewsStories, searchHackerNews } from '../services/tauriApi';

export const useHackerNewsStore = defineStore('hackernews', () => {
    const stories = ref<HackerNewsStory[]>([]);
    const isLoading = ref(false);
    const error = ref<string | null>(null);
    const page = ref(1);
    const hasMore = ref(true);

    const storyType = ref<string>('top'); // top, new, best, show, ask, job
    const searchQuery = ref<string>('');
    const searchSort = ref<string>('popularity'); // popularity, date
    const searchTimeRange = ref<string>(''); // day, week, month, year, or empty for all time

    async function loadStories(append: boolean = false) {
        if (isLoading.value) return;

        if (!append) {
            page.value = 1;
            stories.value = [];
            hasMore.value = true;
        }

        isLoading.value = true;
        error.value = null;

        try {
            let fetched: HackerNewsStory[];

            if (searchQuery.value.trim()) {
                // Use search API
                fetched = await searchHackerNews(
                    searchQuery.value.trim(),
                    searchSort.value,
                    searchTimeRange.value || undefined,
                    page.value
                );
            } else {
                // Use regular stories API
                fetched = await getHackerNewsStories(storyType.value, page.value);
            }

            if (fetched.length === 0) {
                hasMore.value = false;
            } else {
                if (append) {
                    stories.value.push(...fetched);
                } else {
                    stories.value = fetched;
                }
                page.value++;
            }
        } catch (e: any) {
            error.value = e.message || "Failed to load stories";
        } finally {
            isLoading.value = false;
        }
    }

    return {
        stories,
        isLoading,
        error,
        page,
        hasMore,
        storyType,
        searchQuery,
        searchSort,
        searchTimeRange,
        loadStories
    };
});
