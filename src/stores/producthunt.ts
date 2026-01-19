import { defineStore } from 'pinia';
import { ref } from 'vue';
import { type ProductHuntPost, getProductHuntPosts } from '../services/tauriApi';

export const useProductHuntStore = defineStore('producthunt', () => {
    const posts = ref<ProductHuntPost[]>([]);
    const isLoading = ref(false);
    const error = ref<string | null>(null);

    async function loadPosts() {
        isLoading.value = true;
        error.value = null;
        try {
            posts.value = await getProductHuntPosts();
        } catch (e: any) {
            error.value = e.message || "Failed to load Product Hunt posts";
        } finally {
            isLoading.value = false;
        }
    }

    return { posts, isLoading, error, loadPosts };
});
