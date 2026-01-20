<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { searchAll, UnifiedSearchResult } from '../services/tauriApi';
import ViewHeader from '../components/ViewHeader.vue';
import LoadingSkeleton from '../components/LoadingSkeleton.vue';
import { 
    StarIcon, 
    ChatBubbleLeftIcon, 
    ArrowTopRightOnSquareIcon, 
    UserIcon,
    AdjustmentsHorizontalIcon,
    FunnelIcon,
    MagnifyingGlassIcon
} from '@heroicons/vue/24/outline';
import { openUrl } from '@tauri-apps/plugin-opener';

// Drawers
import RepoDrawer from '../components/RepoDrawer.vue';
import RedditDrawer from '../components/RedditDrawer.vue';
import HackerNewsDrawer from '../components/HackerNewsDrawer.vue';
import StackOverflowDrawer from '../components/StackOverflowDrawer.vue';
import DevtoDrawer from '../components/DevtoDrawer.vue';
import LobstersDrawer from '../components/LobstersDrawer.vue';
import CratesDrawer from '../components/CratesDrawer.vue';
import EtsyDrawer from '../components/EtsyDrawer.vue';
import MediumDrawer from '../components/MediumDrawer.vue';
import HashnodeDrawer from '../components/HashnodeDrawer.vue';
import IndieHackersDrawer from '../components/IndieHackersDrawer.vue';

// --- State ---
const route = useRoute();
const router = useRouter();
const query = ref('');
const searchInput = ref(''); 
const results = ref<UnifiedSearchResult[]>([]);
const loading = ref(false);
const loadingMore = ref(false);
const error = ref<string | null>(null);
const page = ref(1);
const hasMore = ref(true); 

// Drawer State
const selectedItem = ref<UnifiedSearchResult | null>(null);

// Filters State
const availableSources = [
    'GitHub', 'Crates.io', 
    'StackOverflow', 'Reddit', 'HackerNews', 'Lobsters', 'IndieHackers',
    'Dev.to', 'Medium', 'Hashnode', 
    'ProductHunt', 'Etsy'
];
const selectedSources = ref<string[]>([]);
const timeFilter = ref('all');
const sortBy = ref('relevance');
const groupBy = ref('category');

// --- Search Logic ---
const performSearch = async (reset = true) => {
  const q = route.query.q as string;
  if (!q) return;
  
  query.value = q;
  searchInput.value = q; 

  if (reset) {
      loading.value = true;
      page.value = 1;
      results.value = [];
      hasMore.value = true;
  } else {
      loadingMore.value = true;
      page.value++;
  }
  
  error.value = null;
  
  const sourcesParam = route.query.sources as string;
  const timeParam = route.query.time as string;
  const sortParam = route.query.sort as string;
  
  selectedSources.value = sourcesParam ? sourcesParam.split(',') : availableSources;
  timeFilter.value = timeParam || 'all';
  sortBy.value = sortParam || 'relevance';
  groupBy.value = (route.query.group as string) || 'category';

  try {
    const newResults = await searchAll(q, selectedSources.value, timeFilter.value, sortBy.value, page.value);
    
    if (reset) {
        results.value = newResults;
    } else {
        results.value.push(...newResults);
    }
    
    if (newResults.length === 0) {
        hasMore.value = false;
    }
  } catch (err: any) {
    error.value = "Failed to load search results: " + err;
    console.error(err);
  } finally {
    loading.value = false;
    loadingMore.value = false;
  }
};

const handleNewSearch = () => {
    if (!searchInput.value.trim()) return;
    router.push({
        query: {
            ...route.query,
            q: searchInput.value.trim()
        }
    });
};

const loadMore = () => {
    if (!loading.value && !loadingMore.value && hasMore.value) {
        performSearch(false);
    }
};

const handleItemClick = (item: UnifiedSearchResult) => {
    if (item.source === 'ProductHunt') {
        openUrl(item.url);
        return;
    }
    selectedItem.value = item;
};

