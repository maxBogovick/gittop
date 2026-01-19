import { defineStore } from 'pinia';
import { ref } from 'vue';
import { type Crate, getCrates } from '../services/tauriApi';

export const useCratesStore = defineStore('crates', () => {
    const crates = ref<Crate[]>([]);
    const isLoading = ref(false);
    const error = ref<string | null>(null);
    const query = ref('');
    const sort = ref('downloads');
    const page = ref(1);
    const hasMore = ref(true);

    async function loadCrates(append = false) {
        if (!append) {
            page.value = 1;
            crates.value = [];
            hasMore.value = true;
        }
        isLoading.value = true;
        error.value = null;
        try {
            const fetched = await getCrates(query.value, sort.value, page.value);
            if (fetched.length < 30) hasMore.value = false;
            if (append) crates.value.push(...fetched);
            else crates.value = fetched;
            page.value++;
        } catch (e: any) {
            error.value = e.message || "Failed to load crates";
        } finally {
            isLoading.value = false;
        }
    }

    return { crates, isLoading, error, query, sort, hasMore, loadCrates };
});
