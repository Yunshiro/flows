<script setup lang="ts">
import { useTodoStore } from '../stores/useTodoStore'
import { useReviewStore } from '../stores/useReviewStore'
import { onMounted, onUnmounted, ref, computed } from 'vue'
import { isTauri } from '../api/adapter'
import { reviewApi } from '../api/reviews'
import { llmApi, type LlmConfig } from '../api/llm'

const todoStore = useTodoStore()
const reviewStore = useReviewStore()
const generating = ref(false)
const summaryContent = ref('')
const streamingChunks = ref<string[]>([])
const llmConfigs = ref<LlmConfig[]>([])
const selectedConfigId = ref('')
let unlisten: (() => void) | null = null

onMounted(async () => {
  await Promise.all([todoStore.fetchTodos(), reviewStore.fetchReviews()])
  try {
    llmConfigs.value = await llmApi.list()
    const def = llmConfigs.value.find(c => c.isDefault)
    if (def) selectedConfigId.value = def.id
    else if (llmConfigs.value.length > 0) selectedConfigId.value = llmConfigs.value[0].id
  } catch { /* */ }
})

onUnmounted(() => {
  if (unlisten) unlisten()
})

const thisWeekTodos = computed(() => todoStore.items)
const completedCount = computed(() => thisWeekTodos.value.filter(t => t.done).length)
const totalCount = computed(() => thisWeekTodos.value.length)
const completionPct = computed(() => totalCount.value === 0 ? 0 : Math.round((completedCount.value / totalCount.value) * 100))

const dailyCompletions = computed(() => {
  const map: Record<string, number> = {}
  thisWeekTodos.value.forEach(t => {
    if (t.done && t.date) map[t.date] = (map[t.date] || 0) + 1
  })
  const days: { date: string; label: string; count: number }[] = []
  const now = new Date()
  for (let i = 6; i >= 0; i--) {
    const d = new Date(now)
    d.setDate(d.getDate() - i)
    const ds = d.toISOString().slice(0, 10)
    const dayLabels = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']
    days.push({ date: ds, label: dayLabels[d.getDay()], count: map[ds] || 0 })
  }
  return days
})

const maxDaily = computed(() => Math.max(...dailyCompletions.value.map(d => d.count), 1))

const weekLabel = computed(() => {
  const now = new Date()
  const day = now.getDay()
  const monday = new Date(now)
  monday.setDate(now.getDate() - (day === 0 ? 6 : day - 1))
  const sunday = new Date(monday)
  sunday.setDate(monday.getDate() + 6)
  return `${monday.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' })} — ${sunday.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' })}`
})

const moodSummary = computed(() => {
  const count: Record<string, number> = { productive: 0, normal: 0, slacking: 0 }
  reviewStore.recentReviews.forEach(r => { if (count[r.mood] !== undefined) count[r.mood]++ })
  const total = count.productive + count.normal + count.slacking
  if (total === 0) return null
  return {
    productivePct: Math.round((count.productive / total) * 100),
    normalPct: Math.round((count.normal / total) * 100),
    slackingPct: Math.round((count.slacking / total) * 100),
    total,
  }
})

async function handleGenerate() {
  generating.value = true
  streamingChunks.value = []
  summaryContent.value = ''

  if (isTauri()) {
    // Streaming mode — listen for llm-chunk events
    try {
      const { listen } = await import('@tauri-apps/api/event')
      unlisten = await listen<{ content: string; done: boolean; error: string | null }>('llm-chunk', (event) => {
        const { content, done, error } = event.payload
        if (error) {
          streamingChunks.value.push(`\n\n> ${error}`)
          generating.value = false
          return
        }
        if (content) {
          streamingChunks.value.push(content)
        }
        if (done) {
          generating.value = false
        }
      })

      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('generate_weekly_summary_stream', {
        configId: selectedConfigId.value || null,
      })
    } catch (e) {
      streamingChunks.value.push(`\n\n> 启动流式生成失败: ${String(e)}`)
      generating.value = false
    }
  } else {
    // Browser fallback — non-streaming
    try {
      summaryContent.value = await reviewApi.generateWeeklySummary()
    } catch (e) {
      summaryContent.value = '生成失败: ' + String(e)
    }
    generating.value = false
  }
}

function handleCopy() {
  const text = summaryContent.value || streamingChunks.value.join('')
  navigator.clipboard.writeText(text)
}
</script>

