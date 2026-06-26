<script setup lang="ts">
import { useTodoStore } from '../stores/useTodoStore'
import { onMounted, ref, computed } from 'vue'
import TagPicker from '../components/TagPicker.vue'
import InlineEdit from '../components/InlineEdit.vue'

const store = useTodoStore()
const newTitle = ref('')
const filterStatus = ref<'all' | 'active' | 'done'>('all')
const filterPriority = ref<'all' | 'high' | 'medium' | 'low'>('all')

onMounted(() => {
  store.fetchTodos()
})

const filteredTodos = computed(() => {
  let list = store.todayTodos
  if (filterStatus.value === 'active') list = list.filter(t => !t.done)
  if (filterStatus.value === 'done') list = list.filter(t => t.done)
  if (filterPriority.value !== 'all') list = list.filter(t => t.priority === filterPriority.value)
  return list
})

async function handleAdd() {
  const title = newTitle.value.trim()
  if (!title) return
  await store.addTodo({ title })
  newTitle.value = ''
}

async function handleToggle(id: string) {
  await store.toggleTodo(id)
}

async function handleDelete(id: string) {
  await store.removeTodo(id)
}

async function handleTitleSave(todoId: string, newTitle: string) {
  await store.updateTodo(todoId, { title: newTitle })
}

async function handlePriorityChange(todoId: string, priority: string) {
  await store.updateTodo(todoId, { priority: priority as 'high' | 'medium' | 'low' })
}

async function handleTagsChange(todoId: string, tags: string[]) {
  await store.updateTodo(todoId, { tags })
}

const draggedIndex = ref<number | null>(null)

function onDragStart(index: number) {
  draggedIndex.value = index
}

function onDragOver(e: DragEvent) {
  e.preventDefault()
}

function onDrop(targetIndex: number) {
  if (draggedIndex.value === null || draggedIndex.value === targetIndex) return
  const ids = filteredTodos.value.map(t => t.id)
  const [moved] = ids.splice(draggedIndex.value, 1)
  ids.splice(targetIndex, 0, moved)
  store.reorderTodos(ids)
  draggedIndex.value = null
}

function onDragEnd() {
  draggedIndex.value = null
}
</script>

<template>
  <div class="page">
    <header class="page-header">
      <div>
        <h2 class="page-title">今日待办</h2>
        <p class="page-subtitle">{{ new Date().toLocaleDateString('zh-CN', { weekday: 'long', month: 'long', day: 'numeric' }) }}</p>
      </div>
      <span class="badge">{{ store.completedCount }}/{{ store.totalCount }}</span>
    </header>

    <!-- Quick Add -->
    <div class="quick-add">
      <input
        v-model="newTitle"
        class="quick-add-input"
        placeholder="添加待办事项..."
        @keydown.enter="handleAdd"
      />
      <button class="btn-add" @click="handleAdd" :disabled="!newTitle.trim()">+</button>
    </div>

    <!-- Filters -->
    <div class="filters">
      <div class="filter-group">
        <button :class="['filter-btn', { active: filterStatus === 'all' }]" @click="filterStatus = 'all'">全部</button>
        <button :class="['filter-btn', { active: filterStatus === 'active' }]" @click="filterStatus = 'active'">未完成</button>
        <button :class="['filter-btn', { active: filterStatus === 'done' }]" @click="filterStatus = 'done'">已完成</button>
      </div>
      <div class="filter-group">
        <select v-model="filterPriority" class="filter-select">
          <option value="all">全部优先级</option>
          <option value="high">High</option>
          <option value="medium">Mid</option>
          <option value="low">Low</option>
        </select>
      </div>
    </div>

    <!-- Todo List -->
    <div class="todo-list">
      <div
        v-for="(todo, index) in filteredTodos"
        :key="todo.id"
        :class="['todo-item', { done: todo.done, dragging: draggedIndex === index }]"
        draggable="true"
        @dragstart="onDragStart(index)"
        @dragover="onDragOver"
        @drop="onDrop(index)"
        @dragend="onDragEnd"
      >
        <span class="todo-grip">
          <svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor" opacity="0.2">
            <circle cx="3" cy="2" r="1"/><circle cx="9" cy="2" r="1"/>
            <circle cx="3" cy="6" r="1"/><circle cx="9" cy="6" r="1"/>
            <circle cx="3" cy="10" r="1"/><circle cx="9" cy="10" r="1"/>
          </svg>
        </span>

        <button :class="['todo-check', { checked: todo.done }]" @click="handleToggle(todo.id)">
          <svg v-if="todo.done" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
            <path d="M5 13l4 4L19 7"/>
          </svg>
        </button>

        <div class="todo-body">
          <InlineEdit
            :model-value="todo.title"
            :class="{ done: todo.done }"
            @save="(v: string) => handleTitleSave(todo.id, v)"
          >
            <span :class="['todo-title', { done: todo.done }]">{{ todo.title }}</span>
          </InlineEdit>

          <div class="todo-meta">
            <select
              class="priority-select"
              :value="todo.priority"
              @change="handlePriorityChange(todo.id, ($event.target as HTMLSelectElement).value)"
            >
              <option value="high">High</option>
              <option value="medium">Mid</option>
              <option value="low">Low</option>
            </select>
            <span :class="['tag', `tag--${todo.priority}`]">{{ todo.priority === 'high' ? 'High' : todo.priority === 'medium' ? 'Mid' : 'Low' }}</span>
            <span class="todo-time">{{ todo.estMinutes }}m</span>
          </div>

          <div class="todo-tags">
            <TagPicker
              :model-value="todo.tags"
              @update:model-value="(tags: string[]) => handleTagsChange(todo.id, tags)"
            />
          </div>
        </div>

        <button class="todo-delete" @click="handleDelete(todo.id)">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 6L6 18M6 6l12 12"/>
          </svg>
        </button>
      </div>
    </div>

    <p v-if="filteredTodos.length === 0 && !store.loading" class="empty">
      {{ filterStatus !== 'all' || filterPriority !== 'all' ? '无匹配的待办事项' : '暂无待办事项，输入上方添加第一条' }}
    </p>
  </div>
