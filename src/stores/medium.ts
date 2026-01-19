import { defineStore } from 'pinia';
import { ref } from 'vue';
import { type MediumArticle, getMediumArticles } from '../services/tauriApi';

export const useMediumStore = defineStore('medium', () => {
    const articles = ref<MediumArticle[]>([]);
    const isLoading = ref(false);
    const error = ref<string | null>(null);

    const tag = ref<string>('technology');

    async function loadArticles() {
        isLoading.value = true;
        error.value = null;

        try {
            articles.value = await getMediumArticles(tag.value);
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
        tag,
        loadArticles
    };
});
