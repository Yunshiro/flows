<script setup lang="ts">
import { useReviewStore } from '../stores/useReviewStore'
import { useTodoStore } from '../stores/useTodoStore'
import { onMounted, ref, computed } from 'vue'
import MarkdownEditor from '../components/MarkdownEditor.vue'

const reviewStore = useReviewStore()
const todoStore = useTodoStore()
const content = ref('')
const mood = ref<'productive' | 'normal' | 'slacking'>('normal')
const saved = ref(false)

const moods = [
  { key: 'productive' as const, label: '高效', sym: '>' },
  { key: 'normal' as const, label: '一般', sym: '=' },
  { key: 'slacking' as const, label: '低效', sym: '<' },
]

onMounted(async () => {
  await Promise.all([reviewStore.fetchReviews(), todoStore.fetchTodos()])
  if (reviewStore.todayReview) {
    content.value = reviewStore.todayReview.content
    mood.value = reviewStore.todayReview.mood
  }
})

async function handleSave() {
  await reviewStore.saveReview({
    content: content.value,
    mood: mood.value,
  })
  saved.value = true
  setTimeout(() => saved.value = false, 2000)
}

const todayCompletedTodos = computed(() =>
  todoStore.todayTodos.filter(t => t.done)
)

function insertTodoLink(title: string) {
  content.value += `\n- [x] ${title}`
}
</script>

<template>
  <div class="page">
    <header class="page-header">
      <div>
        <h2 class="page-title">每日复盘</h2>
        <p class="page-subtitle">{{ new Date().toLocaleDateString('zh-CN', { month: 'long', day: 'numeric', weekday: 'long' }) }}</p>
      </div>
      <button class="btn-save" @click="handleSave">{{ saved ? '已保存' : '保存复盘' }}</button>
    </header>

    <!-- Mood -->
    <div class="mood-row">
      <button
        v-for="m in moods" :key="m.key"
        :class="['mood-btn', { active: mood === m.key }]"
        @click="mood = m.key"
      >
        <span class="mood-sym">{{ m.sym }}</span>
        <span>{{ m.label }}</span>
      </button>
    </div>

    <!-- Editor -->
    <MarkdownEditor
      v-model="content"
      placeholder="今天完成了什么？遇到了什么阻碍？明天的计划...&#10;&#10;## 完成&#10;- &#10;&#10;## 问题&#10;- &#10;&#10;## 明日计划&#10;- "
    />

    <!-- Linked Todos -->
    <div v-if="todayCompletedTodos.length > 0" class="linked-section">
      <h3 class="section-title">今日完成的待办</h3>
      <div class="linked-list">
        <button
          v-for="todo in todayCompletedTodos"
          :key="todo.id"
          class="linked-item"
          @click="insertTodoLink(todo.title)"
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
            <path d="M5 13l4 4L19 7"/>
          </svg>
          <span>{{ todo.title }}</span>
          <span class="linked-hint">点击引用</span>
        </button>
      </div>
    </div>

    <!-- Timeline -->
    <section class="timeline" v-if="reviewStore.recentReviews.length > 1">
      <h3 class="section-title">最近复盘</h3>
      <div v-for="r in reviewStore.recentReviews.slice(1, 10)" :key="r.date" class="timeline-item">
        <span class="timeline-date">{{ r.date.slice(5) }}</span>
        <span :class="['timeline-mood', `mood--${r.mood}`]">
          {{ r.mood === 'productive' ? '高效' : r.mood === 'normal' ? '一般' : '低效' }}
        </span>
        <span class="timeline-preview">{{ r.content.replace(/[#*\n]/g, ' ').slice(0, 50) }}{{ r.content.length > 50 ? '...' : '' }}</span>
      </div>
    </section>
  </div>
</template>

<style scoped>
.page { max-width: 820px; }
.page-header { display: flex; align-items: flex-start; justify-content: space-between; margin-bottom: 22px; }
.page-title { font-size: 32px; font-weight: 760; letter-spacing: -0.02em; line-height: 1.08; }
.page-subtitle { font-size: 13px; color: var(--text-secondary); margin-top: 2px; }

.btn-save { display: inline-flex; padding: 8px 16px; border: none; border-radius: 10px; background: var(--accent); color: var(--accent-contrast); font-family: var(--font-sans); font-size: 13px; font-weight: 600; cursor: pointer; box-shadow: 0 10px 22px rgba(var(--accent-rgb), 0.18); }
.btn-save:hover { background: var(--accent-hover); }

.mood-row { display: flex; gap: 8px; margin-bottom: 16px; }
.mood-btn { flex: 1; display: flex; align-items: center; justify-content: center; gap: 6px; padding: 10px; border: 1px solid var(--border); border-radius: 12px; background: rgba(var(--bg-surface-rgb), 0.72); font-family: var(--font-sans); font-size: 13px; color: var(--text-secondary); cursor: pointer; box-shadow: var(--shadow-xs); }
.mood-btn:hover { background: var(--bg-surface); border-color: var(--border-strong); }
.mood-btn.active { border-color: rgba(var(--accent-rgb), 0.36); background: var(--accent-soft); color: var(--text-primary); box-shadow: inset 0 0 0 1px rgba(var(--accent-rgb), 0.1); }
.mood-sym { font-family: var(--font-mono); font-size: 15px; font-weight: 600; }

.linked-section { margin-top: 16px; }
.section-title { font-size: 13px; font-weight: 650; margin-bottom: 8px; color: var(--text-primary); }
.linked-list { display: flex; flex-direction: column; gap: 4px; }
.linked-item { display: flex; align-items: center; gap: 8px; padding: 7px 10px; border: 1px solid var(--border); border-radius: 10px; background: rgba(var(--bg-surface-rgb), 0.7); color: var(--text-secondary); font-family: var(--font-sans); font-size: 12.5px; cursor: pointer; width: 100%; text-align: left; }
.linked-item:hover { border-color: var(--accent); color: var(--text-primary); background: var(--bg-surface); }
.linked-hint { margin-left: auto; font-size: 10px; color: var(--text-disabled); }
.linked-item:hover .linked-hint { color: var(--text-secondary); }

.timeline { margin-top: 28px; display: flex; flex-direction: column; }
.timeline-item { display: flex; align-items: center; gap: 10px; padding: 8px 0; border-bottom: 1px solid var(--border); }
.timeline-date { font-family: var(--font-mono); font-size: 12px; color: var(--text-tertiary); min-width: 48px; }
.timeline-mood { font-size: 11px; padding: 1px 6px; border-radius: 999px; flex-shrink: 0; }
.mood--productive { background: var(--tag-green-bg); color: var(--tag-green-text); }
.mood--normal { background: var(--tag-gray-bg); color: var(--text-secondary); }
.mood--slacking { background: var(--tag-red-bg); color: var(--tag-red-text); }
.timeline-preview { font-size: 12.5px; color: var(--text-secondary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
</style>
