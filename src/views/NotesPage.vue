<script setup lang="ts">
import { useNotesStore } from '../stores/useNotesStore'
import { onMounted, ref } from 'vue'
import { marked } from 'marked'
import { computed } from 'vue'

const store = useNotesStore()
const activePath = ref<string | null>(null)
const editorContent = ref('')
const searchQuery = ref('')
const showPreview = ref(false)
const creating = ref(false)
const newNoteName = ref('')
const saved = ref(false)

onMounted(() => {
  store.fetchNotes()
})

const renderedHtml = computed(() => {
  if (!editorContent.value) return ''
  return marked(editorContent.value, { breaks: true, gfm: true }) as string
})

async function handleSelect(path: string) {
  activePath.value = path
  const note = await store.readNote(path)
  if (note) {
    editorContent.value = note.content
  }
  saved.value = false
  showPreview.value = false
}

async function handleSave() {
  if (!activePath.value) return
  await store.saveNote(activePath.value, editorContent.value)
  saved.value = true
  setTimeout(() => saved.value = false, 2000)
}

async function handleCreate() {
  const name = newNoteName.value.trim()
  if (!name) return
  await store.createNote('', name)
  await store.fetchNotes()
  newNoteName.value = ''
  creating.value = false
}

async function handleDelete() {
  if (!activePath.value) return
  await store.deleteNote(activePath.value)
  await store.fetchNotes()
  activePath.value = null
  editorContent.value = ''
}

async function handleSearch() {
  if (!searchQuery.value.trim()) return
  await store.searchNotes(searchQuery.value)
}

// Keyboard shortcuts
function onEditorKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key === 's') {
    e.preventDefault()
    handleSave()
  }
}

// Wiki link click handler
function handleWikiLinkClick(e: MouseEvent) {
  const target = e.target as HTMLElement
  if (target.tagName === 'A' && target.getAttribute('data-wiki')) {
    e.preventDefault()
    const linkPath = target.getAttribute('data-wiki') + '.md'
    // Navigate to linked note if it exists in tree
    for (const node of store.tree) {
      for (const child of node.children) {
        if (!child.isDir && child.path === linkPath) {
          handleSelect(linkPath)
          return
        }
      }
    }
  }
}

function insertMarkdown(syntax: string) {
  const el = document.querySelector('.editor-textarea') as HTMLTextAreaElement
  if (!el) return
  const start = el.selectionStart
  const end = el.selectionEnd
  const text = editorContent.value
  let insertion = ''
  switch (syntax) {
    case 'bold': insertion = `**${text.slice(start, end) || 'bold'}**`; break
    case 'italic': insertion = `*${text.slice(start, end) || 'italic'}*`; break
    case 'heading': insertion = `\n## ${text.slice(start, end) || 'Heading'}\n`; break
    case 'list': insertion = `\n- ${text.slice(start, end) || 'item'}\n`; break
    case 'code': insertion = `\`${text.slice(start, end) || 'code'}\``; break
    case 'codeblock': insertion = `\n\`\`\`\n${text.slice(start, end) || 'code'}\n\`\`\`\n`; break
    case 'link': insertion = `[[${text.slice(start, end) || 'note-name'}]]`; break
  }
  editorContent.value = text.slice(0, start) + insertion + text.slice(end)
}

const toolbarButtons = [
  { key: 'heading', label: 'H', title: '标题 (## )' },
  { key: 'bold', label: 'B', title: '加粗 (**)' },
  { key: 'italic', label: 'I', title: '斜体 (*)' },
  { key: 'code', label: '<>', title: '行内代码' },
  { key: 'codeblock', label: '{ }', title: '代码块' },
  { key: 'list', label: '--', title: '无序列表' },
  { key: 'link', label: '[[ ]]', title: 'Wiki 链接' },
]
</script>