<template>
  <div class="page">
    <header class="page-header">
      <div>
        <h2 class="page-title">每周总览</h2>
        <p class="page-subtitle">{{ weekLabel }}</p>
      </div>
    </header>

    <!-- Stats -->
    <section class="card">
      <h3 class="card-title">本周统计</h3>
      <div class="stats-row">
        <div class="stat-ring-wrap">
          <svg class="ring" viewBox="0 0 100 100">
            <circle class="ring-bg" cx="50" cy="50" r="42" />
            <circle class="ring-fill" cx="50" cy="50" r="42"
              :stroke-dasharray="2 * Math.PI * 42"
              :stroke-dashoffset="2 * Math.PI * 42 * (1 - completionPct / 100)"
            />
          </svg>
          <div class="ring-inner">
            <span class="ring-num">{{ completionPct }}%</span>
            <span class="ring-label">{{ completedCount }}/{{ totalCount }} 项</span>
          </div>
        </div>
        <div class="bars">
          <div v-for="d in dailyCompletions" :key="d.date" class="bar-col">
            <div class="bar-track">
              <div class="bar-fill" :style="{ height: (d.count / maxDaily * 100) + '%' }">
                <span v-if="d.count > 0" class="bar-val">{{ d.count }}</span>
              </div>
            </div>
            <span class="bar-day">{{ d.label }}</span>
          </div>
        </div>
      </div>
      <div v-if="moodSummary" class="mood-summary">
        <div class="mood-bar">
          <div class="mood-seg mood--productive" :style="{ flex: moodSummary.productivePct || 0.1 }" />
          <div class="mood-seg mood--normal" :style="{ flex: moodSummary.normalPct || 0.1 }" />
          <div class="mood-seg mood--slacking" :style="{ flex: moodSummary.slackingPct || 0.1 }" />
        </div>
        <div class="mood-legend">
          <span class="mood-legend-item"><span class="mood-dot mood--productive" />高效 {{ moodSummary.productivePct }}%</span>
          <span class="mood-legend-item"><span class="mood-dot mood--normal" />一般 {{ moodSummary.normalPct }}%</span>
          <span class="mood-legend-item"><span class="mood-dot mood--slacking" />低效 {{ moodSummary.slackingPct }}%</span>
        </div>
      </div>
    </section>

    <!-- AI Summary -->
    <section class="card">
      <div class="card-header">
        <div class="card-header-left">
          <h3 class="card-title">AI 周报总结</h3>
          <select v-if="llmConfigs.length > 0" v-model="selectedConfigId" class="config-select">
            <option v-for="c in llmConfigs" :key="c.id" :value="c.id">{{ c.name }} ({{ c.model }})</option>
          </select>
          <span v-else class="no-config-hint">请先在设置中配置 AI 模型</span>
        </div>
        <div class="card-actions">
          <button v-if="summaryContent || streamingChunks.length > 0" class="btn-ghost" @click="handleCopy">复制</button>
          <button class="btn-gen" @click="handleGenerate" :disabled="generating || llmConfigs.length === 0">
            {{ generating ? '生成中...' : summaryContent || streamingChunks.length > 0 ? '重新生成' : '生成总结' }}
          </button>
        </div>
      </div>

      <!-- Streaming display -->
      <div v-if="streamingChunks.length > 0" class="stream-output">
        <span v-for="(chunk, i) in streamingChunks" :key="i" class="stream-chunk">{{ chunk }}</span>
        <span v-if="generating" class="cursor-blink">|</span>
      </div>

      <!-- Fallback display -->
      <div v-else-if="summaryContent" class="summary-text">{{ summaryContent }}</div>

      <p v-else class="summary-placeholder">
        点击"生成总结"，AI 将根据每日复盘和待办数据<strong>实时流式输出</strong>本周工作报告。
      </p>
    </section>
  </div>
</template>

<style scoped>
.page { max-width: 860px; }
.page-header { display: flex; align-items: flex-start; justify-content: space-between; margin-bottom: 22px; }
.page-title { font-size: 32px; font-weight: 760; letter-spacing: -0.02em; line-height: 1.08; }
.page-subtitle { font-size: 13px; color: var(--text-secondary); margin-top: 2px; }

.card { background: rgba(var(--bg-surface-rgb), 0.78); border: 1px solid var(--border); border-radius: var(--radius-xl); padding: 22px; margin-bottom: 18px; box-shadow: var(--shadow-sm); backdrop-filter: blur(18px); }
.card-title { font-size: 15px; font-weight: 680; margin-bottom: 8px; color: var(--text-primary); }
.card-header { display: flex; align-items: flex-start; justify-content: space-between; margin-bottom: 8px; }
.card-header-left { display: flex; flex-direction: column; gap: 8px; }
.card-actions { display: flex; gap: 8px; align-items: center; flex-shrink: 0; }

