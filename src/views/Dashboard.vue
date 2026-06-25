<script setup lang="ts">
import { ref, computed } from 'vue'

// ── Types ──
interface TodoItem {
  id: string
  title: string
  priority: 'high' | 'medium' | 'low'
  tags: string[]
  done: boolean
  estMinutes: number
}

interface DayStat {
  day: string
  completed: number
}

interface RecentNote {
  title: string
  path: string
  preview: string
  updatedAt: string
}

// ── Navigation ──
const navItems = [
  { key: 'todos', label: '今日待办' },
  { key: 'review', label: '每日复盘' },
  { key: 'weekly', label: '每周总览' },
  { key: 'notes', label: '笔记' },
]
const activeNav = ref('todos')

// ── Today's Todos ──
const todos = ref<TodoItem[]>([
  { id: '1', title: '完成 Flows 仪表盘 UI 设计', priority: 'high', tags: ['dev'], done: true, estMinutes: 120 },
  { id: '2', title: '实现 Tauri SQLite 数据持久层', priority: 'high', tags: ['rust'], done: false, estMinutes: 90 },
  { id: '3', title: '编写每日复盘 Markdown 编辑器', priority: 'medium', tags: ['dev', 'ui'], done: false, estMinutes: 60 },
  { id: '4', title: '调研 Git 同步方案可行性', priority: 'medium', tags: ['research'], done: false, estMinutes: 45 },
  { id: '5', title: '整理项目开发计划文档', priority: 'low', tags: ['docs'], done: true, estMinutes: 30 },
])

const newTodoTitle = ref('')

function toggleTodo(id: string) {
  const item = todos.value.find(t => t.id === id)
  if (item) item.done = !item.done
}

function addTodo() {
  const title = newTodoTitle.value.trim()
  if (!title) return
  todos.value.unshift({
    id: crypto.randomUUID(),
    title,
    priority: 'medium',
    tags: [],
    done: false,
    estMinutes: 30,
  })
  newTodoTitle.value = ''
}

function removeTodo(id: string) {
  todos.value = todos.value.filter(t => t.id !== id)
}

const priorityLabel: Record<string, string> = { high: 'High', medium: 'Mid', low: 'Low' }

// ── Weekly Stats ──
const dayStats = ref<DayStat[]>([
  { day: 'Mon', completed: 4 },
  { day: 'Tue', completed: 6 },
  { day: 'Wed', completed: 3 },
  { day: 'Thu', completed: 5 },
  { day: 'Fri', completed: 7 },
  { day: 'Sat', completed: 2 },
  { day: 'Sun', completed: 0 },
])

const weeklyTotal = computed(() => todos.value.length)
const weeklyDone = computed(() => todos.value.filter(t => t.done).length)
const completionRate = computed(() => {
  if (weeklyTotal.value === 0) return 0
  return Math.round((weeklyDone.value / weeklyTotal.value) * 100)
})

const maxCompleted = computed(() => Math.max(...dayStats.value.map(d => d.completed), 1))

// ── Today's Review ──
const reviewContent = ref('')
const reviewMood = ref<'productive' | 'normal' | 'slacking'>('normal')
const moods = [
  { key: 'productive' as const, label: '高效', face: '>' },
  { key: 'normal' as const, label: '一般', face: '=' },
  { key: 'slacking' as const, label: '低效', face: '<' },
]

// ── Recent Notes ──
const recentNotes = ref<RecentNote[]>([
  { title: 'Tauri v2 踩坑记录', path: 'dev/tauri-notes', preview: '记录 Tauri v2 开发中遇到的问题与解决方案...', updatedAt: '2 小时前' },
  { title: '时间管理方法论', path: 'notes/methodology', preview: 'GTD、番茄工作法、四象限法则的个人实践总结...', updatedAt: '昨天' },
  { title: 'Flows 产品路线图', path: 'flows/roadmap', preview: '面向个人用户的轻量级时间管理桌面工具...', updatedAt: '3 天前' },
])

// ── Greeting ──
const greeting = computed(() => {
  const h = new Date().getHours()
  if (h < 12) return '早上好'
  if (h < 18) return '下午好'
  return '晚上好'
})

const todayDate = computed(() => {
  return new Date().toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    weekday: 'long',
  })
})
</script>