// --- Computed Mappers for Drawers ---
const selectedRepo = computed(() => {
    if (selectedItem.value?.source !== 'GitHub') return null;
    const parts = selectedItem.value.id.split('/');
    return {
        owner: parts[0] || selectedItem.value.author || '',
        name: parts[1] || selectedItem.value.title,
        owner_avatar_url: '', // Missing
        primary_language: selectedItem.value.tags[0],
        stars_total: selectedItem.value.points || 0,
        forks: selectedItem.value.comment_count || 0, // Mapped forks to comment_count in search.rs
        open_issues: 0,
        repository_url: selectedItem.value.url,
        description: selectedItem.value.description,
        license: null,
        updated_at: selectedItem.value.created_at || '',
        last_activity_at: null
    };
});

const selectedRedditPost = computed(() => {
    if (selectedItem.value?.source !== 'Reddit') return null;
    return {
        id: selectedItem.value.id,
        title: selectedItem.value.title,
        author: selectedItem.value.author || 'unknown',
        subreddit: selectedItem.value.tags[0] || 'unknown',
        score: selectedItem.value.points || 0,
        num_comments: selectedItem.value.comment_count || 0,
        created_utc: selectedItem.value.created_at || '', // Format might mismatch
        url: selectedItem.value.url, // This is permalink in search.rs map
        selftext: selectedItem.value.description || '',
        permalink: selectedItem.value.url,
        thumbnail: 'default'
    };
});

const selectedHackerNewsStory = computed(() => {
    if (selectedItem.value?.source !== 'HackerNews') return null;
    return {
        id: Number(selectedItem.value.id),
        title: selectedItem.value.title,
        url: selectedItem.value.url,
        by: selectedItem.value.author || 'unknown',
        time: 0, // Timestamp parsing needed?
        score: selectedItem.value.points || 0,
        descendants: selectedItem.value.comment_count || 0,
        text: selectedItem.value.description
    };
});

const selectedStackOverflowQuestion = computed(() => {
    if (selectedItem.value?.source !== 'StackOverflow') return null;
    return {
        question_id: Number(selectedItem.value.id),
        title: selectedItem.value.title,
        link: selectedItem.value.url,
        score: selectedItem.value.points || 0,
        answer_count: selectedItem.value.comment_count || 0,
        view_count: 0,
        is_answered: false,
        tags: selectedItem.value.tags,
        creation_date: 0, // TS mismatch
        owner: { display_name: selectedItem.value.author },
        body: selectedItem.value.description // Not really body but summary
    };
});

const selectedLobstersStory = computed(() => {
    if (selectedItem.value?.source !== 'Lobsters') return null;
    return {
        short_id: selectedItem.value.id,
        title: selectedItem.value.title,
        url: selectedItem.value.url,
        score: selectedItem.value.points || 0,
        comment_count: selectedItem.value.comment_count || 0,
        tags: selectedItem.value.tags,
        created_at: selectedItem.value.created_at || '',
        comments_url: '',
        submitter_user: selectedItem.value.author || ''
    };
});

const selectedDevtoArticle = computed(() => {
    if (selectedItem.value?.source !== 'Dev.to') return null;
    return {
        id: Number(selectedItem.value.id),
        title: selectedItem.value.title,
        description: selectedItem.value.description,
        cover_image: null,
        social_image: null,
        published_at: selectedItem.value.created_at || '',
        tag_list: selectedItem.value.tags,
        slug: '',
        path: '',
        url: selectedItem.value.url,
        canonical_url: selectedItem.value.url,
        comments_count: selectedItem.value.comment_count || 0,
        public_reactions_count: selectedItem.value.points || 0,
        positive_reactions_count: selectedItem.value.points || 0,
        reading_time_minutes: 0,
        user: { name: selectedItem.value.author || '', username: '', profile_image: null },
        body_html: null,
        body_markdown: null
    };
});

// For others, we might mock or pass minimally. 
// Crates, Etsy, Medium, Hashnode, IndieHackers logic similar.
const selectedCrate = computed(() => {
    if (selectedItem.value?.source !== 'Crates.io') return null;
    return {
        id: selectedItem.value.id,
        name: selectedItem.value.title,
        description: selectedItem.value.description,
        downloads: selectedItem.value.points || 0,
        max_version: '',
        updated_at: selectedItem.value.created_at || '',
        homepage: selectedItem.value.url,
        repository: ''
    };
});

// --- Watchers ---
onMounted(() => {
  performSearch();
});

watch(() => route.query, (newVal, oldVal) => {
    if (newVal.q !== oldVal.q || 
        newVal.sources !== oldVal.sources || 
        newVal.time !== oldVal.time || 
        newVal.sort !== oldVal.sort) {
        performSearch(true);
    } else if (newVal.group !== oldVal.group) {
        groupBy.value = (newVal.group as string) || 'category';
    }
});

