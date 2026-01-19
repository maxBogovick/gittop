<template>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    <div 
        v-for="post in posts" 
        :key="post.id" 
        class="bg-white rounded-2xl shadow-sm border border-slate-200 overflow-hidden hover:shadow-md hover:scale-[1.02] transition-all duration-300 cursor-pointer flex flex-col group"
        @click="$emit('select', post)"
    >
      <div class="aspect-video w-full overflow-hidden bg-slate-100 relative">
         <img v-if="post.coverImage" :src="post.coverImage.url" class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110" loading="lazy" />
         <div v-else class="w-full h-full flex items-center justify-center text-slate-300">
            <svg class="h-12 w-12" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 00-2 2z" />
            </svg>
         </div>
      </div>
      <div class="p-5 flex-1 flex flex-col">
        <h3 class="text-lg font-bold text-slate-900 group-hover:text-blue-600 transition-colors line-clamp-2 leading-tight mb-2">
            {{ post.title }}
        </h3>
        <p class="text-sm text-slate-500 line-clamp-2 mb-4">
            {{ post.brief }}
        </p>
        <div class="mt-auto pt-4 flex items-center justify-between">
            <div class="flex items-center gap-2">
                <img v-if="post.author.profilePicture" :src="post.author.profilePicture" class="h-6 w-6 rounded-full" />
                <span class="text-xs font-medium text-slate-500">{{ post.author.name }}</span>
            </div>
            <div class="flex items-center gap-3 text-[10px] font-bold text-slate-400 uppercase tracking-wider">
                <span class="flex items-center gap-1">
                    <svg class="h-3 w-3" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M3.172 5.172a4 4 0 015.656 0L10 6.343l1.172-1.171a4 4 0 115.656 5.656L10 17.657l-6.828-6.829a4 4 0 010-5.656z" clip-rule="evenodd" /></svg>
                    {{ post.reactionCount }}
                </span>
                <span>{{ post.readTimeInMinutes }}m</span>
            </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type HashnodePost } from '../services/tauriApi';

defineProps<{
  posts: HashnodePost[];
}>();

defineEmits<{
  (e: 'select', post: HashnodePost): void;
}>();
</script>