<template>
  <div class="dashboard">
    <!-- Sidebar -->
    <aside class="sidebar">
      <div class="sidebar-brand">
        <svg class="brand-icon" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2">
          <circle cx="12" cy="12" r="10"/>
          <path d="M12 6v6l4 2"/>
        </svg>
        <span class="brand-name">Flows</span>
      </div>

      <nav class="sidebar-nav">
        <button
          v-for="item in navItems"
          :key="item.key"
          :class="['nav-item', { active: activeNav === item.key }]"
          @click="activeNav = item.key"
        >
          <span class="nav-dot" />
          {{ item.label }}
        </button>
      </nav>

      <div class="sidebar-footer">
        <div class="sidebar-version">v0.1.0</div>
      </div>
    </aside>

    <!-- Main Content -->
    <main class="main">
      <header class="main-header">
        <div>
          <h2 class="greeting">{{ greeting }}</h2>
          <p class="today-date">{{ todayDate }}</p>
        </div>
        <button class="btn-primary" @click="addTodo">+ 新建待办</button>
      </header>

      <!-- Quick Add -->
      <div class="quick-add">
        <input
          v-model="newTodoTitle"
          class="quick-add-input"
          placeholder="快速添加待办事项..."
          @keydown.enter="addTodo"
        />
        <button class="quick-add-btn" @click="addTodo" :disabled="!newTodoTitle.trim()">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <path d="M12 5v14M5 12h14"/>
          </svg>
        </button>
      </div>

      <!-- Todo List -->
      <section class="card card--todos">
        <div class="card-header">
          <h3 class="card-title">今日待办</h3>
          <span class="card-badge">{{ weeklyDone }}/{{ weeklyTotal }}</span>
        </div>
        <div class="todo-list">
          <div
            v-for="(todo, index) in todos"
            :key="todo.id"
            :class="['todo-item', { done: todo.done }]"
            :style="{ '--index': index }"
          >
            <span class="todo-grip">
              <svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor" opacity="0.2">
                <circle cx="3" cy="2" r="1"/><circle cx="9" cy="2" r="1"/>
                <circle cx="3" cy="6" r="1"/><circle cx="9" cy="6" r="1"/>
                <circle cx="3" cy="10" r="1"/><circle cx="9" cy="10" r="1"/>
              </svg>
            </span>

            <button
              :class="['todo-check', { checked: todo.done }]"
              @click="toggleTodo(todo.id)"
            >
              <svg v-if="todo.done" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                <path d="M5 13l4 4L19 7"/>
              </svg>
            </button>

            <div class="todo-body">
              <span :class="['todo-title', { done: todo.done }]">{{ todo.title }}</span>
              <div class="todo-meta">
                <span :class="['tag', `tag--${todo.priority}`]">{{ priorityLabel[todo.priority] }}</span>
                <span v-for="tag in todo.tags" :key="tag" class="tag tag--category">{{ tag }}</span>
                <span class="todo-time">{{ todo.estMinutes }}m</span>
              </div>
            </div>

            <button class="todo-delete" @click="removeTodo(todo.id)">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M18 6L6 18M6 6l12 12"/>
              </svg>
            </button>
          </div>
        </div>
      </section>

      <!-- Weekly Stats -->
      <section class="card card--stats">
        <div class="card-header">
          <h3 class="card-title">本周概览</h3>
        </div>
        <div class="stats-grid">
          <div class="stat-ring">
            <svg class="ring-svg" viewBox="0 0 100 100">
              <circle class="ring-bg" cx="50" cy="50" r="42" />
              <circle
                class="ring-fill"
                cx="50" cy="50" r="42"
                :stroke-dasharray="2 * Math.PI * 42"
                :stroke-dashoffset="2 * Math.PI * 42 * (1 - completionRate / 100)"
              />
            </svg>
            <div class="ring-inner">
              <span class="ring-value">{{ completionRate }}%</span>
              <span class="ring-label">完成率</span>
            </div>
          </div>

          <div class="stat-bars">
            <div
              v-for="stat in dayStats"
              :key="stat.day"
              class="bar-col"
            >
              <div class="bar-track">
                <div
                  class="bar-fill"
                  :style="{ height: (stat.completed / maxCompleted * 100) + '%' }"
                />
              </div>
              <span class="bar-label">{{ stat.day }}</span>
            </div>
          </div>
        </div>
      </section>
    </main>

    <!-- Right Panel -->
    <aside class="right-panel">
      <!-- Daily Review -->
      <div class="panel-card">
        <div class="panel-header">
          <h4 class="panel-title">今日复盘</h4>
          <span class="panel-date">{{ new Date().toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' }) }}</span>
        </div>
        <div class="mood-selector">
          <button
            v-for="m in moods"
            :key="m.key"
            :class="['mood-btn', { active: reviewMood === m.key }]"
            @click="reviewMood = m.key"
          >
            <span class="mood-symbol">{{ m.face }}</span>
            <span class="mood-label">{{ m.label }}</span>
          </button>
        </div>
        <textarea
          v-model="reviewContent"
          class="review-textarea"
          placeholder="今天完成了什么？遇到了什么阻碍？明天的计划..."
          rows="4"
        />
        <button class="btn-primary btn-block">保存复盘</button>
      </div>

      <!-- Recent Notes -->
      <div class="panel-card">
        <div class="panel-header">
          <h4 class="panel-title">最近笔记</h4>
          <a href="#" class="panel-link" @click.prevent="activeNav = 'notes'">全部</a>
        </div>
        <div class="notes-list">
          <div
            v-for="note in recentNotes"
            :key="note.path"
            class="note-item"
          >
            <div class="note-icon">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/>
                <path d="M14 2v6h6M16 13H8M16 17H8M10 9H8"/>
              </svg>
            </div>
            <div class="note-body">
              <span class="note-title">{{ note.title }}</span>
              <span class="note-meta">{{ note.updatedAt }}</span>
            </div>
          </div>
        </div>
      </div>
    </aside>
  </div>
</template>

<style>
/* ── Reset & Base ── */
*,
*::before,
*::after {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  --bg-canvas: #F7F6F3;
  --bg-surface: #FFFFFF;
  --border: #EAEAEA;
  --text-primary: #111111;
  --text-secondary: #787774;
  --text-tertiary: #9E9E9E;
  --accent: #111111;
  --tag-red-bg: #FDEBEC;
  --tag-red-text: #9F2F2D;
  --tag-yellow-bg: #FBF3DB;
  --tag-yellow-text: #956400;
  --tag-blue-bg: #E1F3FE;
  --tag-blue-text: #1F6C9F;
  --tag-gray-bg: #F1F1EF;
  --tag-gray-text: #787774;
  --font-sans: 'SF Pro Display', 'Geist Sans', 'Helvetica Neue', 'PingFang SC', 'Microsoft YaHei', sans-serif;
  --font-mono: 'Geist Mono', 'SF Mono', 'JetBrains Mono', 'Cascadia Code', monospace;
}

body {
  background: var(--bg-canvas);
  color: var(--text-primary);
  font-family: var(--font-sans);
  font-size: 14px;
  line-height: 1.6;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  overflow: hidden;
}
</style>

<style scoped>
/* ── Dashboard Layout ── */
.dashboard {
  display: grid;
  grid-template-columns: 200px 1fr 260px;
  height: 100vh;
  overflow: hidden;
}

/* ── Sidebar ── */
.sidebar {
  display: flex;
  flex-direction: column;
  padding: 20px 16px;
  border-right: 1px solid var(--border);
  background: var(--bg-surface);
  user-select: none;
}

.sidebar-brand {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 8px 24px;
  color: var(--text-primary);
}

.brand-icon {
  color: var(--text-primary);
  flex-shrink: 0;
}

.brand-name {
  font-size: 17px;
  font-weight: 620;
  letter-spacing: -0.02em;
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: 2px;
  flex: 1;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 8px 10px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--text-secondary);
  font-family: var(--font-sans);
  font-size: 14px;
  font-weight: 450;
  cursor: pointer;
  transition: background 150ms ease, color 150ms ease;
  text-align: left;
}

.nav-item:hover {
  background: rgba(0, 0, 0, 0.04);
  color: var(--text-primary);
}

.nav-item.active {
  background: rgba(0, 0, 0, 0.06);
  color: var(--text-primary);
  font-weight: 520;
}

.nav-dot {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background: transparent;
  transition: background 150ms ease;
  flex-shrink: 0;
}

.nav-item.active .nav-dot {
  background: var(--text-primary);
}

.sidebar-footer {
  padding: 16px 10px 0;
}

.sidebar-version {
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--text-tertiary);
}

