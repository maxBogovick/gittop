<template>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    <div v-for="post in posts" :key="post.link" class="bg-white/70 dark:bg-slate-900/70 backdrop-blur-md rounded-2xl shadow-lg border border-slate-200/50 dark:border-slate-700/50 overflow-hidden hover:shadow-xl hover:scale-[1.02] transition-all duration-300 cursor-pointer flex flex-col group" @click="$emit('select', post)">
      <div class="p-6 flex-1 flex flex-col">
        <div class="flex items-center gap-2 mb-4">
            <span class="bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-300 text-[10px] font-black uppercase tracking-widest px-2 py-1 rounded">Interview</span>
            <span class="text-[10px] font-bold text-slate-400 dark:text-slate-500 uppercase ml-auto">{{ formatDate(post.pub_date) }}</span>
        </div>
        <h3 class="text-xl font-black text-slate-900 dark:text-white group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors leading-tight mb-3">
            {{ post.title }}
        </h3>
        <div class="text-sm text-slate-500 dark:text-slate-400 line-clamp-3 mb-6" v-html="post.content"></div>
        <div class="mt-auto pt-4 flex items-center border-t border-slate-200/50 dark:border-slate-700/50">
            <div class="h-8 w-8 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center text-xs font-bold text-slate-500 dark:text-slate-400 border border-slate-200 dark:border-slate-700">
                {{ post.author.charAt(0) }}
            </div>
            <span class="text-xs font-bold text-slate-600 dark:text-slate-300 ml-3">{{ post.author }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type IndieHackersPost } from '../services/tauriApi';
defineProps<{ posts: IndieHackersPost[] }>();
defineEmits<{ (e: 'select', post: IndieHackersPost): void }>();

function formatDate(dateStr: string): string {
  const date = new Date(dateStr);
  return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(date);
}
</script>
