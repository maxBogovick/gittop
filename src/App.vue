<template>
  <div class="min-h-screen bg-slate-50 flex flex-col font-sans text-slate-900">
    <!-- Main Header -->
    <header class="sticky top-0 z-50 bg-white/80 backdrop-blur-md border-b border-slate-200/60 shadow-sm">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <!-- Logo & Primary Nav -->
          <div class="flex items-center gap-8 overflow-hidden">
            <div class="flex-shrink-0 flex items-center group cursor-pointer" @click="$router.push('/')">
              <div class="w-8 h-8 bg-gradient-to-tr from-blue-600 to-indigo-600 rounded-lg flex items-center justify-center mr-3 shadow-blue-200 shadow-lg group-hover:scale-110 transition-transform">
                <svg class="w-5 h-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
              </div>
              <span class="font-black text-xl tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-slate-900 to-slate-600">GitTop</span>
            </div>

            <nav class="hidden lg:flex items-center space-x-1 overflow-x-auto no-scrollbar py-1">
              <router-link 
                v-for="item in navItems" 
                :key="item.path" 
                :to="item.path"
                :class="[
                  'flex items-center gap-2 px-3 py-2 rounded-xl text-sm font-bold transition-all duration-200 whitespace-nowrap',
                  $route.path.startsWith(item.basePath) 
                    ? 'bg-slate-900 text-white shadow-md shadow-slate-200' 
                    : 'text-slate-500 hover:bg-slate-100 hover:text-slate-900'
                ]"
              >
                <component :is="item.icon" class="w-4 h-4" />
                {{ item.name }}
              </router-link>
            </nav>
          </div>

          <!-- Secondary Actions -->
          <div class="flex items-center gap-3">
            <button 
              @click="handleRefresh" 
              :disabled="isRefreshing"
              class="p-2 text-slate-400 hover:text-blue-600 hover:bg-blue-50 rounded-xl transition-all active:scale-95 disabled:opacity-50"
              title="Refresh Cache"
            >
              <svg :class="['w-5 h-5', isRefreshing ? 'animate-spin' : '']" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.001 0 01-15.357-2m15.357 2H15" /></svg>
            </button>
            <div class="w-px h-6 bg-slate-200 mx-1"></div>
            <button class="w-9 h-9 rounded-xl bg-slate-100 border border-slate-200 flex items-center justify-center text-slate-500 hover:bg-slate-200 transition-colors">
              <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /></svg>
            </button>
          </div>
        </div>
      </div>
    </header>

    <!-- Content -->
    <main class="flex-grow max-w-7xl w-full mx-auto px-4 sm:px-6 lg:px-8 py-8 animate-fade-in-up">
      <router-view v-slot="{ Component }">
        <transition name="page" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, markRaw } from 'vue';
import { refreshCache } from './services/tauriApi';

const isRefreshing = ref(false);

