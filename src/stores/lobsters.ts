import { defineStore } from 'pinia';
import { ref } from 'vue';
import { type LobstersStory, getLobstersStories } from '../services/tauriApi';

export const useLobstersStore = defineStore('lobsters', () => {
    const stories = ref<LobstersStory[]>([]);
    const isLoading = ref(false);
    const error = ref<string | null>(null);
    const storyType = ref('hottest');

    async function loadStories() {
        isLoading.value = true;
        error.value = null;
        try {
            stories.value = await getLobstersStories(storyType.value);
        } catch (e: any) {
            error.value = e.message || "Failed to load Lobsters stories";
        } finally {
            isLoading.value = false;
        }
    }

    return { stories, isLoading, error, storyType, loadStories };
});