/* ── Main Content ── */
.main {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 28px 28px 28px 28px;
  overflow-y: auto;
}

.main-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
}

.greeting {
  font-size: 26px;
  font-weight: 620;
  line-height: 1.2;
  letter-spacing: -0.025em;
  color: var(--text-primary);
}

.today-date {
  font-size: 13px;
  color: var(--text-secondary);
  margin-top: 2px;
}

/* ── Buttons ── */
.btn-primary {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 7px 16px;
  border: none;
  border-radius: 6px;
  background: var(--accent);
  color: #FFFFFF;
  font-family: var(--font-sans);
  font-size: 13px;
  font-weight: 520;
  cursor: pointer;
  transition: background 150ms ease, transform 100ms ease;
}

.btn-primary:hover {
  background: #333333;
}

.btn-primary:active {
  transform: scale(0.98);
}

.btn-primary:disabled {
  opacity: 0.4;
  cursor: default;
}

.btn-block {
  width: 100%;
  justify-content: center;
  padding: 8px 16px;
}

/* ── Quick Add ── */
.quick-add {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 0 0 0;
}

.quick-add-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-surface);
  color: var(--text-primary);
  font-family: var(--font-sans);
  font-size: 13px;
  outline: none;
  transition: border-color 150ms ease;
}

.quick-add-input:focus {
  border-color: var(--text-secondary);
}

