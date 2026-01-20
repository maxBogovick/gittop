<template>
  <aside
    :class="[
      'hidden lg:flex flex-col fixed left-0 top-0 h-screen border-r border-slate-200/50 dark:border-slate-700/50 bg-white/70 dark:bg-slate-900/70 backdrop-blur-xl transition-all duration-300 z-40',
      collapsed ? 'w-[var(--sidebar-collapsed-width)]' : 'w-[var(--sidebar-width)]'
    ]"
  >
    <!-- Logo -->
    <div class="flex-shrink-0 h-16 flex items-center px-4 border-b border-slate-200/50 dark:border-slate-700/50">
      <div
        class="flex items-center gap-3 cursor-pointer group"
        @click="$router.push('/')"
      >
        <div class="w-9 h-9 bg-gradient-to-tr from-blue-600 to-indigo-600 rounded-xl flex items-center justify-center shadow-lg shadow-blue-500/20 group-hover:scale-105 transition-transform">
          <svg class="w-5 h-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M13 10V3L4 14h7v7l9-11h-7z" />
          </svg>
        </div>
        <span
          v-if="!collapsed"
          class="font-black text-lg tracking-tight text-[var(--color-text-primary)]"
        >
          GitTop
        </span>
      </div>
    </div>

    <!-- Navigation -->
    <nav class="flex-1 overflow-y-auto py-4 px-3 custom-scrollbar">
      <template v-for="(group, idx) in navGroups" :key="group.label">
        <div :class="idx > 0 ? 'mt-6' : ''">
          <h3
            v-if="!collapsed"
            class="px-3 mb-2 text-xs font-semibold uppercase tracking-wider text-[var(--color-text-muted)]"
          >
            {{ group.label }}
          </h3>
          <div v-else class="h-px bg-[var(--color-border-primary)] mx-2 mb-3"></div>

          <router-link
            v-for="item in group.items"
            :key="item.path"
            :to="item.path"
            :title="collapsed ? item.name : undefined"
            :class="[
              'flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-all duration-150 group mb-1',
              isActive(item.basePath)
                ? 'bg-[var(--color-accent-primary)] text-white shadow-md shadow-blue-500/20'
                : 'text-[var(--color-text-secondary)] hover:bg-[var(--color-bg-hover)] hover:text-[var(--color-text-primary)]',
              collapsed ? 'justify-center' : ''
            ]"
          >
            <component :is="item.icon" class="w-5 h-5 flex-shrink-0" />
            <span v-if="!collapsed" class="truncate">{{ item.name }}</span>
          </router-link>
        </div>
      </template>
    </nav>

    <!-- Collapse Toggle -->
    <div class="flex-shrink-0 p-3 border-t border-[var(--color-border-primary)]">
      <button
        @click="$emit('toggle')"
        :class="[
          'w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors',
          'text-[var(--color-text-secondary)] hover:bg-[var(--color-bg-hover)] hover:text-[var(--color-text-primary)]',
          collapsed ? 'justify-center' : ''
        ]"
        :title="collapsed ? 'Expand sidebar' : 'Collapse sidebar'"
      >
        <svg
          :class="['w-5 h-5 transition-transform', collapsed ? 'rotate-180' : '']"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 19l-7-7 7-7m8 14l-7-7 7-7" />
        </svg>
        <span v-if="!collapsed">Collapse</span>
      </button>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { markRaw } from 'vue';
import { useRoute } from 'vue-router';

defineProps<{
  collapsed: boolean;
}>();

defineEmits<{
  (e: 'toggle'): void;
}>();

const route = useRoute();

function isActive(basePath: string): boolean {
  return route.path === basePath || route.path.startsWith(basePath + '/');
}