// --- Filter Updates ---
const updateFilters = () => {
    router.push({
        query: {
            ...route.query,
            sources: selectedSources.value.join(','),
            time: timeFilter.value,
            sort: sortBy.value,
            group: groupBy.value
        }
    });
};

const toggleSource = (source: string) => {
    if (selectedSources.value.includes(source)) {
        if (selectedSources.value.length > 1) { 
             selectedSources.value = selectedSources.value.filter(s => s !== source);
        }
    } else {
        selectedSources.value.push(source);
    }
    updateFilters();
};

const setTimeFilter = (val: string) => {
    timeFilter.value = val;
    updateFilters();
};

const setSortBy = (val: string) => {
    sortBy.value = val;
    updateFilters();
};

const setGroupBy = (val: string) => {
    groupBy.value = val;
    router.push({ query: { ...route.query, group: val } }); 
};

// --- Computed ---
const groupedResults = computed(() => {
    if (groupBy.value === 'none') {
        return [{ title: 'All Results', items: results.value }];
    }
    
    const keyFn = groupBy.value === 'category' 
        ? (item: UnifiedSearchResult) => item.category 
        : (item: UnifiedSearchResult) => item.source;

    const groups: Record<string, UnifiedSearchResult[]> = {};
    
    results.value.forEach(item => {
        const key = keyFn(item);
        if (!groups[key]) groups[key] = [];
        groups[key].push(item);
    });
    
    let keys = Object.keys(groups);
    if (groupBy.value === 'category') {
        const order = ['Code', 'Discussion', 'Article', 'Product'];
        keys.sort((a, b) => {
            const ixA = order.indexOf(a);
            const ixB = order.indexOf(b);
            return (ixA === -1 ? 99 : ixA) - (ixB === -1 ? 99 : ixB);
        });
    } else {
        keys.sort();
    }
    
    return keys.map(key => ({
        title: key,
        items: groups[key]
    }));
});

// --- UI Helpers ---
const getBadgeColor = (type: string) => {
    switch (type) {
        case 'Code': return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
        case 'Discussion': return 'bg-orange-100 text-orange-800 dark:bg-orange-900/30 dark:text-orange-300';
        case 'Article': return 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300';
        case 'Product': return 'bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-300';
        case 'GitHub': return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
        case 'Reddit': return 'bg-orange-100 text-orange-800 dark:bg-orange-900/30 dark:text-orange-300';
        case 'StackOverflow': return 'bg-orange-50 text-orange-700 dark:bg-orange-900/20 dark:text-orange-300';
        case 'HackerNews': return 'bg-orange-100 text-orange-800 dark:bg-orange-900/30 dark:text-orange-300';
        case 'Dev.to': return 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300';
        case 'Etsy': return 'bg-orange-100 text-orange-800';
        case 'ProductHunt': return 'bg-orange-100 text-orange-800';
        default: return 'bg-gray-100 text-gray-800';
    }
};

const formatDate = (dateStr?: string) => {
    if (!dateStr) return '';
    const date = new Date(dateStr);
    return date.toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' });
};
</script>

