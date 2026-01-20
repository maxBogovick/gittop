<template>
  <div class="relative">
    <ViewHeader title="Crates.io" subtitle="The Rust community's crate registry">
      <template #actions>
          <div class="flex flex-col sm:flex-row items-center gap-4 w-full">
            <SearchBar v-model="store.query" placeholder="Search crates..." @update:model-value="store.loadCrates(false)" class="w-full sm:w-80" />
            <div class="flex items-center gap-2 bg-white dark:bg-slate-800 p-1.5 rounded-xl border border-slate-200 dark:border-slate-700 shadow-sm">
                <select v-model="store.sort" @change="store.loadCrates(false)" class="block pl-3 pr-10 py-2 text-sm bg-transparent text-slate-900 dark:text-white border-transparent focus:ring-0 rounded-lg transition-all cursor-pointer">
                    <option value="downloads" class="bg-white dark:bg-slate-800">Downloads</option>
                    <option value="recent-downloads" class="bg-white dark:bg-slate-800">Recent Downloads</option>
                    <option value="new" class="bg-white dark:bg-slate-800">Newest</option>
                    <option value="alpha" class="bg-white dark:bg-slate-800">Alphabetical</option>
                </select>
            </div>
          </div>
      </template>
    </ViewHeader>

    <LoadingSkeleton v-if="store.isLoading && !store.crates.length" />
    <div v-else class="mt-6">
      <CratesTable :crates="store.crates" @select="handleSelect" />
      <LoadMoreButton :loading="store.isLoading" :has-more="store.hasMore" @click="store.loadCrates(true)" />
    </div>

    <Transition name="slide-over">
      <CratesDrawer
        v-if="selectedCrate"
        :crate="selectedCrate"
        @close="selectedCrate = null"
      />
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useCratesStore } from '../stores/crates';
import ViewHeader from '../components/ViewHeader.vue';
import CratesTable from '../components/CratesTable.vue';
import CratesDrawer from '../components/CratesDrawer.vue';
import SearchBar from '../components/SearchBar.vue';
import LoadingSkeleton from '../components/LoadingSkeleton.vue';
import LoadMoreButton from '../components/LoadMoreButton.vue';
import { type Crate } from '../services/tauriApi';

const store = useCratesStore();
const selectedCrate = ref<Crate | null>(null);

function handleSelect(item: Crate) {
  selectedCrate.value = item;
}

onMounted(() => { if (!store.crates.length) store.loadCrates(); });
</script>