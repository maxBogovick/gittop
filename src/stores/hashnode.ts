import { defineStore } from 'pinia';
import { ref } from 'vue';
import { type HashnodePost, getHashnodePosts } from '../services/tauriApi';

export const useHashnodeStore = defineStore('hashnode', () => {
    const posts = ref<HashnodePost[]>([]);
    const isLoading = ref(false);
    const error = ref<string | null>(null);
    const hasMore = ref(true);
    const endCursor = ref<string | undefined>(undefined);

    const tag = ref<string>('');
    const feedType = ref<string>('FEATURED');

    async function loadPosts(append: boolean = false) {
        if (isLoading.value) return;

        if (!append) {
            posts.value = [];
            hasMore.value = true;
            endCursor.value = undefined;
        }

        isLoading.value = true;
        error.value = null;

        try {
            const tagValue = tag.value?.trim() || undefined;
            const fetched = await getHashnodePosts(tagValue, feedType.value, endCursor.value);

            if (fetched.length === 0) {
                hasMore.value = false;
            } else {
                if (append) {
                    posts.value.push(...fetched);
                } else {
                    posts.value = fetched;
                }
                hasMore.value = false;
            }
        } catch (e: any) {
            error.value = e.message || "Failed to load posts";
        } finally {
            isLoading.value = false;
        }
    }

    return {
        posts,
        isLoading,
        error,
        hasMore,
        tag,
        feedType,
        loadPosts
    };
});
