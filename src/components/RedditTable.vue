<template>
  <div class="overflow-x-auto bg-white shadow rounded-lg">
    <table class="min-w-full divide-y divide-gray-200">
      <thead class="bg-gray-50">
        <tr>
          <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Post</th>
          <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Subreddit</th>
          <th 
            scope="col" 
            class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:text-gray-700"
            @click="$emit('sort', 'score')"
          >
            Score
            <span v-if="currentSort === 'score'" class="ml-1">▼</span>
          </th>
          <th 
            scope="col" 
            class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:text-gray-700"
            @click="$emit('sort', 'comments')"
          >
            Comments
            <span v-if="currentSort === 'comments'" class="ml-1">▼</span>
          </th>
          <th 
            scope="col" 
            class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:text-gray-700"
            @click="$emit('sort', 'date')"
          >
            Posted
            <span v-if="currentSort === 'date'" class="ml-1">▼</span>
          </th>
        </tr>
      </thead>
      <tbody class="divide-y divide-gray-200">
        <tr 
          v-for="post in posts" 
          :key="post.id" 
          class="hover:bg-gray-50 transition-colors cursor-pointer group"
          @click="$emit('select', post)"
        >
          <td class="px-6 py-4">
            <div class="flex items-start">
              <div class="flex-shrink-0 h-12 w-12 mr-4" v-if="post.thumbnail && post.thumbnail !== 'self' && post.thumbnail !== 'default'">
                 <img :src="decodeUrl(post.thumbnail)" referrerpolicy="no-referrer" class="h-12 w-12 rounded object-cover border border-gray-200" @error="($event.target as HTMLImageElement).style.display='none'" />
              </div>
              <div>
                <div class="text-sm font-medium text-blue-600 group-hover:underline line-clamp-2">
                  {{ post.title }}
                </div>
                <div class="text-xs text-gray-500 mt-1">
                  by u/{{ post.author }}
                </div>
              </div>
            </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap">
             <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-orange-100 text-orange-800">
               r/{{ post.subreddit }}
             </span>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-600 text-right font-mono">
            {{ formatNumber(post.score) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-600 text-right font-mono">
            {{ formatNumber(post.num_comments) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 text-right">
            {{ formatDate(post.created_utc) }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import type { RedditPost } from '../services/tauriApi';

defineProps<{
  posts: RedditPost[];
  currentSort?: string;
}>();

defineEmits<{
  (e: 'select', post: RedditPost): void;
  (e: 'sort', column: string): void;
}>();

function formatNumber(num: number): string {
  return new Intl.NumberFormat('en-US', { notation: "compact", compactDisplay: "short" }).format(num);
}

function formatDate(dateStr: string): string {
  if (!dateStr) return '';
  const date = new Date(dateStr);
  // Simple relative time could be better, but consistent with RepoTable for now
  return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(date);
}

function decodeUrl(url: string): string {
  return url.replace(/&amp;/g, '&');
}
</script>