const IconGH = { template: '<svg fill="currentColor" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd"/></svg>' };
const IconReddit = { template: '<svg fill="currentColor" viewBox="0 0 24 24"><path d="M12 0A12 12 0 0 0 0 12a12 12 0 0 0 12 12 12 12 0 0 0 12-12A12 12 0 0 0 12 0zm5.01 4.744c.688 0 1.25.561 1.25 1.249a1.25 1.25 0 0 1-2.498.056l-2.597-.547-.8 3.747c1.824.07 3.48.632 4.674 1.488.308-.309.73-.491 1.207-.491.968 0 1.754.786 1.754 1.754 0 .716-.435 1.333-1.01 1.614a3.111 3.111 0 0 1 .042.52c0 2.694-3.13 4.87-7.004 4.87-3.874 0-7.004-2.176-7.004-4.87 0-.183.015-.366.043-.534A1.748 1.748 0 0 1 4.028 12c0-.968.786-1.754 1.754-1.754.463 0 .875.182 1.185.476 1.162-.806 2.753-1.34 4.496-1.435l.693-3.227c.01-.043.015-.086.015-.126l3.607.775c.056-.616.57-.1.57-.1zm-10.277 9.187c-.612 0-1.108.497-1.108 1.108 0 .612.496 1.108 1.108 1.108.613 0 1.108-.496 1.108-1.108 0-.611-.495-1.108-1.108-1.108zm4.398 2.33a5.163 5.163 0 0 1-2.251-.538.44.44 0 0 1-.19-.563.44.44 0 0 1 .574-.19c.814.355 1.81.355 2.623 0a.44.44 0 0 1 .574.19.44.44 0 0 1-.19.563 5.163 5.163 0 0 1-1.14.538zm2.743-2.33c-.613 0-1.108.497-1.108 1.108 0 .612.495 1.108 1.108 1.108s1.108-.496 1.108-1.108c0-.611-.496-1.108-1.108-1.108z"/></svg>' };
const IconSO = { template: '<svg fill="currentColor" viewBox="0 0 24 24"><path d="M18.986 21.867v-6.404h2.155v8.551H.887v-8.551h2.155v6.404h15.944zM6.567 14.842l10.033 2.085.432-2.077-10.033-2.087-.432 2.079zm1.583-4.981l9.908 4.456.812-1.872-9.909-4.456-.811 1.872zm3.273-4.828l8.152 7.305 1.435-1.602-8.152-7.305-1.435 1.602zm5.729-3.201l-1.73 1.269 6.21 8.463 1.73-1.269-6.21-8.463zM14.775 0l-1.95 1.015 4.082 7.885 1.95-1.015L14.775 0z"/></svg>' };
const IconArticle = { template: '<svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10l4 4v10a2 2 0 01-2 2z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 4v4h4" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 8h1M7 12h10M7 16h10" /></svg>' };
const IconHN = { template: '<svg fill="currentColor" viewBox="0 0 448 512"><path d="M0 32v448h448V32H0zm212.2 202.1L150 100h24.7l40.5 81.2c7.3 15.1 13.9 29.8 19.7 44h1.1c6.4-14.9 13.1-29.8 20.9-44.4L299.5 100h23.8l-63.9 134.1V380h-47.2V234.1z"/></svg>' };
const IconPH = { template: '<svg fill="currentColor" viewBox="0 0 24 24"><path d="M13.563 8.906H10.5v3.406h3.063c.938 0 1.563-.5 1.563-1.719s-.625-1.687-1.563-1.687zM12 0C5.375 0 0 5.375 0 12s5.375 12 12 12 12-5.375 12-12S18.625 0 12 0zm3.125 12.313H10.5v3.437H8.437V6.844h5.125c2.313 0 3.656 1.344 3.656 3.125 0 1.781-1.343 2.344-2.093 2.344z"/></svg>' };
const IconLobsters = { template: '<svg fill="currentColor" viewBox="0 0 24 24"><path d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm0 22C6.486 22 2 17.514 2 12S6.486 2 12 2s10 4.486 10 10-4.486 10-10 10zm-1-15h2v6h-2zm0 8h2v2h-2z"/></svg>' };
const IconCrates = { template: '<svg fill="currentColor" viewBox="0 0 24 24"><path d="M12 0L1.5 6v12L12 24l10.5-6V6L12 0zm9 17.15L12 22.3l-9-5.15V6.85L12 1.7l9 5.15v10.3zM12 5.2L5.5 8.9v6.2l6.5 3.7 6.5-3.7V8.9L12 5.2zm5 9.2l-5 2.8-5-2.8V9.6l5-2.8 5 2.8v4.8z"/></svg>' };
const IconIndie = { template: '<svg fill="currentColor" viewBox="0 0 24 24"><path d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm5 17h-2v-5h-2v5H9V7h2v5h2V7h2v10z"/></svg>' };

const navItems = [
  { name: 'GitHub', path: '/top', basePath: '/top', icon: markRaw(IconGH) },
  { name: 'Reddit', path: '/reddit/top', basePath: '/reddit', icon: markRaw(IconReddit) },
  { name: 'StackOverflow', path: '/stackoverflow', basePath: '/stackoverflow', icon: markRaw(IconSO) },
  { name: 'PH', path: '/producthunt', basePath: '/producthunt', icon: markRaw(IconPH) },
  { name: 'Crates', path: '/crates', basePath: '/crates', icon: markRaw(IconCrates) },
  { name: 'Lobsters', path: '/lobsters', basePath: '/lobsters', icon: markRaw(IconLobsters) },
  { name: 'Dev.to', path: '/devto/top', basePath: '/devto', icon: markRaw(IconArticle) },
  { name: 'HN', path: '/hackernews', basePath: '/hackernews', icon: markRaw(IconHN) },
  { name: 'Medium', path: '/medium', basePath: '/medium', icon: markRaw(IconArticle) },
  { name: 'Indie', path: '/indiehackers', basePath: '/indiehackers', icon: markRaw(IconIndie) },
  { name: 'Hashnode', path: '/hashnode', basePath: '/hashnode', icon: markRaw(IconArticle) },
];

async function handleRefresh() {
  isRefreshing.value = true;
  try {
    await refreshCache();
  } finally {
    setTimeout(() => isRefreshing.value = false, 1000);
  }
}
</script>

<style>
.no-scrollbar::-webkit-scrollbar {
  display: none;
}
.no-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

/* Page transitions */
.page-enter-active,
.page-leave-active {
  transition: opacity 0.2s, transform 0.2s;
}
.page-enter-from {
  opacity: 0;
  transform: translateY(10px);
}
.page-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}
</style>