<template>
  <div class="page-layout">
    <!-- File Tree Sidebar -->
    <aside class="file-tree">
      <div class="tree-header">
        <h3 class="tree-title">笔记</h3>
        <button class="tree-add" @click="creating = !creating" title="新建笔记">+</button>
      </div>

      <div v-if="creating" class="tree-create">
        <input
          v-model="newNoteName"
          class="tree-create-input"
          placeholder="笔记名称..."
          @keydown.enter="handleCreate"
          @keydown.escape="creating = false"
        />
      </div>

      <div class="tree-search">
        <input
          v-model="searchQuery"
          class="tree-search-input"
          placeholder="搜索笔记..."
          @keydown.enter="handleSearch"
        />
      </div>

      <div class="tree-list">
        <template v-for="node in store.tree" :key="node.path">
          <!-- Directory with children -->
          <template v-if="node.isDir">
            <div class="tree-dir">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2">
                <path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z"/>
              </svg>
              <span>{{ node.name }}</span>
            </div>
            <div
              v-for="child in node.children.filter(c => !c.isDir)"
              :key="child.path"
              :class="['tree-item', { active: activePath === child.path }]"
              @click="handleSelect(child.path)"
            >
              <svg class="tree-icon" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/><path d="M14 2v6h6"/>
              </svg>
              <span class="tree-name">{{ child.name }}</span>
            </div>
          </template>
          <!-- Top-level file -->
          <div
            v-else
            :class="['tree-item', { active: activePath === node.path }]"
            @click="handleSelect(node.path)"
          >
            <svg class="tree-icon" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/><path d="M14 2v6h6"/>
            </svg>
            <span class="tree-name">{{ node.name }}</span>
          </div>
        </template>
      </div>
      <p v-if="store.tree.length === 0 && !store.loading" class="tree-empty">暂无笔记</p>
    </aside>

    <!-- Editor Area -->
    <section class="editor-area" v-if="activePath">
      <div class="editor-toolbar">
        <div class="editor-toolbar-left">
          <button
            v-for="btn in toolbarButtons" :key="btn.key"
            class="toolbar-btn"
            :title="btn.title"
            @click="insertMarkdown(btn.key)"
          >{{ btn.label }}</button>
        </div>
        <div class="editor-toolbar-right">
          <button :class="['toggle-preview', { active: showPreview }]" @click="showPreview = !showPreview">
            {{ showPreview ? '编辑' : '预览' }}
          </button>
          <button class="btn-save" @click="handleSave">{{ saved ? '已保存' : '保存' }}</button>
          <button class="btn-delete" @click="handleDelete">删除</button>
        </div>
      </div>

      <div class="editor-body">
        <textarea
          v-if="!showPreview"
          v-model="editorContent"
          class="editor-textarea"
          placeholder="开始编写..."
          @keydown="onEditorKeydown"
        />
        <div
          v-else
          class="editor-preview"
          v-html="renderedHtml"
          @click="handleWikiLinkClick"
        />
      </div>

      <div class="editor-footer">
        <span class="editor-path">{{ activePath }}</span>
        <span class="editor-hint">Ctrl+S 保存</span>
      </div>
    </section>

    <!-- Empty State -->
    <section class="editor-area editor-empty" v-else>
      <div class="empty-state">
        <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.2">
          <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/><path d="M14 2v6h6M16 13H8M16 17H8M10 9H8"/>
        </svg>
        <p>选择左侧笔记或新建笔记开始</p>
      </div>
    </section>
  </div>
</template>

<style scoped>
.page-layout { display: flex; height: calc(100vh - 56px); margin: -28px; }
.file-tree { width: 220px; border-right: 1px solid var(--border); background: var(--bg-surface); display: flex; flex-direction: column; overflow: hidden; }
.tree-header { display: flex; align-items: center; justify-content: space-between; padding: 16px 14px 8px; }
.tree-title { font-size: 14px; font-weight: 600; }
.tree-add { width: 24px; height: 24px; border: 1px solid var(--border); border-radius: 5px; background: var(--bg-surface); color: var(--text-secondary); font-size: 15px; cursor: pointer; display: flex; align-items: center; justify-content: center; }
.tree-add:hover { background: rgba(0,0,0,0.04); }