</template>

<style scoped>
.page { max-width: 700px; }
.page-header { display: flex; align-items: flex-start; justify-content: space-between; margin-bottom: 16px; }
.page-title { font-size: 28px; font-weight: 700; letter-spacing: -0.035em; }
.page-subtitle { font-size: 13px; color: var(--text-secondary); margin-top: 2px; }
.badge { font-family: var(--font-mono); font-size: 12px; color: var(--text-secondary); padding: 2px 8px; background: var(--tag-gray-bg); border-radius: 999px; }

.quick-add { display: flex; gap: 8px; margin-bottom: 14px; }
.quick-add-input { flex: 1; padding: 8px 12px; border: 1px solid var(--border); border-radius: 6px; background: var(--bg-surface); font-family: var(--font-sans); font-size: 13px; outline: none; }
.quick-add-input:focus { border-color: var(--text-secondary); }
.quick-add-input::placeholder { color: var(--text-tertiary); }
.btn-add { width: 34px; height: 34px; border: 1px solid var(--border); border-radius: 6px; background: var(--bg-surface); font-size: 18px; color: var(--text-secondary); cursor: pointer; flex-shrink: 0; }
.btn-add:hover:not(:disabled) { background: rgba(0,0,0,0.04); }
.btn-add:disabled { opacity: 0.4; cursor: default; }

/* Filters */
.filters { display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px; }
.filter-group { display: flex; gap: 2px; }
.filter-btn { padding: 4px 10px; border: none; border-radius: 5px; background: transparent; color: var(--text-secondary); font-family: var(--font-sans); font-size: 12px; cursor: pointer; transition: all 100ms ease; }
.filter-btn:hover { color: var(--text-primary); }
.filter-btn.active { background: rgba(0,0,0,0.06); color: var(--text-primary); font-weight: 500; }
.filter-select { padding: 4px 8px; border: 1px solid var(--border); border-radius: 5px; background: var(--bg-surface); color: var(--text-secondary); font-family: var(--font-sans); font-size: 12px; outline: none; cursor: pointer; }
.filter-select:focus { border-color: var(--text-secondary); }

/* Todo List */
.todo-list { display: flex; flex-direction: column; }
.todo-item { display: flex; align-items: flex-start; gap: 10px; padding: 10px 8px; border-radius: 6px; transition: background 100ms ease, opacity 150ms ease; }
.todo-item:hover { background: rgba(0,0,0,0.02); }
.todo-item:hover .todo-grip { opacity: 1; }
.todo-item:hover .todo-delete { opacity: 1; }
.todo-item.dragging { opacity: 0.4; }

.todo-grip { opacity: 0; padding-top: 5px; cursor: grab; transition: opacity 150ms ease; flex-shrink: 0; }
.todo-check { display: flex; align-items: center; justify-content: center; width: 18px; height: 18px; margin-top: 3px; border: 1.5px solid var(--text-disabled); border-radius: 4px; background: transparent; cursor: pointer; flex-shrink: 0; transition: all 150ms ease; }
.todo-check:hover { border-color: var(--text-secondary); }
.todo-check.checked { background: var(--accent); border-color: var(--accent); color: var(--bg-surface); }

.todo-body { flex: 1; min-width: 0; }
.todo-title { font-size: 13.5px; font-weight: 400; color: var(--text-primary); display: block; }
.todo-title.done { color: var(--text-tertiary); text-decoration: line-through; }
.todo-meta { display: flex; align-items: center; gap: 6px; margin-top: 4px; }
.todo-time { font-family: var(--font-mono); font-size: 11px; color: var(--text-tertiary); margin-left: auto; }
.todo-tags { margin-top: 4px; }

.priority-select {
  padding: 1px 6px;
  border: 1px solid var(--border);
  border-radius: 4px;
  background: var(--bg-surface);
  color: var(--text-secondary);
  font-family: var(--font-sans);
  font-size: 11px;
  outline: none;
  cursor: pointer;
  opacity: 0;
  transition: opacity 100ms ease;
}
.todo-item:hover .priority-select { opacity: 1; }

.todo-delete { opacity: 0; display: flex; align-items: center; justify-content: center; width: 24px; height: 24px; border: none; border-radius: 4px; background: transparent; color: var(--text-tertiary); cursor: pointer; flex-shrink: 0; transition: all 100ms ease; }
.todo-delete:hover { background: var(--tag-red-bg); color: var(--tag-red-text); }

.tag { display: inline-flex; padding: 1px 7px; border-radius: 999px; font-size: 10.5px; font-weight: 500; letter-spacing: 0.04em; text-transform: uppercase; }
.tag--high { background: var(--tag-red-bg); color: var(--tag-red-text); }
.tag--medium { background: var(--tag-yellow-bg); color: var(--tag-yellow-text); }
.tag--low { background: var(--tag-blue-bg); color: var(--tag-blue-text); }

.empty { color: var(--text-tertiary); text-align: center; padding: 40px 0; font-size: 13px; }
</style>
