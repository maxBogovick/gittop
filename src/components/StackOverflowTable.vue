<template>
  <div class="overflow-hidden bg-white/70 dark:bg-slate-900/70 backdrop-blur-md shadow-lg border border-slate-200/50 dark:border-slate-700/50 rounded-2xl">
    <table class="min-w-full divide-y divide-slate-200/50 dark:divide-slate-700/50">
      <thead class="bg-slate-50/50 dark:bg-slate-800/50">
        <tr>
          <th scope="col" class="px-6 py-4 text-left text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Question</th>
          <th scope="col" class="px-6 py-4 text-left text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Tags</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Score</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Answers</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Views</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider">Created</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-200/50 dark:divide-slate-700/50">
        <tr
          v-for="question in questions"
          :key="question.question_id"
          class="hover:bg-white/50 dark:hover:bg-slate-800/50 transition-all duration-150 cursor-pointer group"
          @click="$emit('select', question)"
        >
          <td class="px-6 py-4">
            <div class="flex items-start">
              <div class="flex-shrink-0 h-10 w-10">
                <img v-if="question.owner.profile_image" class="h-10 w-10 rounded-full object-cover ring-2 ring-white dark:ring-slate-800 shadow-sm" :src="question.owner.profile_image" alt="" />
                <div v-else class="h-10 w-10 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center text-slate-500 dark:text-slate-400 font-bold ring-2 ring-white dark:ring-slate-800 shadow-sm">
                  {{ (question.owner.display_name || '?').charAt(0).toUpperCase() }}
                </div>
              </div>
              <div class="ml-4 max-w-md">
                <div class="text-sm font-bold text-slate-900 dark:text-white group-hover:text-orange-600 dark:group-hover:text-orange-400 transition-colors line-clamp-2" v-html="question.title"></div>
                <div class="text-xs text-slate-500 dark:text-slate-400 mt-1">
                  by {{ question.owner.display_name }}
                </div>
              </div>
            </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap">
             <div class="flex gap-1 flex-wrap max-w-xs">
                <span v-for="tag in question.tags.slice(0, 3)" :key="tag" class="inline-flex items-center px-2 py-0.5 rounded-md text-[10px] font-medium bg-orange-50 dark:bg-orange-900/20 text-orange-700 dark:text-orange-300 ring-1 ring-inset ring-orange-600/10 dark:ring-orange-400/20">
                  {{ tag }}
                </span>
             </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-700 dark:text-slate-300 text-right font-medium">
            {{ formatNumber(question.score) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-right">
             <span :class="[
                'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border',
                question.answer_count > 0 ? (question.is_answered ? 'bg-green-50 dark:bg-green-900/30 text-green-700 dark:text-green-300 border-green-200 dark:border-green-800' : 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 border-slate-200 dark:border-slate-700') : 'text-slate-400 dark:text-slate-500 border-transparent'
             ]">
                {{ question.answer_count }} answers
             </span>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500 dark:text-slate-400 text-right font-mono">
            {{ formatNumber(question.view_count) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500 dark:text-slate-400 text-right">
            {{ formatDate(question.creation_date) }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { type StackOverflowQuestion } from '../services/tauriApi';

defineProps<{
  questions: StackOverflowQuestion[];
}>();

defineEmits<{
  (e: 'select', question: StackOverflowQuestion): void;
}>();

function formatNumber(num: number): string {
  return new Intl.NumberFormat('en-US', { notation: "compact", compactDisplay: "short" }).format(num);
}

function formatDate(timestamp: number): string {
  if (!timestamp) return '';
  const date = new Date(timestamp * 1000);
  return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(date);
}
</script>