<template>
  <div class="h-full flex flex-col">
    <ViewHeader 
      title="Search Results" 
      subtitle="Aggregated results from multiple sources"
      :icon="MagnifyingGlassIcon"
    >
        <template #actions>
            <!-- Search Bar -->
            <div class="flex items-center gap-2 w-full sm:w-auto mr-auto">
                <div class="relative group w-full sm:w-80">
                    <input 
                        v-model="searchInput"
                        @keydown.enter="handleNewSearch"
                        type="text"
                        class="w-full pl-3 pr-10 py-2.5 bg-white/50 dark:bg-slate-800/50 backdrop-blur-sm border border-slate-200 dark:border-slate-700 rounded-xl text-sm text-slate-900 dark:text-white focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 outline-none transition-all shadow-sm group-hover:bg-white/80 dark:group-hover:bg-slate-800/80"
                        placeholder="Refine search..."
                    />
                     <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
                        <kbd class="text-[10px] text-slate-500 border border-slate-200 dark:border-slate-700 rounded px-1.5 py-0.5 bg-slate-100/50 dark:bg-slate-700/50">Enter</kbd>
                    </div>
                </div>
                <button 
                    @click="handleNewSearch"
                    class="px-4 py-2.5 bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-700 hover:to-indigo-700 text-white rounded-xl text-sm font-medium transition-all shadow-md hover:shadow-lg hover:-translate-y-0.5"
                >
                    Search
                </button>
            </div>

            <!-- Group By Controls -->
            <div class="flex items-center gap-2 text-sm mt-4 sm:mt-0">
                <span class="text-slate-500 font-medium whitespace-nowrap">Group by:</span>
                <div class="flex bg-slate-100/50 dark:bg-slate-800/50 backdrop-blur-sm rounded-lg p-1 border border-slate-200 dark:border-slate-700">
                     <button 
                        @click="setGroupBy('category')"
                        :class="['px-3 py-1.5 rounded-md transition-all text-xs font-medium', groupBy === 'category' ? 'bg-white dark:bg-slate-700 shadow-sm text-slate-900 dark:text-white' : 'text-slate-500 hover:text-slate-700 dark:hover:text-slate-300']"
                    >
                        Category
                    </button>
                    <button 
                        @click="setGroupBy('source')"
                        :class="['px-3 py-1.5 rounded-md transition-all text-xs font-medium', groupBy === 'source' ? 'bg-white dark:bg-slate-700 shadow-sm text-slate-900 dark:text-white' : 'text-slate-500 hover:text-slate-700 dark:hover:text-slate-300']"
                    >
                        Source
                    </button>
                    <button 
                        @click="setGroupBy('none')"
                        :class="['px-3 py-1.5 rounded-md transition-all text-xs font-medium', groupBy === 'none' ? 'bg-white dark:bg-slate-700 shadow-sm text-slate-900 dark:text-white' : 'text-slate-500 hover:text-slate-700 dark:hover:text-slate-300']"
                    >
                        None
                    </button>
                </div>
            </div>
        </template>
    </ViewHeader>

    <div class="flex-1 flex overflow-hidden">
        <!-- Sidebar Filters -->
        <aside class="w-64 border-r border-slate-200 dark:border-slate-700 bg-white/40 dark:bg-slate-900/40 backdrop-blur-md overflow-y-auto p-4 space-y-6 hidden md:block">
            <!-- Sources -->
            <div>
                <h3 class="text-xs font-bold text-slate-500 uppercase tracking-wider mb-3 flex items-center gap-2">
                    <FunnelIcon class="w-4 h-4" /> Sources
                </h3>
                <div class="space-y-2">
                    <div v-for="source in availableSources" :key="source" class="flex items-center group">
                        <input 
                            type="checkbox" 
                            :id="source" 
                            :checked="selectedSources.includes(source)"
                            @change="toggleSource(source)"
                            class="h-4 w-4 text-blue-600 border-slate-300 rounded focus:ring-blue-500 bg-white/50 cursor-pointer transition-colors"
                        >
                        <label :for="source" class="ml-2 text-sm text-slate-600 dark:text-slate-300 cursor-pointer select-none group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors">
                            {{ source }}
                        </label>
                    </div>
                </div>
            </div>

            <!-- Time -->
            <div>
                 <h3 class="text-xs font-bold text-slate-500 uppercase tracking-wider mb-3">Time Range</h3>
                 <div class="space-y-1">
                     <button 
                        v-for="opt in ['all', 'day', 'week', 'month', 'year']" 
                        :key="opt"
                        @click="setTimeFilter(opt)"
                        :class="['w-full text-left px-3 py-2 rounded-lg text-sm transition-all', timeFilter === opt ? 'bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-300 font-medium' : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100/50 dark:hover:bg-slate-800/50']"
                    >
                        {{ opt.charAt(0).toUpperCase() + opt.slice(1) }}
                    </button>
                 </div>
            </div>

            <!-- Sort -->
            <div>
                 <h3 class="text-xs font-bold text-slate-500 uppercase tracking-wider mb-3 flex items-center gap-2">
                    <AdjustmentsHorizontalIcon class="w-4 h-4" /> Sort By
                 </h3>
                 <div class="relative">
                     <select 
                        v-model="sortBy" 
                        @change="setSortBy(($event.target as HTMLSelectElement).value)"
                        class="w-full bg-white/50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-lg text-sm px-3 py-2 text-slate-700 dark:text-slate-200 focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 outline-none appearance-none"
                     >
                         <option value="relevance">Relevance</option>
                         <option value="date">Date</option>
                         <option value="rating">Rating</option>
                     </select>
                     <div class="absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none text-slate-400">
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" /></svg>
                     </div>
                 </div>
            </div>
        </aside>

        <!-- Results Area -->
        <main class="flex-1 overflow-y-auto p-6 scroll-smooth">
             <div v-if="loading && results.length === 0" class="space-y-6">
                <div v-for="i in 3" :key="i">
                    <div v-if="groupBy !== 'none'" class="h-6 w-32 bg-slate-200/50 dark:bg-slate-700/50 rounded mb-4 animate-pulse"></div>
                    <LoadingSkeleton />
                    <LoadingSkeleton />
                </div>
            </div>

            <div v-else-if="error" class="text-center py-10">
                <div class="inline-flex items-center justify-center w-16 h-16 rounded-full bg-red-50 dark:bg-red-900/20 mb-4 text-red-500">
                    <svg class="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
                </div>
                <p class="text-red-600 dark:text-red-400 font-medium">{{ error }}</p>
            </div>

            <div v-else-if="results.length === 0" class="text-center py-20">
                <div class="inline-flex items-center justify-center w-20 h-20 rounded-full bg-slate-100/50 dark:bg-slate-800/50 backdrop-blur-sm mb-6 border border-slate-200 dark:border-slate-700">
                     <FunnelIcon class="w-10 h-10 text-slate-400" />
                </div>
                <h3 class="text-xl font-bold text-slate-800 dark:text-white">No results found</h3>
                <p class="text-slate-500 mt-2 max-w-sm mx-auto">We couldn't find anything matching your search. Try adjusting your filters or using different keywords.</p>
            </div>

            <div v-else class="space-y-10">
                <div v-for="group in groupedResults" :key="group.title" class="animate-fade-in-up">
                    <h2 v-if="groupBy !== 'none'" class="text-lg font-bold text-slate-800 dark:text-white mb-5 flex items-center gap-3">
                        <span :class="['w-1.5 h-6 rounded-full bg-gradient-to-b', getBadgeColor(group.title).includes('blue') ? 'from-blue-500 to-indigo-600' : 'from-orange-500 to-red-600']"></span>
                        {{ group.title }}
                        <span class="text-xs font-medium px-2 py-0.5 rounded-full bg-slate-100 dark:bg-slate-800 text-slate-500 border border-slate-200 dark:border-slate-700">
                            {{ group.items.length }}
                        </span>
                    </h2>

                    <div class="grid grid-cols-1 gap-4">
                        <div 
                            v-for="item in group.items" 
                            :key="item.source + item.id" 
                            class="group relative bg-white/70 dark:bg-slate-800/70 backdrop-blur-sm rounded-xl border border-slate-200/60 dark:border-slate-700/60 p-5 hover:shadow-xl hover:shadow-blue-500/5 dark:hover:shadow-blue-900/10 transition-all duration-300 hover:-translate-y-0.5 hover:border-blue-300/50 dark:hover:border-blue-500/30 cursor-pointer"
                            @click="handleItemClick(item)"
                        >
                             <div class="flex justify-between items-start gap-5">
                                <div class="flex-1 min-w-0">
                                    <div class="flex items-center gap-2 mb-2">
                                        <!-- Source Badge -->
                                        <span :class="['text-[10px] uppercase font-bold px-2.5 py-1 rounded-lg border shadow-sm', getBadgeColor(item.source)]">
                                            {{ item.source }}
                                        </span>
                                        <!-- Category Badge -->
                                        <span v-if="groupBy !== 'category'" class="text-[10px] uppercase font-bold px-2 py-1 rounded-lg border border-slate-200 dark:border-slate-700 text-slate-500 bg-slate-50/50 dark:bg-slate-800/50">
                                            {{ item.category }}
                                        </span>
                                        
                                        <span v-if="item.created_at" class="text-xs font-medium text-slate-400 ml-auto sm:ml-0 flex items-center gap-1">
                                            <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                                            {{ formatDate(item.created_at) }}
                                        </span>
                                    </div>
                                    
                                    <div class="block group/link">
                                        <h3 class="text-lg font-bold text-slate-800 dark:text-white group-hover/link:text-blue-600 dark:group-hover/link:text-blue-400 transition-colors leading-tight truncate pr-8">
                                            {{ item.title }}
                                        </h3>
                                        <p v-if="item.description" class="mt-2 text-sm text-slate-600 dark:text-slate-300 line-clamp-2 leading-relaxed font-medium">
                                            {{ item.description }}
                                        </p>
                                    </div>

                                    <div class="flex flex-wrap items-center gap-x-5 gap-y-2 mt-4 text-xs font-medium text-slate-500 dark:text-slate-400">
                                        <div v-if="item.points !== undefined" class="flex items-center gap-1.5" title="Score/Stars">
                                            <StarIcon class="w-4 h-4 text-amber-400 fill-amber-400/20" />
                                            <span>{{ item.points }}</span>
                                        </div>
                                        <div v-if="item.comment_count !== undefined" class="flex items-center gap-1.5" title="Comments">
                                            <ChatBubbleLeftIcon class="w-4 h-4 text-blue-400" />
                                            <span>{{ item.comment_count }}</span>
                                        </div>
                                        <div v-if="item.author" class="flex items-center gap-1.5" title="Author">
                                            <UserIcon class="w-4 h-4 text-slate-400" />
                                            <span>{{ item.author }}</span>
                                        </div>
                                         <div v-if="item.tags && item.tags.length > 0" class="flex items-center gap-1 ml-auto">
                                            <span v-for="tag in item.tags.slice(0, 3)" :key="tag" class="px-2 py-1 bg-slate-100 dark:bg-slate-700/50 rounded-md text-slate-600 dark:text-slate-300 border border-slate-200 dark:border-slate-700/50">
                                                {{ tag }}
                                            </span>
                                        </div>
                                    </div>
                                </div>
                                
                                <button class="p-2 rounded-lg bg-slate-50 dark:bg-slate-800 text-slate-400 hover:text-blue-600 dark:hover:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/20 transition-all">
                                    <ArrowTopRightOnSquareIcon class="w-5 h-5" />
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
                
                <!-- Load More -->
                <div v-if="hasMore" class="flex justify-center pt-8 pb-10">
                    <button 
                        @click="loadMore" 
                        :disabled="loadingMore"
                        class="px-8 py-3 bg-white/50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-xl text-sm font-semibold text-slate-700 dark:text-slate-200 hover:bg-white dark:hover:bg-slate-800 hover:shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-3 backdrop-blur-sm"
                    >
                        <span v-if="loadingMore" class="animate-spin h-4 w-4 border-2 border-current border-t-transparent rounded-full"></span>
                        <span v-else>Load More Results</span>
                    </button>
                </div>
            </div>
        </main>
    </div>
    
    <!-- Drawers -->
    <Transition name="slide-over">
        <RepoDrawer 
            v-if="selectedRepo" 
            :repo="selectedRepo" 
            @close="selectedItem = null" 
        />
    </Transition>
    <Transition name="slide-over">
        <RedditDrawer 
            v-if="selectedRedditPost" 
            :post="selectedRedditPost" 
            @close="selectedItem = null" 
        />
    </Transition>
    <Transition name="slide-over">
        <HackerNewsDrawer 
            v-if="selectedHackerNewsStory" 
            :story="selectedHackerNewsStory" 
            @close="selectedItem = null" 
        />
    </Transition>
    <Transition name="slide-over">
        <StackOverflowDrawer 
            v-if="selectedStackOverflowQuestion" 
            :question="selectedStackOverflowQuestion" 
            @close="selectedItem = null" 
        />
    </Transition>
    <Transition name="slide-over">
        <LobstersDrawer 
            v-if="selectedLobstersStory" 
            :story="selectedLobstersStory" 
            @close="selectedItem = null" 
        />
    </Transition>
    <Transition name="slide-over">
        <DevtoDrawer 
            v-if="selectedDevtoArticle" 
            :article="selectedDevtoArticle" 
            @close="selectedItem = null" 
        />
    </Transition>
    <Transition name="slide-over">
        <CratesDrawer 
            v-if="selectedCrate" 
            :crate="selectedCrate" 
            @close="selectedItem = null" 
        />
    </Transition>
    <!-- Add other drawers if they exist -->
  </div>
</template>