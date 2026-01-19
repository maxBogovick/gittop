import { defineStore } from 'pinia';
import { ref } from 'vue';
import { EtsyListing, getEtsyTop, getEtsyNew, searchEtsy } from '../services/tauriApi';

export const useEtsyStore = defineStore('etsy', () => {
    const keyword = ref<string>("");
    const offset = ref(0);
    const limit = 30;
    const listings = ref<EtsyListing[]>([]);
    const isLoading = ref(false);
    const error = ref<string | null>(null);
    const hasMore = ref(true);

    function reset() {
        offset.value = 0;
        listings.value = [];
        hasMore.value = true;
        error.value = null;
    }

    async function loadTopListings(append: boolean = false) {
        if (!append) reset();
        if (!hasMore.value && append) return;

        isLoading.value = true;
        error.value = null;
        try {
            let newListings: EtsyListing[];
            if (keyword.value) {
                newListings = await searchEtsy(keyword.value, limit, offset.value);
            } else {
                newListings = await getEtsyTop(limit, offset.value);
            }
            
            if (newListings.length < limit) {
                hasMore.value = false;
            }
            if (append) {
                listings.value.push(...newListings);
            } else {
                listings.value = newListings;
            }
            if (newListings.length > 0) offset.value += limit;
        } catch (e: any) {
            error.value = e.toString();
        } finally {
            isLoading.value = false;
        }
    }

    async function loadNewListings(append: boolean = false) {
        if (!append) reset();
        if (!hasMore.value && append) return;

        isLoading.value = true;
        error.value = null;
        try {
            let newListings: EtsyListing[];
             if (keyword.value) {
                // If searching, we likely want to stick to search, maybe sorted by new?
                // The current backend search hardcodes sort by "score". 
                // For now, we'll just use the same search for simplicity or maybe we should fallback to top search?
                // Let's assume search is universal.
                newListings = await searchEtsy(keyword.value, limit, offset.value);
            } else {
                newListings = await getEtsyNew(limit, offset.value);
            }

            if (newListings.length < limit) {
                hasMore.value = false;
            }
            if (append) {
                listings.value.push(...newListings);
            } else {
                listings.value = newListings;
            }
            if (newListings.length > 0) offset.value += limit;
        } catch (e: any) {
            error.value = e.toString();
        } finally {
            isLoading.value = false;
        }
    }

    return {
        keyword,
        offset,
        listings,
        isLoading,
        error,
        hasMore,
        loadTopListings,
        loadNewListings
    };
});
