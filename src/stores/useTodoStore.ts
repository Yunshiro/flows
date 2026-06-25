import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { todoApi } from '../api/todos'

export interface Todo {
  id: string
  title: string
  priority: 'high' | 'medium' | 'low'
  tags: string[]
  done: boolean
  estMinutes: number
  actualMinutes: number | null
  date: string
  createdAt: string
  completedAt: string | null
  sortOrder: number
}

export const useTodoStore = defineStore('todos', () => {
  const items = ref<Todo[]>([])
  const loading = ref(false)

  const todayTodos = computed(() => {
    const today = new Date().toISOString().slice(0, 10)
    return items.value
      .filter(t => t.date === today)
      .sort((a, b) => {
        if (a.done !== b.done) return a.done ? 1 : -1
        return a.sortOrder - b.sortOrder
      })
  })

  const completedCount = computed(() => todayTodos.value.filter(t => t.done).length)
  const totalCount = computed(() => todayTodos.value.length)

  async function fetchTodos(date?: string) {
    loading.value = true
    try {
      items.value = await todoApi.list(date)
    } catch (e) {
      console.error('fetchTodos failed:', e)
    } finally {
      loading.value = false
    }
  }

  async function addTodo(data: Partial<Todo>) {
    try {
      const todo = await todoApi.add({
        title: data.title || '',
        priority: data.priority || 'medium',
        estMinutes: data.estMinutes || 30,
        tags: data.tags || [],
      })
      items.value.unshift(todo)
      return todo
    } catch (e) {
      console.error('addTodo failed:', e)
      return null
    }
  }

  async function updateTodo(id: string, changes: Partial<Todo>) {
    try {
      const updated = await todoApi.update(id, changes)
      const idx = items.value.findIndex(t => t.id === id)
      if (idx !== -1) items.value[idx] = updated
      return updated
    } catch (e) {
      console.error('updateTodo failed:', e)
      return null
    }
  }

  async function toggleTodo(id: string) {
    try {
      const todo = await todoApi.toggle(id)
      const idx = items.value.findIndex(t => t.id === id)
      if (idx !== -1) items.value[idx] = todo
      return todo
    } catch (e) {
      console.error('toggleTodo failed:', e)
      return null
    }
  }

  async function removeTodo(id: string) {
    try {
      await todoApi.delete(id)
      items.value = items.value.filter(t => t.id !== id)
    } catch (e) {
      console.error('removeTodo failed:', e)
    }
  }

  async function reorderTodos(ids: string[]) {
    try {
      await todoApi.reorder(ids)
      ids.forEach((id, i) => {
        const todo = items.value.find(t => t.id === id)
        if (todo) todo.sortOrder = i
      })
      items.value.sort((a, b) => {
        const ai = ids.indexOf(a.id)
        const bi = ids.indexOf(b.id)
        return (ai === -1 ? 999 : ai) - (bi === -1 ? 999 : bi)
      })
    } catch (e) {
      console.error('reorderTodos failed:', e)
    }
  }

  return {
    items,
    loading,
    todayTodos,
    completedCount,
    totalCount,
    fetchTodos,
    addTodo,
    updateTodo,
    toggleTodo,
    removeTodo,
    reorderTodos,
  }
})