.quick-add-input::placeholder {
  color: var(--text-tertiary);
}

.quick-add-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-surface);
  color: var(--text-secondary);
  cursor: pointer;
  transition: background 100ms ease;
  flex-shrink: 0;
}

.quick-add-btn:hover:not(:disabled) {
  background: rgba(0, 0, 0, 0.04);
}

/* ── Cards ── */
.card {
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 8px;
  overflow: hidden;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 20px 0;
}

.card-title {
  font-size: 15px;
  font-weight: 580;
  color: var(--text-primary);
}

.card-badge {
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--text-secondary);
  padding: 2px 8px;
  background: var(--tag-gray-bg);
  border-radius: 999px;
}

/* ── Todo List ── */
.card--todos {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.todo-list {
  padding: 14px 12px 16px;
  display: flex;
  flex-direction: column;
}

.todo-item {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 10px 8px;
  border-radius: 6px;
  transition: background 100ms ease;
  animation: fadeIn 400ms ease both;
  animation-delay: calc(var(--index) * 60ms);
}

.todo-item:hover {
  background: rgba(0, 0, 0, 0.02);
}

.todo-item:hover .todo-grip {
  opacity: 1;
}

.todo-item:hover .todo-delete {
  opacity: 1;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(6px); }
  to   { opacity: 1; transform: translateY(0); }
}

.todo-grip {
  opacity: 0;
  padding-top: 5px;
  cursor: grab;
  transition: opacity 150ms ease;
  flex-shrink: 0;
}

.todo-check {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  margin-top: 3px;
  border: 1.5px solid #CECECE;
  border-radius: 4px;
  background: transparent;
  color: transparent;
  cursor: pointer;
  transition: all 150ms ease;
  flex-shrink: 0;
}

.todo-check:hover {
  border-color: var(--text-secondary);
}

.todo-check.checked {
  background: var(--accent);
  border-color: var(--accent);
  color: #FFFFFF;
}

.todo-body {
  flex: 1;
  min-width: 0;
}

.todo-title {
  font-size: 13.5px;
  font-weight: 480;
  color: var(--text-primary);
  line-height: 1.4;
}

.todo-title.done {
  color: var(--text-tertiary);
  text-decoration: line-through;
  text-decoration-color: #CECECE;
}

.todo-meta {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-top: 5px;
}

.todo-time {
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--text-tertiary);
  margin-left: auto;
}

.todo-delete {
  opacity: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  margin-top: 1px;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  transition: all 100ms ease;
  flex-shrink: 0;
}

.todo-delete:hover {
  background: var(--tag-red-bg);
  color: var(--tag-red-text);
}

/* ── Tags ── */
.tag {
  display: inline-flex;
  align-items: center;
  padding: 1px 7px;
  border-radius: 999px;
  font-size: 10.5px;
  font-weight: 520;
  letter-spacing: 0.04em;
  text-transform: uppercase;
}

.tag--high {
  background: var(--tag-red-bg);
  color: var(--tag-red-text);
}

.tag--medium {
  background: var(--tag-yellow-bg);
  color: var(--tag-yellow-text);
}

