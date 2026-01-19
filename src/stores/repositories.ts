import { defineStore } from 'pinia';
import { ref } from 'vue';
import { TimeRange, Metric, Repository, getTopRepositories, getNewRepositories } from '../services/tauriApi';

export const useRepositoryStore = defineStore('repositories', () => {
    const timeRange = ref<TimeRange>(TimeRange.Week);
    const metric = ref<Metric>(Metric.Stars);
    const language = ref<string>(""); 
    const keyword = ref<string>("");
    const page = ref(1);
    const repositories = ref<Repository[]>([]);
    const isLoading = ref(false);
    const error = ref<string | null>(null);
    const hasMore = ref(true);

    function reset() {
        page.value = 1;
        repositories.value = [];
        hasMore.value = true;
        error.value = null;
    }

    async function loadTopRepositories(append: boolean = false) {
        if (!append) reset();
        if (!hasMore.value && append) return;

        isLoading.value = true;
        error.value = null;
        try {
            const newRepos = await getTopRepositories(timeRange.value, metric.value, language.value, keyword.value, page.value);
            if (newRepos.length < 30) {
                hasMore.value = false;
            }
            if (append) {
                repositories.value.push(...newRepos);
            } else {
                repositories.value = newRepos;
            }
            if (newRepos.length > 0) page.value++;
        } catch (e: any) {
            error.value = e.toString();
        } finally {
            isLoading.value = false;
        }
    }

    async function loadNewRepositories(append: boolean = false) {
        if (!append) reset();
         if (!hasMore.value && append) return;

        isLoading.value = true;
        error.value = null;
        try {
            const newRepos = await getNewRepositories(timeRange.value, metric.value, language.value, keyword.value, page.value);
             if (newRepos.length < 30) {
                hasMore.value = false;
            }
            if (append) {
                repositories.value.push(...newRepos);
            } else {
                repositories.value = newRepos;
            }
            if (newRepos.length > 0) page.value++;
        } catch (e: any) {
            error.value = e.toString();
        } finally {
            isLoading.value = false;
        }
    }

    return {
        timeRange,
        metric,
        language,
        keyword,
        page,
        repositories,
        isLoading,
        error,
        hasMore,
        loadTopRepositories,
        loadNewRepositories
    };
});