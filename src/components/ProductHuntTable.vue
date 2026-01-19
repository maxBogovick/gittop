<template>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    <div 
        v-for="post in posts" 
        :key="post.id" 
        class="bg-white rounded-2xl shadow-sm border border-slate-200 overflow-hidden hover:shadow-md hover:scale-[1.02] transition-all duration-300 cursor-pointer flex flex-col group"
        @click="$emit('select', post)"
    >
      <div class="aspect-video w-full overflow-hidden bg-slate-100 relative">
         <img v-if="post.thumbnail" :src="post.thumbnail.url" class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110" loading="lazy" />
         <div v-else class="w-full h-full flex items-center justify-center text-slate-300 font-bold text-4xl">
            {{ post.name.charAt(0) }}
         </div>
      </div>
      <div class="p-5 flex-1 flex flex-col">
        <h3 class="text-lg font-bold text-slate-900 group-hover:text-blue-600 transition-colors line-clamp-1 leading-tight mb-1">
            {{ post.name }}
        </h3>
        <p class="text-sm text-slate-500 line-clamp-2 mb-4 flex-1">
            {{ post.tagline }}
        </p>
        <div class="mt-auto pt-4 flex items-center justify-between border-t border-slate-50">
            <div class="flex items-center gap-3">
                <div class="flex items-center gap-1 text-slate-600 font-bold text-sm bg-slate-50 px-2 py-1 rounded-lg">
                    ▲ {{ post.votesCount }}
                </div>
                <div class="flex items-center gap-1 text-slate-400 text-sm">
                    💬 {{ post.commentsCount }}
                </div>
            </div>
            <span class="text-[10px] font-bold text-slate-400 uppercase tracking-widest">{{ formatDate(post.createdAt) }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type ProductHuntPost } from '../services/tauriApi';

defineProps<{ posts: ProductHuntPost[] }>();
defineEmits<{ (e: 'select', post: ProductHuntPost): void }>();

function formatDate(dateStr: string): string {
  const date = new Date(dateStr);
  return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(date);
}
</script>