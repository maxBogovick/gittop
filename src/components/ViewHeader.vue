<template>
  <div class="mb-8">
    <div class="flex flex-col md:flex-row md:items-end justify-between gap-6">
      <div class="space-y-1">
        <div class="flex items-center gap-3">
            <div v-if="icon" class="p-2 bg-white rounded-xl shadow-sm border border-slate-100 text-slate-600">
                <component :is="icon" class="w-6 h-6" />
            </div>
            <h1 class="text-3xl font-black text-slate-900 tracking-tight">{{ title }}</h1>
        </div>
        <p class="text-slate-500 font-medium ml-1">{{ subtitle }}</p>
      </div>

      <!-- Contextual Switcher (Top/New) -->
      <div v-if="hasToggle" class="flex bg-slate-200/50 p-1 rounded-xl border border-slate-200 shadow-inner">
        <router-link 
            :to="topPath" 
            active-class="bg-white text-slate-900 shadow-sm ring-1 ring-slate-200"
            class="px-6 py-2 rounded-lg text-xs font-black uppercase tracking-wider text-slate-500 hover:text-slate-700 transition-all"
        >
            Top
        </router-link>
        <router-link 
            :to="newPath" 
            active-class="bg-white text-slate-900 shadow-sm ring-1 ring-slate-200"
            class="px-6 py-2 rounded-lg text-xs font-black uppercase tracking-wider text-slate-500 hover:text-slate-700 transition-all"
        >
            New
        </router-link>
      </div>
    </div>
    
    <!-- Action Bar Slot (Filters, Search, etc) -->
    <div class="mt-6 flex flex-col sm:flex-row items-center gap-4">
        <slot name="actions"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  title: string;
  subtitle: string;
  icon?: any;
  hasToggle?: boolean;
  topPath?: string;
  newPath?: string;
}>(), {
  hasToggle: false,
  topPath: '',
  newPath: ''
});
</script>