.tag--low {
  background: var(--tag-blue-bg);
  color: var(--tag-blue-text);
}

.tag--category {
  background: var(--tag-gray-bg);
  color: var(--tag-gray-text);
}

/* ── Weekly Stats ── */
.card--stats {
  padding-bottom: 20px;
}

.stats-grid {
  display: grid;
  grid-template-columns: 140px 1fr;
  gap: 24px;
  padding: 16px 20px 8px;
  align-items: end;
}

/* Ring chart */
.stat-ring {
  position: relative;
  width: 100px;
  height: 100px;
  margin: 0 auto;
}

.ring-svg {
  width: 100%;
  height: 100%;
  transform: rotate(-90deg);
}

.ring-bg {
  fill: none;
  stroke: #F1F1EF;
  stroke-width: 6;
}

.ring-fill {
  fill: none;
  stroke: var(--accent);
  stroke-width: 6;
  stroke-linecap: round;
  transition: stroke-dashoffset 800ms cubic-bezier(0.16, 1, 0.3, 1);
}

.ring-inner {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.ring-value {
  font-size: 22px;
  font-weight: 620;
  color: var(--text-primary);
  line-height: 1;
  letter-spacing: -0.02em;
}

.ring-label {
  font-size: 11px;
  color: var(--text-tertiary);
  margin-top: 3px;
}

/* Bar chart */
.stat-bars {
  display: flex;
  align-items: flex-end;
  gap: 10px;
  height: 100px;
}

.bar-col {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  height: 100%;
}

.bar-track {
  flex: 1;
  width: 100%;
  display: flex;
  align-items: flex-end;
  background: #F1F1EF;
  border-radius: 3px;
  overflow: hidden;
}

.bar-fill {
  width: 100%;
  background: var(--accent);
  border-radius: 3px;
  min-height: 2px;
  transition: height 600ms cubic-bezier(0.16, 1, 0.3, 1);
}

.bar-label {
  font-size: 10.5px;
  font-weight: 480;
  color: var(--text-tertiary);
  font-family: var(--font-mono);
}

/* ── Right Panel ── */
.right-panel {
  display: flex;
  flex-direction: column;
  gap: 14px;
  padding: 20px 16px;
  border-left: 1px solid var(--border);
  background: var(--bg-surface);
  overflow-y: auto;
}

.panel-card {
  background: transparent;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 10px;
}

.panel-title {
  font-size: 14px;
  font-weight: 580;
  color: var(--text-primary);
}

.panel-date {
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--text-tertiary);
}

.panel-link {
  font-size: 12px;
  color: var(--text-secondary);
  text-decoration: none;
  transition: color 100ms ease;
}

.panel-link:hover {
  color: var(--text-primary);
}

/* Mood Selector */
.mood-selector {
  display: flex;
  gap: 6px;
  margin-bottom: 10px;
}

.mood-btn {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
  padding: 8px 4px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-surface);
  cursor: pointer;
  transition: all 120ms ease;
}

.mood-btn:hover {
  background: rgba(0, 0, 0, 0.02);
}

.mood-btn.active {
  border-color: var(--text-primary);
  background: rgba(0, 0, 0, 0.03);
}

.mood-symbol {
  font-family: var(--font-mono);
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.mood-label {
  font-size: 11px;
  color: var(--text-secondary);
}

/* Review Textarea */
.review-textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-canvas);
  color: var(--text-primary);
  font-family: var(--font-sans);
  font-size: 12.5px;
  line-height: 1.6;
  resize: vertical;
  outline: none;
  margin-bottom: 10px;
  transition: border-color 150ms ease;
}

.review-textarea:focus {
  border-color: var(--text-secondary);
}

.review-textarea::placeholder {
  color: var(--text-tertiary);
}

/* Notes List */
.notes-list {
  display: flex;
  flex-direction: column;
}

.note-item {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 9px 8px;
  border-radius: 6px;
  cursor: pointer;
  transition: background 100ms ease;
}

.note-item:hover {
  background: rgba(0, 0, 0, 0.02);
}

.note-icon {
  color: var(--text-tertiary);
  flex-shrink: 0;
  margin-top: 1px;
}

.note-body {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.note-title {
  font-size: 13px;
  font-weight: 480;
  color: var(--text-primary);
  line-height: 1.4;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.note-meta {
  font-size: 11px;
  color: var(--text-tertiary);
  margin-top: 2px;
}
</style>