// Icons as components
const IconGH = { template: '<svg fill="currentColor" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd"/></svg>' };
const IconCrates = { template: '<svg fill="currentColor" viewBox="0 0 24 24"><path d="M12 0L1.5 6v12L12 24l10.5-6V6L12 0zm9 17.15L12 22.3l-9-5.15V6.85L12 1.7l9 5.15v10.3zM12 5.2L5.5 8.9v6.2l6.5 3.7 6.5-3.7V8.9L12 5.2zm5 9.2l-5 2.8-5-2.8V9.6l5-2.8 5 2.8v4.8z"/></svg>' };
const IconReddit = { template: '<svg fill="currentColor" viewBox="0 0 24 24"><path d="M12 0A12 12 0 0 0 0 12a12 12 0 0 0 12 12 12 12 0 0 0 12-12A12 12 0 0 0 12 0zm5.01 4.744c.688 0 1.25.561 1.25 1.249a1.25 1.25 0 0 1-2.498.056l-2.597-.547-.8 3.747c1.824.07 3.48.632 4.674 1.488.308-.309.73-.491 1.207-.491.968 0 1.754.786 1.754 1.754 0 .716-.435 1.333-1.01 1.614a3.111 3.111 0 0 1 .042.52c0 2.694-3.13 4.87-7.004 4.87-3.874 0-7.004-2.176-7.004-4.87 0-.183.015-.366.043-.534A1.748 1.748 0 0 1 4.028 12c0-.968.786-1.754 1.754-1.754.463 0 .875.182 1.185.476 1.162-.806 2.753-1.34 4.496-1.435l.693-3.227c.01-.043.015-.086.015-.126l3.607.775c.056-.616.57-.1.57-.1zm-10.277 9.187c-.612 0-1.108.497-1.108 1.108 0 .612.496 1.108 1.108 1.108.613 0 1.108-.496 1.108-1.108 0-.611-.495-1.108-1.108-1.108zm4.398 2.33a5.163 5.163 0 0 1-2.251-.538.44.44 0 0 1-.19-.563.44.44 0 0 1 .574-.19c.814.355 1.81.355 2.623 0a.44.44 0 0 1 .574.19.44.44 0 0 1-.19.563 5.163 5.163 0 0 1-1.14.538zm2.743-2.33c-.613 0-1.108.497-1.108 1.108 0 .612.495 1.108 1.108 1.108s1.108-.496 1.108-1.108c0-.611-.496-1.108-1.108-1.108z"/></svg>' };
const IconHN = { template: '<svg fill="currentColor" viewBox="0 0 448 512"><path d="M0 32v448h448V32H0zm212.2 202.1L150 100h24.7l40.5 81.2c7.3 15.1 13.9 29.8 19.7 44h1.1c6.4-14.9 13.1-29.8 20.9-44.4L299.5 100h23.8l-63.9 134.1V380h-47.2V234.1z"/></svg>' };
const IconLobsters = { template: '<svg fill="currentColor" viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>' };
const IconArticle = { template: '<svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10l4 4v10a2 2 0 01-2 2z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 4v4h4" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 8h1M7 12h10M7 16h10" /></svg>' };
const IconPH = { template: '<svg fill="currentColor" viewBox="0 0 24 24"><path d="M13.563 8.906H10.5v3.406h3.063c.938 0 1.563-.5 1.563-1.719s-.625-1.687-1.563-1.687zM12 0C5.375 0 0 5.375 0 12s5.375 12 12 12 12-5.375 12-12S18.625 0 12 0zm3.125 12.313H10.5v3.437H8.437V6.844h5.125c2.313 0 3.656 1.344 3.656 3.125 0 1.781-1.343 2.344-2.093 2.344z"/></svg>' };
const IconIndie = { template: '<svg fill="currentColor" viewBox="0 0 24 24"><path d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm5 17h-2v-5h-2v5H9V7h2v5h2V7h2v10z"/></svg>' };
const IconEtsy = { template: '<svg fill="currentColor" viewBox="0 0 24 24"><path d="M8.559 3.386c0-.432.248-.756.724-.756h7.104c.248 0 .393.144.393.372v1.788c0 .204-.145.372-.393.372-.248 0-.681-.048-1.165-.048h-4.179c-.288 0-.433.144-.433.432v4.308c0 .288.145.432.433.432h3.478c.248 0 .393.144.393.372v1.788c0 .204-.145.372-.393.372h-3.478c-.288 0-.433.144-.433.432v4.884c0 .288.145.432.433.432h4.755c.484 0 .917-.048 1.165-.048.248 0 .393.168.393.372v1.788c0 .228-.145.372-.393.372H9.283c-.476 0-.724-.324-.724-.756V3.386zM12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0z"/></svg>' };

const navGroups = [
  {
    label: 'Code',
    items: [
      { name: 'GitHub', path: '/top', basePath: '/top', icon: markRaw(IconGH) },
      { name: 'Crates.io', path: '/crates', basePath: '/crates', icon: markRaw(IconCrates) },
    ]
  },
  {
    label: 'Community',
    items: [
      { name: 'Reddit', path: '/reddit/top', basePath: '/reddit', icon: markRaw(IconReddit) },
      { name: 'Hacker News', path: '/hackernews', basePath: '/hackernews', icon: markRaw(IconHN) },
      { name: 'Lobsters', path: '/lobsters', basePath: '/lobsters', icon: markRaw(IconLobsters) },
      { name: 'StackOverflow', path: '/stackoverflow', basePath: '/stackoverflow', icon: markRaw({ template: '<svg fill="currentColor" viewBox="0 0 24 24"><path d="M18.986 21.867v-6.404h2.155v8.551H.887v-8.551h2.155v6.404h15.944zM6.567 14.842l10.033 2.085.432-2.077-10.033-2.087-.432 2.079zm1.583-4.981l9.908 4.456.812-1.872-9.909-4.456-.811 1.872zm3.273-4.828l8.152 7.305 1.435-1.602-8.152-7.305-1.435 1.602zm5.729-3.201l-1.73 1.269 6.21 8.463 1.73-1.269-6.21-8.463z"/></svg>' }) },
    ]
  },
  {
    label: 'Content',
    items: [
      { name: 'Dev.to', path: '/devto/top', basePath: '/devto', icon: markRaw(IconArticle) },
      { name: 'Medium', path: '/medium', basePath: '/medium', icon: markRaw(IconArticle) },
      { name: 'Hashnode', path: '/hashnode', basePath: '/hashnode', icon: markRaw(IconArticle) },
    ]
  },
  {
    label: 'Products',
    items: [
      { name: 'Product Hunt', path: '/producthunt', basePath: '/producthunt', icon: markRaw(IconPH) },
      { name: 'Indie Hackers', path: '/indiehackers', basePath: '/indiehackers', icon: markRaw(IconIndie) },
      { name: 'Etsy', path: '/etsy/top', basePath: '/etsy', icon: markRaw(IconEtsy) },
    ]
  },
];
</script>
