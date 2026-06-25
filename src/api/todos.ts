import { tauriInvoke } from './adapter'
import type { Todo } from '../stores/useTodoStore'

export const todoApi = {
  list: (date?: string) =>
    tauriInvoke<Todo[]>('list_todos', { date: date || null }),

  add: (data: { title: string; priority?: string; estMinutes?: number; tags?: string[] }) =>
    tauriInvoke<Todo>('add_todo', {
      title: data.title,
      priority: data.priority || 'medium',
      estMinutes: data.estMinutes || 30,
      tags: data.tags || [],
    }),

  update: (id: string, changes: Partial<Todo>) =>
    tauriInvoke<Todo>('update_todo', { id, changes }),

  toggle: (id: string) =>
    tauriInvoke<Todo>('toggle_todo', { id }),

  delete: (id: string) =>
    tauriInvoke<void>('delete_todo', { id }),

  reorder: (ids: string[]) =>
    tauriInvoke<void>('reorder_todos', { ids }),
}
