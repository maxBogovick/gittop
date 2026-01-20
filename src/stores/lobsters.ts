import { defineStore } from 'pinia';
import { ref } from 'vue';
import { type LobstersStory, getLobstersStories, getLobstersStoryDetails } from '../services/tauriApi';

export const useLobstersStore = defineStore('lobsters', () => {
    const stories = ref<LobstersStory[]>([]);
    const selectedStory = ref<LobstersStory | null>(null);
    const isLoading = ref(false);
    const isLoadingDetails = ref(false);
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

    async function loadStoryDetails(shortId: string) {
        isLoadingDetails.value = true;
        // Don't clear selectedStory immediately to prevent flickering if we are just refreshing
        // But if we are switching stories, we might want to clear or handle it in UI
        try {
             // Fetch full details including comments
            const details = await getLobstersStoryDetails(shortId);
            selectedStory.value = details;
        } catch (e: any) {
            console.error("Failed to load story details:", e);
        } finally {
            isLoadingDetails.value = false;
        }
    }

    return { stories, selectedStory, isLoading, isLoadingDetails, error, storyType, loadStories, loadStoryDetails };
});