.tree-create { padding: 0 14px 8px; }
.tree-create-input { width: 100%; padding: 5px 8px; border: 1px solid #111111; border-radius: 4px; font-family: var(--font-sans); font-size: 12px; outline: none; }

.tree-search { padding: 0 14px 8px; }
.tree-search-input { width: 100%; padding: 5px 8px; border: 1px solid var(--border); border-radius: 5px; background: var(--bg-canvas); font-family: var(--font-sans); font-size: 12px; outline: none; }
.tree-search-input:focus { border-color: var(--text-secondary); }

.tree-list { flex: 1; overflow-y: auto; padding: 4px 10px; }
.tree-node { margin-bottom: 2px; }
.tree-dir { display: flex; align-items: center; gap: 6px; padding: 6px 8px; font-size: 12.5px; font-weight: 500; color: var(--text-secondary); }
.tree-item { display: flex; align-items: center; gap: 7px; padding: 6px 8px 6px 20px; border-radius: 5px; cursor: pointer; color: var(--text-secondary); font-size: 12.5px; transition: all 80ms ease; }
.tree-item:hover { background: rgba(0,0,0,0.03); color: #111111; }
.tree-item.active { background: rgba(0,0,0,0.05); color: #111111; font-weight: 500; }
.tree-name { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.tree-icon { flex-shrink: 0; opacity: 0.5; }
.tree-empty { font-size: 12px; color: var(--text-tertiary); padding: 16px 14px; }

/* Editor */
.editor-area { flex: 1; display: flex; flex-direction: column; background: var(--bg-surface); overflow: hidden; }
.editor-empty { background: var(--bg-canvas); }

.editor-toolbar { display: flex; align-items: center; justify-content: space-between; padding: 8px 14px; border-bottom: 1px solid var(--border); background: var(--editor-toolbar-bg); flex-shrink: 0; }
.editor-toolbar-left { display: flex; gap: 2px; }
.editor-toolbar-right { display: flex; gap: 8px; align-items: center; }

.toolbar-btn { display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; border: none; border-radius: 4px; background: transparent; color: var(--text-secondary); font-family: var(--font-mono); font-size: 11.5px; font-weight: 600; cursor: pointer; }
.toolbar-btn:hover { background: rgba(0,0,0,0.06); color: #111111; }

.toggle-preview { padding: 4px 10px; border: 1px solid var(--border); border-radius: 4px; background: var(--bg-surface); color: var(--text-secondary); font-family: var(--font-sans); font-size: 11px; cursor: pointer; }
.toggle-preview.active { background: var(--accent); color: var(--bg-surface); border-color: var(--accent); }

.btn-save { padding: 5px 14px; border: none; border-radius: 5px; background: var(--accent); color: var(--bg-surface); font-family: var(--font-sans); font-size: 12px; cursor: pointer; }
.btn-save:hover { background: var(--accent-hover); }

.btn-delete { padding: 5px 10px; border: 1px solid var(--border); border-radius: 5px; background: var(--bg-surface); color: var(--text-tertiary); font-family: var(--font-sans); font-size: 12px; cursor: pointer; }
.btn-delete:hover { background: var(--tag-red-bg); color: var(--tag-red-text); border-color: var(--tag-red-bg); }

.editor-body { flex: 1; display: flex; overflow: hidden; }
.editor-textarea { flex: 1; padding: 18px 20px; border: none; font-family: var(--font-sans); font-size: 14px; line-height: 1.7; color: #111111; resize: none; outline: none; }
.editor-textarea::placeholder { color: var(--text-tertiary); }
.editor-preview { flex: 1; padding: 18px 20px; overflow-y: auto; font-size: 14px; line-height: 1.7; }
.editor-preview :deep(h1) { font-size: 1.6em; margin: 0.5em 0 0.3em; font-weight: 600; }
.editor-preview :deep(h2) { font-size: 1.3em; margin: 0.5em 0 0.3em; font-weight: 600; }
.editor-preview :deep(h3) { font-size: 1.1em; margin: 0.4em 0 0.2em; font-weight: 600; }
.editor-preview :deep(p) { margin: 0.4em 0; }
.editor-preview :deep(ul), .editor-preview :deep(ol) { padding-left: 1.5em; margin: 0.4em 0; }
.editor-preview :deep(code) { background: var(--tag-gray-bg); padding: 1px 5px; border-radius: 3px; font-family: var(--font-mono); font-size: 0.88em; }
.editor-preview :deep(pre) { background: var(--accent); color: #F9FAFB; padding: 14px; border-radius: 6px; overflow-x: auto; margin: 0.6em 0; }
.editor-preview :deep(pre code) { background: transparent; color: inherit; padding: 0; }
.editor-preview :deep(blockquote) { border-left: 3px solid #EAEAEA; padding-left: 12px; color: var(--text-secondary); margin: 0.6em 0; }
.editor-preview :deep(a) { color: #2563eb; cursor: pointer; }
.editor-preview :deep(table) { border-collapse: collapse; width: 100%; }
.editor-preview :deep(th), .editor-preview :deep(td) { border: 1px solid var(--border); padding: 6px 10px; font-size: 0.93em; }
.editor-preview :deep(hr) { border: none; border-top: 1px solid var(--border); margin: 1em 0; }

.editor-footer { display: flex; align-items: center; justify-content: space-between; padding: 6px 16px; border-top: 1px solid var(--border); background: var(--editor-toolbar-bg); flex-shrink: 0; }
.editor-path { font-family: var(--font-mono); font-size: 11px; color: var(--text-tertiary); }
.editor-hint { font-size: 10px; color: var(--text-disabled); }

.empty-state { display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 12px; height: 100%; color: var(--text-tertiary); font-size: 13px; }
</style>
