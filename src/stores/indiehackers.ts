import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { type IndieHackersPost, getIndieHackersPosts } from '../services/tauriApi';

export const useIndieHackersStore = defineStore('indiehackers', () => {
    const allPosts = ref<IndieHackersPost[]>([]);
    const isLoading = ref(false);
    const error = ref<string | null>(null);
    const searchQuery = ref<string>('');

    const posts = computed(() => {
        if (!searchQuery.value.trim()) {
            return allPosts.value;
        }
        const query = searchQuery.value.toLowerCase();
        return allPosts.value.filter(post =>
            post.title.toLowerCase().includes(query) ||
            post.content.toLowerCase().includes(query) ||
            post.author.toLowerCase().includes(query)
        );
    });

    async function loadPosts() {
        isLoading.value = true;
        error.value = null;
        try {
            allPosts.value = await getIndieHackersPosts();
        } catch (e: any) {
            error.value = e.message || "Failed to load Indie Hackers posts";
        } finally {
            isLoading.value = false;
        }
    }

    return { posts, allPosts, isLoading, error, searchQuery, loadPosts };
});
