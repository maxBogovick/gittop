import { defineStore } from 'pinia';
import { ref } from 'vue';
import { type StackOverflowQuestion, getStackOverflowQuestions } from '../services/tauriApi';

export const useStackOverflowStore = defineStore('stackoverflow', () => {
    const questions = ref<StackOverflowQuestion[]>([]);
    const isLoading = ref(false);
    const error = ref<string | null>(null);
    const page = ref(1);
    const hasMore = ref(true);

    const tag = ref<string>('');
    const sort = ref<string>('votes'); // votes, activity, creation, hot

    async function loadQuestions(append: boolean = false) {
        if (isLoading.value) return;

        if (!append) {
            page.value = 1;
            questions.value = [];
            hasMore.value = true;
        }

        isLoading.value = true;
        error.value = null;

        try {
            const fetched = await getStackOverflowQuestions(tag.value, sort.value, page.value);
            
            if (fetched.length === 0) {
                hasMore.value = false;
            } else {
                if (append) {
                    questions.value.push(...fetched);
                } else {
                    questions.value = fetched;
                }
                page.value++;
            }
        } catch (e: any) {
            error.value = e.message || "Failed to load questions";
        } finally {
            isLoading.value = false;
        }
    }

    return {
        questions,
        isLoading,
        error,
        page,
        hasMore,
        tag,
        sort,
        loadQuestions
    };
});
