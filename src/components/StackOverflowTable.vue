<template>
  <div class="overflow-hidden bg-white shadow-sm border border-slate-200 rounded-xl">
    <table class="min-w-full divide-y divide-slate-200">
      <thead class="bg-slate-50/80">
        <tr>
          <th scope="col" class="px-6 py-4 text-left text-xs font-semibold text-slate-500 uppercase tracking-wider">Question</th>
          <th scope="col" class="px-6 py-4 text-left text-xs font-semibold text-slate-500 uppercase tracking-wider">Tags</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-semibold text-slate-500 uppercase tracking-wider">Score</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-semibold text-slate-500 uppercase tracking-wider">Answers</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-semibold text-slate-500 uppercase tracking-wider">Views</th>
          <th scope="col" class="px-6 py-4 text-right text-xs font-semibold text-slate-500 uppercase tracking-wider">Created</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-100 bg-white">
        <tr 
          v-for="question in questions" 
          :key="question.question_id" 
          class="hover:bg-slate-50/80 transition-all duration-150 cursor-pointer group"
          @click="$emit('select', question)"
        >
          <td class="px-6 py-4">
            <div class="flex items-start">
              <div class="flex-shrink-0 h-10 w-10">
                <img v-if="question.owner.profile_image" class="h-10 w-10 rounded-full object-cover ring-2 ring-white shadow-sm" :src="question.owner.profile_image" alt="" />
                <div v-else class="h-10 w-10 rounded-full bg-slate-100 flex items-center justify-center text-slate-400 font-bold ring-2 ring-white shadow-sm">
                  {{ (question.owner.display_name || '?').charAt(0).toUpperCase() }}
                </div>
              </div>
              <div class="ml-4 max-w-md">
                <div class="text-sm font-semibold text-slate-900 group-hover:text-blue-600 transition-colors line-clamp-2" v-html="question.title"></div>
                <div class="text-xs text-slate-500 mt-1">
                  by {{ question.owner.display_name }}
                </div>
              </div>
            </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap">
             <div class="flex gap-1 flex-wrap max-w-xs">
                <span v-for="tag in question.tags.slice(0, 3)" :key="tag" class="inline-flex items-center px-2 py-0.5 rounded-md text-[10px] font-medium bg-slate-100 text-slate-600 ring-1 ring-inset ring-slate-700/10">
                  {{ tag }}
                </span>
             </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-600 text-right font-medium">
            {{ formatNumber(question.score) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-right">
             <span :class="[
                'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border',
                question.answer_count > 0 ? (question.is_answered ? 'bg-green-50 text-green-700 border-green-200' : 'bg-slate-50 text-slate-700 border-slate-200') : 'text-slate-400 border-transparent'
             ]">
                {{ question.answer_count }} answers
             </span>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500 text-right font-mono">
            {{ formatNumber(question.view_count) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500 text-right">
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