.config-select {
  padding: 4px 8px;
  border: 1px solid var(--border);
  border-radius: 5px;
  background: var(--bg-canvas);
  font-family: var(--font-sans);
  font-size: 12px;
  color: var(--text-primary);
  outline: none;
  cursor: pointer;
  max-width: 280px;
}

.config-select:focus { border-color: var(--accent); }

.no-config-hint {
  font-size: 12px;
  color: var(--text-tertiary);
}

.stats-row { display: flex; gap: 28px; align-items: flex-end; margin: 12px 0 8px; }
.stat-ring-wrap { position: relative; width: 90px; height: 90px; flex-shrink: 0; }
.ring { width: 100%; height: 100%; transform: rotate(-90deg); }
.ring-bg { fill: none; stroke: var(--tag-gray-bg); stroke-width: 6; }
.ring-fill { fill: none; stroke: var(--accent); stroke-width: 6; stroke-linecap: round; transition: stroke-dashoffset 800ms cubic-bezier(0.16, 1, 0.3, 1); filter: drop-shadow(0 3px 8px rgba(var(--accent-rgb), 0.22)); }
.ring-inner { position: absolute; inset: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; }
.ring-num { font-size: 20px; font-weight: 600; line-height: 1; }
.ring-label { font-size: 10px; color: var(--text-tertiary); margin-top: 2px; }

.bars { display: flex; align-items: flex-end; gap: 8px; flex: 1; height: 80px; }
.bar-col { flex: 1; display: flex; flex-direction: column; align-items: center; gap: 4px; height: 100%; }
.bar-track { flex: 1; width: 100%; display: flex; align-items: flex-end; background: var(--tag-gray-bg); border-radius: 3px; overflow: hidden; }
.bar-fill { position: relative; width: 100%; background: linear-gradient(180deg, var(--accent-hover), var(--accent)); border-radius: 3px; min-height: 2px; transition: height 600ms cubic-bezier(0.16, 1, 0.3, 1); display: flex; align-items: flex-start; justify-content: center; }
.bar-val { font-size: 9px; color: var(--accent-contrast); padding-top: 2px; font-weight: 600; }
.bar-day { font-family: var(--font-mono); font-size: 10px; color: var(--text-tertiary); }

.mood-summary { margin-top: 16px; padding-top: 12px; border-top: 1px solid var(--border); }
.mood-bar { display: flex; height: 6px; border-radius: 3px; overflow: hidden; gap: 1px; }
.mood-seg { min-width: 2px; }
.mood-seg.mood--productive { background: var(--tag-green-text); }
.mood-seg.mood--normal { background: var(--text-tertiary); }
.mood-seg.mood--slacking { background: var(--tag-red-text); }
.mood-legend { display: flex; gap: 16px; margin-top: 8px; }
.mood-legend-item { display: flex; align-items: center; gap: 4px; font-size: 12px; color: var(--text-secondary); }
.mood-dot { width: 8px; height: 8px; border-radius: 50%; display: inline-block; }
.mood-dot.mood--productive { background: var(--tag-green-text); }
.mood-dot.mood--normal { background: var(--text-tertiary); }
.mood-dot.mood--slacking { background: var(--tag-red-text); }

.btn-gen { padding: 7px 14px; border: 1px solid var(--accent); border-radius: 9px; background: var(--accent); color: var(--accent-contrast); font-family: var(--font-sans); font-size: 13px; font-weight: 600; cursor: pointer; box-shadow: 0 10px 22px rgba(var(--accent-rgb), 0.18); }
.btn-gen:hover:not(:disabled) { background: var(--accent-hover); border-color: var(--accent-hover); }
.btn-gen:disabled { opacity: 0.5; cursor: default; }

.btn-ghost { padding: 5px 10px; border: none; background: transparent; color: var(--text-secondary); font-family: var(--font-sans); font-size: 12px; cursor: pointer; border-radius: 4px; }
.btn-ghost:hover { background: var(--surface-hover); color: var(--text-primary); }

/* Streaming output */
.stream-output {
  font-size: 14px;
  color: var(--text-primary);
  line-height: 1.7;
  white-space: pre-wrap;
  min-height: 60px;
  padding: 12px 0;
}

.stream-chunk {
  animation: fadeIn 200ms ease;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.cursor-blink {
  display: inline;
  color: var(--text-primary);
  font-weight: 300;
  animation: blink 0.8s infinite;
}

@keyframes blink {
  0%, 50% { opacity: 1; }
  51%, 100% { opacity: 0; }
}

.summary-text { font-size: 14px; color: var(--text-primary); line-height: 1.7; white-space: pre-wrap; }
.summary-placeholder { font-size: 13px; color: var(--text-tertiary); padding: 20px 0; }
.summary-placeholder strong { color: var(--text-secondary); }
</style>
