import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { TimeRange, type RedditPost, getRedditTop, getRedditNew, searchReddit } from '../services/tauriApi';

export const useRedditStore = defineStore('reddit', () => {
    const timeRange = ref<TimeRange>(TimeRange.Week);
    const subreddit = ref<string>(""); 
    const keyword = ref<string>("");
    const afterCursor = ref<string | undefined>(undefined);
    const posts = ref<RedditPost[]>([]);
    const isLoading = ref(false);
    const error = ref<string | null>(null);
    const hasMore = ref(true);
    
    // Sorting state
    const sortBy = ref<string>('score'); // score, comments, date
    const sortDesc = ref(true);

    const sortedPosts = computed(() => {
        const p = [...posts.value];
        return p.sort((a, b) => {
            let valA, valB;
            switch (sortBy.value) {
                case 'score':
                    valA = a.score;
                    valB = b.score;
                    break;
                case 'comments':
                    valA = a.num_comments;
                    valB = b.num_comments;
                    break;
                case 'date':
                    valA = new Date(a.created_utc).getTime();
                    valB = new Date(b.created_utc).getTime();
                    break;
                default:
                    return 0;
            }
            if (valA < valB) return sortDesc.value ? 1 : -1;
            if (valA > valB) return sortDesc.value ? -1 : 1;
            return 0;
        });
    });

    function sort(column: string) {
        if (sortBy.value === column) {
            sortDesc.value = !sortDesc.value;
        } else {
            sortBy.value = column;
            sortDesc.value = true;
        }
    }

    function reset() {
        afterCursor.value = undefined;
        posts.value = [];
        hasMore.value = true;
        error.value = null;
        // Keep sort settings? Yes.
    }

    async function loadTopPosts(append: boolean = false) {
        if (!append) reset();
        if (!hasMore.value && append) return;

        isLoading.value = true;
        error.value = null;
        try {
            let result;
            if (keyword.value.trim()) {
                result = await searchReddit(keyword.value, timeRange.value, afterCursor.value);
            } else {
                result = await getRedditTop(subreddit.value || "all", timeRange.value, afterCursor.value);
            }

            if (result.posts.length === 0) {
                hasMore.value = false;
            } else {
                if (append) {
                    posts.value.push(...result.posts);
                } else {
                    posts.value = result.posts;
                }
                afterCursor.value = result.after;
                if (!result.after) hasMore.value = false;
            }
        } catch (e: any) {
            error.value = e.toString();
        } finally {
            isLoading.value = false;
        }
    }

    async function loadNewPosts(append: boolean = false) {
        if (!append) reset();
        if (!hasMore.value && append) return;

        isLoading.value = true;
        error.value = null;
        try {
            let result;
            if (keyword.value.trim()) {
                result = await searchReddit(keyword.value, timeRange.value, afterCursor.value);
            } else {
                result = await getRedditNew(subreddit.value || "all", afterCursor.value);
            }

            if (result.posts.length === 0) {
                hasMore.value = false;
            } else {
                if (append) {
                    posts.value.push(...result.posts);
                } else {
                    posts.value = result.posts;
                }
                afterCursor.value = result.after;
                if (!result.after) hasMore.value = false;
            }
        } catch (e: any) {
            error.value = e.toString();
        } finally {
            isLoading.value = false;
        }
    }

    return {
        timeRange,
        subreddit,
        keyword,
        posts,
        sortedPosts,
        sortBy,
        sortDesc,
        isLoading,
        error,
        hasMore,
        loadTopPosts,
        loadNewPosts,
        sort
    };
});