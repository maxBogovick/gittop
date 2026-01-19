<template>
  <div class="overflow-x-auto bg-white shadow rounded-lg">
    <table class="min-w-full divide-y divide-gray-200">
      <thead class="bg-gray-50">
        <tr>
          <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Article</th>
          <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Tags</th>
          <th scope="col" class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">Reactions</th>
          <th scope="col" class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">Comments</th>
          <th scope="col" class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">Reading Time</th>
          <th scope="col" class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">Published</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-gray-200">
        <tr 
          v-for="article in articles" 
          :key="article.id" 
          class="hover:bg-gray-50 transition-colors cursor-pointer group"
          @click="$emit('select', article)"
        >
          <td class="px-6 py-4">
            <div class="flex items-center">
              <div class="flex-shrink-0 h-10 w-10">
                <img v-if="article.user.profile_image" class="h-10 w-10 rounded-full object-cover border border-gray-200" :src="article.user.profile_image" alt="" />
                <div v-else class="h-10 w-10 rounded-full bg-gray-200 flex items-center justify-center text-gray-500 font-bold">
                  {{ article.user.username.charAt(0).toUpperCase() }}
                </div>
              </div>
              <div class="ml-4">
                <div class="text-sm font-medium text-blue-600 group-hover:underline line-clamp-2" :title="article.title">
                  {{ article.title }}
                </div>
                <div class="text-sm text-gray-500">
                  by {{ article.user.name }}
                </div>
              </div>
            </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap">
             <div class="flex gap-1 flex-wrap max-w-xs">
                <span v-for="tag in article.tag_list" :key="tag" class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-gray-100 text-gray-600">
                  #{{ tag }}
                </span>
             </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-600 text-right font-mono">
            ❤️ {{ formatNumber(article.public_reactions_count) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-600 text-right font-mono">
            💬 {{ formatNumber(article.comments_count) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-600 text-right">
            {{ article.reading_time_minutes }} min
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 text-right">
            {{ formatDate(article.published_at) }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { type DevtoArticle } from '../services/tauriApi';

defineProps<{
  articles: DevtoArticle[];
}>();

defineEmits<{
  (e: 'select', article: DevtoArticle): void;
}>();

function formatNumber(num: number): string {
  return new Intl.NumberFormat('en-US', { notation: "compact", compactDisplay: "short" }).format(num);
}

function formatDate(dateStr: string): string {
  if (!dateStr) return '';
  const date = new Date(dateStr);
  return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(date);
}
</script>
