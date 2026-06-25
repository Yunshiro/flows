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
.page { max-width: 700px; }
.page-header { display: flex; align-items: flex-start; justify-content: space-between; margin-bottom: 16px; }
.page-title { font-size: 26px; font-weight: 620; letter-spacing: -0.025em; }
.page-subtitle { font-size: 13px; color: #787774; margin-top: 2px; }

.btn-save { display: inline-flex; padding: 7px 16px; border: none; border-radius: 6px; background: #111111; color: #FFFFFF; font-family: var(--font-sans); font-size: 13px; font-weight: 520; cursor: pointer; transition: all 150ms ease; }
.btn-save:hover { background: #333333; }

.mood-row { display: flex; gap: 6px; margin-bottom: 14px; }
.mood-btn { flex: 1; display: flex; align-items: center; justify-content: center; gap: 6px; padding: 8px; border: 1px solid #EAEAEA; border-radius: 6px; background: #FFFFFF; font-family: var(--font-sans); font-size: 13px; color: #787774; cursor: pointer; transition: all 120ms ease; }
.mood-btn:hover { background: rgba(0,0,0,0.02); }
.mood-btn.active { border-color: #111111; background: rgba(0,0,0,0.03); color: #111111; }
.mood-sym { font-family: var(--font-mono); font-size: 15px; font-weight: 600; }

.linked-section { margin-top: 16px; }
.section-title { font-size: 13px; font-weight: 580; margin-bottom: 8px; color: #111111; }
.linked-list { display: flex; flex-direction: column; gap: 4px; }
.linked-item { display: flex; align-items: center; gap: 8px; padding: 6px 10px; border: 1px solid #EAEAEA; border-radius: 6px; background: #FFFFFF; color: #787774; font-family: var(--font-sans); font-size: 12.5px; cursor: pointer; transition: all 100ms ease; width: 100%; text-align: left; }
.linked-item:hover { border-color: #787774; color: #111111; }
.linked-hint { margin-left: auto; font-size: 10px; color: #CECECE; }
.linked-item:hover .linked-hint { color: #787774; }

.timeline { margin-top: 28px; display: flex; flex-direction: column; }
.timeline-item { display: flex; align-items: center; gap: 10px; padding: 8px 0; border-bottom: 1px solid #EAEAEA; }
.timeline-date { font-family: var(--font-mono); font-size: 12px; color: #9E9E9E; min-width: 48px; }
.timeline-mood { font-size: 11px; padding: 1px 6px; border-radius: 999px; flex-shrink: 0; }
.mood--productive { background: #EDF3EC; color: #346538; }
.mood--normal { background: #F1F1EF; color: #787774; }
.mood--slacking { background: #FDEBEC; color: #9F2F2D; }
.timeline-preview { font-size: 12.5px; color: #787774; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
</style>
