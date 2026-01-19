<template>
  <div class="relative">
    <ViewHeader
      title="Medium"
      subtitle="Deep dives and personal stories from across the design and tech world"
    >
      <template #actions>
        <div class="flex flex-col lg:flex-row items-center gap-4 w-full">
          <SearchBar
              v-model="store.tag"
              placeholder="Enter tag (e.g. rust, design, startup)..."
              @update:model-value="reload"
              class="w-full lg:w-72"
          />
          <div class="flex flex-wrap gap-2">
            <button
                v-for="tag in popularTags"
                :key="tag.value"
                @click="store.tag = tag.value; reload()"
                :class="[
                    'px-3 py-1.5 text-xs font-semibold rounded-full transition-all border',
                    store.tag === tag.value
                        ? 'bg-green-600 text-white border-green-600'
                        : 'bg-white text-slate-600 border-slate-200 hover:border-green-400 hover:text-green-600'
                ]"
            >
                {{ tag.label }}
            </button>
          </div>
        </div>
      </template>
    </ViewHeader>
    
    <div v-if="store.error" class="bg-red-50 border border-red-100 p-4 rounded-xl shadow-sm mb-6 flex items-center gap-3">
        <div class="flex-1 text-sm font-semibold text-red-800">{{ store.error }}</div>
        <button @click="reload" class="text-sm font-bold text-red-600 hover:text-red-700 underline transition-colors">Retry</button>
    </div>

    <LoadingSkeleton v-if="store.isLoading && !store.articles.length" />
    
    <div v-else-if="store.articles.length === 0 && !store.isLoading" class="flex flex-col items-center justify-center py-20 text-slate-400">
      <p class="font-medium">No articles found.</p>
    </div>
    
    <div v-else class="space-y-6">
      <MediumTable 
        :articles="store.articles" 
        @select="handleSelect" 
      />
    </div>

    <Transition name="slide-over">
        <MediumDrawer 
            v-if="selectedArticle" 
            :article="selectedArticle" 
            @close="selectedArticle = null" 
        />
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useMediumStore } from '../stores/medium';
import ViewHeader from '../components/ViewHeader.vue';
import MediumTable from '../components/MediumTable.vue';
import MediumDrawer from '../components/MediumDrawer.vue';
import LoadingSkeleton from '../components/LoadingSkeleton.vue';
import SearchBar from '../components/SearchBar.vue';
import { type MediumArticle } from '../services/tauriApi';

const store = useMediumStore();
const selectedArticle = ref<MediumArticle | null>(null);

const popularTags = [
  { value: 'technology', label: 'Technology' },
  { value: 'programming', label: 'Programming' },
  { value: 'javascript', label: 'JavaScript' },
  { value: 'python', label: 'Python' },
  { value: 'artificial-intelligence', label: 'AI' },
  { value: 'startup', label: 'Startup' },
  { value: 'design', label: 'Design' },
  { value: 'productivity', label: 'Productivity' },
];

function reload() {
  store.loadArticles();
}

function handleSelect(article: MediumArticle) {
  selectedArticle.value = article;
}

onMounted(() => {
  if (store.articles.length === 0) {
      reload();
  }
});
</script>