/** Detect whether running inside Tauri */
let _isTauri: boolean | null = null

export function isTauri(): boolean {
  if (_isTauri !== null) return _isTauri
  try {
    _isTauri = !!(window as any).__TAURI_INTERNALS__
  } catch {
    _isTauri = false
  }
  return _isTauri
}

/** Tauri invoke wrapper with localStorage fallback for browser dev mode */
export async function tauriInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (isTauri()) {
    const { invoke } = await import('@tauri-apps/api/core')
    return invoke<T>(cmd, args)
  }

  // Browser-only fallback using localStorage
  switch (cmd) {
    case 'list_todos': {
      const date = (args?.date as string) || new Date().toISOString().slice(0, 10)
      const all = JSON.parse(localStorage.getItem('flows_todos') || '[]') as any[]
      return all.filter((t: any) => t.date === date) as T
    }
    case 'add_todo': {
      const all = JSON.parse(localStorage.getItem('flows_todos') || '[]') as any[]
      const today = new Date().toISOString().slice(0, 10)
      const todo = {
        id: crypto.randomUUID(),
        title: args?.title || '',
        priority: args?.priority || 'medium',
        tags: args?.tags || [],
        done: false,
        estMinutes: args?.estMinutes || 30,
        actualMinutes: null,
        date: today,
        createdAt: new Date().toISOString(),
        completedAt: null,
        sortOrder: 0,
      }
      all.unshift(todo)
      localStorage.setItem('flows_todos', JSON.stringify(all))
      return todo as T
    }
    case 'update_todo': {
      const all = JSON.parse(localStorage.getItem('flows_todos') || '[]') as any[]
      const changes = (args?.changes || {}) as any
      const idx = all.findIndex((t: any) => t.id === args?.id)
      if (idx !== -1) {
        if (changes.done) {
          all[idx].done = true
          all[idx].completedAt = new Date().toISOString()
        } else if (changes.done === false) {
          all[idx].done = false
          all[idx].completedAt = null
        }
        if (changes.title) all[idx].title = changes.title
        if (changes.priority) all[idx].priority = changes.priority
        if (changes.tags) all[idx].tags = changes.tags
        if (changes.estMinutes) all[idx].estMinutes = changes.estMinutes
        if (changes.sortOrder !== undefined) all[idx].sortOrder = changes.sortOrder
        localStorage.setItem('flows_todos', JSON.stringify(all))
        return all[idx] as T
      }
      return null as T
    }
    case 'toggle_todo': {
      const all = JSON.parse(localStorage.getItem('flows_todos') || '[]') as any[]
      const idx = all.findIndex((t: any) => t.id === args?.id)
      if (idx !== -1) {
        all[idx].done = !all[idx].done
        all[idx].completedAt = all[idx].done ? new Date().toISOString() : null
        localStorage.setItem('flows_todos', JSON.stringify(all))
        return all[idx] as T
      }
      return null as T
    }
    case 'delete_todo': {
      let all = JSON.parse(localStorage.getItem('flows_todos') || '[]') as any[]
      all = all.filter((t: any) => t.id !== args?.id)
      localStorage.setItem('flows_todos', JSON.stringify(all))
      return undefined as T
    }
    case 'reorder_todos': {
      const all = JSON.parse(localStorage.getItem('flows_todos') || '[]') as any[]
      const ids = (args?.ids || []) as string[]
      ids.forEach((id, i) => {
        const t = all.find((x: any) => x.id === id)
        if (t) t.sortOrder = i
      })
      all.sort((a: any, b: any) => {
        const ai = ids.indexOf(a.id)
        const bi = ids.indexOf(b.id)
        return (ai === -1 ? 999 : ai) - (bi === -1 ? 999 : bi)
      })
      localStorage.setItem('flows_todos', JSON.stringify(all))
      return undefined as T
    }
    case 'list_reviews':
    case 'get_review': {
      const all = JSON.parse(localStorage.getItem('flows_reviews') || '[]') as any[]
      if (cmd === 'get_review') {
        const date = args?.date as string
        return (all.find((r: any) => r.date === date) || null) as T
      }
      return all as T
    }
    case 'save_review': {
      const all = JSON.parse(localStorage.getItem('flows_reviews') || '[]') as any[]
      const today = new Date().toISOString().slice(0, 10)
      const now = new Date().toISOString()
      const review = {
        date: today,
        content: args?.content || '',
        mood: args?.mood || 'normal',
        linkedTodoIds: args?.linkedTodoIds || [],
        createdAt: now,
        updatedAt: now,
      }
      const idx = all.findIndex((r: any) => r.date === today)
      if (idx !== -1) {
        all[idx] = review
      } else {
        all.unshift(review)
      }
      localStorage.setItem('flows_reviews', JSON.stringify(all))
      return review as T
    }
    case 'get_settings': {
      const s = localStorage.getItem('flows_settings')
      if (s) return JSON.parse(s) as T
      return {
        notesPath: '',
        gitRemoteUrl: '',
        gitBranch: '',
        autoSyncEnabled: false,
        autoSyncIntervalMinutes: 30,
        theme: 'system',
        llmProvider: 'claude',
        llmApiKey: '',
        llmModel: 'claude-opus-4-7',
      } as T
    }
    case 'save_settings': {
      localStorage.setItem('flows_settings', JSON.stringify(args?.settings))
      return undefined as T
    }
    case 'list_llm_configs': {
      return JSON.parse(localStorage.getItem('flows_llm_configs') || '[]') as T
    }
    case 'save_llm_config': {
      const configs = JSON.parse(localStorage.getItem('flows_llm_configs') || '[]') as any[]
      const cfg = {
        id: args?.id || crypto.randomUUID(),
        name: args?.name || '',
        baseUrl: args?.baseUrl || '',
        apiKey: args?.apiKey || '',
        model: args?.model || '',
        isDefault: args?.isDefault || false,
        createdAt: new Date().toISOString(),
      }
      if (cfg.isDefault) configs.forEach((c: any) => c.isDefault = false)
      const idx = configs.findIndex((c: any) => c.id === cfg.id)
      if (idx >= 0) configs[idx] = cfg; else configs.push(cfg)
      localStorage.setItem('flows_llm_configs', JSON.stringify(configs))
      return cfg as T
    }
    case 'delete_llm_config': {
      let configs = JSON.parse(localStorage.getItem('flows_llm_configs') || '[]') as any[]
      configs = configs.filter((c: any) => c.id !== args?.id)
      localStorage.setItem('flows_llm_configs', JSON.stringify(configs))
      return undefined as T
    }
    case 'get_llm_config': {
      const configs = JSON.parse(localStorage.getItem('flows_llm_configs') || '[]') as any[]
      return (configs.find((c: any) => c.id === args?.id) || null) as T
    }
    case 'test_llm_connection':
      return '浏览器模式不支持 API 测试，请使用 Tauri 桌面应用。' as T
    case 'generate_weekly_summary': {
      const todos = JSON.parse(localStorage.getItem('flows_todos') || '[]') as any[]
      const completed = todos.filter((t: any) => t.done).length
      const total = todos.length
      return `## 本周总结\n\n完成率: ${total > 0 ? Math.round(completed / total * 100) : 0}% (${completed}/${total})\n\n*由 Flows 生成（离线模式）*` as T
    }
    case 'list_notes': {
      const index = JSON.parse(localStorage.getItem('flows_note_index') || '[]') as any[]
      const tree: any[] = []
      const dirMap = new Map<string, any>()
      for (const entry of index) {
        const parts = entry.path.split('/')
        let current = tree
        for (let i = 0; i < parts.length - 1; i++) {
          const dirPath = parts.slice(0, i + 1).join('/')
          if (!dirMap.has(dirPath)) {
            const dirNode = { name: parts[i], path: dirPath, isDir: true, children: [] }
            current.push(dirNode)
            dirMap.set(dirPath, dirNode)
          }
          current = dirMap.get(dirPath)!.children
        }
        current.push({ name: entry.title, path: entry.path, isDir: false, children: [] })
      }
      return tree as T
    }
    case 'read_note': {
      const key = 'flows_note_' + (args?.path || '')
      const data = localStorage.getItem(key)
      if (!data) return null as T
      return JSON.parse(data) as T
    }
    case 'save_note': {
      const path = (args?.path || '') as string
      const content = (args?.content || '') as string
      const key = 'flows_note_' + path
      const existing = JSON.parse(localStorage.getItem(key) || 'null')
      const now = new Date().toISOString()
      const note = {
        path,
        title: path.split('/').pop()?.replace(/\.md$/, '') || 'Untitled',
        content,
        tags: [] as string[],
        links: [] as string[],
        createdAt: existing?.createdAt || now,
        updatedAt: now,
      }
      localStorage.setItem(key, JSON.stringify(note))
      // Update index
      const index = JSON.parse(localStorage.getItem('flows_note_index') || '[]') as any[]
      const idx = index.findIndex((e: any) => e.path === path)
      if (idx === -1) {
        index.push({ path, title: note.title, updatedAt: now })
      } else {
        index[idx].updatedAt = now
      }
      localStorage.setItem('flows_note_index', JSON.stringify(index))
      return note as T
    }
    case 'create_note': {
      const dir = (args?.dir || '') as string
      const title = (args?.title || 'Untitled') as string
      const path = dir ? `${dir}/${title}.md` : `${title}.md`
      const now = new Date().toISOString()
      const note = {
        path,
        title,
        content: '',
        tags: [],
        links: [],
        createdAt: now,
        updatedAt: now,
      }
      localStorage.setItem('flows_note_' + path, JSON.stringify(note))
      const index = JSON.parse(localStorage.getItem('flows_note_index') || '[]') as any[]
      if (!index.find((e: any) => e.path === path)) {
        index.push({ path, title, updatedAt: now })
      }
      localStorage.setItem('flows_note_index', JSON.stringify(index))
      return note as T
    }
    case 'delete_note': {
      const path = (args?.path || '') as string
      localStorage.removeItem('flows_note_' + path)
      const index = JSON.parse(localStorage.getItem('flows_note_index') || '[]') as any[]
      localStorage.setItem('flows_note_index', JSON.stringify(index.filter((e: any) => e.path !== path)))
      return undefined as T
    }
    case 'search_notes': {
      const query = ((args?.query || '') as string).toLowerCase()
      const index = JSON.parse(localStorage.getItem('flows_note_index') || '[]') as any[]
      const results: any[] = []
      for (const entry of index) {
        if (entry.title.toLowerCase().includes(query)) {
          const data = localStorage.getItem('flows_note_' + entry.path)
          if (data) results.push(JSON.parse(data))
        }
      }
      return results as T
    }
    case 'git_list_files':
      return [] as T
    case 'git_get_remote':
      return '' as T
    case 'git_init':
      return '浏览器模式不支持 Git 操作' as T
    case 'git_set_remote':
      return '浏览器模式不支持 Git 操作' as T
    case 'git_checkout_branch':
      return '浏览器模式不支持 Git 操作' as T
    case 'git_list_branches':
      return [] as T
    case 'git_push':
      return '浏览器模式不支持 Git 操作' as T
    case 'git_pull':
      return '浏览器模式不支持 Git 操作' as T
    case 'git_status':
      return '浏览器模式（无 Git）' as T
    default:
      return undefined as T
  }
}
