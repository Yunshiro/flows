<script setup lang="ts">
import { ref, computed } from 'vue'
import { marked } from 'marked'

const props = defineProps<{
  modelValue: string
  placeholder?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const showPreview = ref(false)
const textareaRef = ref<HTMLTextAreaElement>()

const renderedHtml = computed(() => {
  if (!props.modelValue) return ''
  return marked(props.modelValue, { breaks: true, gfm: true }) as string
})

function insertMarkdown(syntax: string) {
  const el = textareaRef.value
  if (!el) return
  const start = el.selectionStart
  const end = el.selectionEnd
  const text = props.modelValue
  let insertion: string

  switch (syntax) {
    case 'bold': insertion = `**${text.slice(start, end) || '加粗文本'}**`; break
    case 'italic': insertion = `*${text.slice(start, end) || '斜体文本'}*`; break
    case 'heading': insertion = `\n## ${text.slice(start, end) || '标题'}\n`; break
    case 'list': insertion = `\n- ${text.slice(start, end) || '列表项'}\n`; break
    case 'code': insertion = `\`${text.slice(start, end) || 'code'}\``; break
    case 'codeblock': insertion = `\n\`\`\`\n${text.slice(start, end) || '代码块'}\n\`\`\`\n`; break
    case 'link': insertion = `[${text.slice(start, end) || '链接'}](url)`; break
    default: return
  }

  const newText = text.slice(0, start) + insertion + text.slice(end)
  emit('update:modelValue', newText)
}

const toolbarItems = [
  { key: 'heading', label: 'H', title: '标题' },
  { key: 'bold', label: 'B', title: '加粗' },
  { key: 'italic', label: 'I', title: '斜体' },
  { key: 'code', label: '<>', title: '行内代码' },
  { key: 'codeblock', label: '{ }', title: '代码块' },
  { key: 'list', label: '--', title: '列表' },
  { key: 'link', label: '~', title: '链接' },
]
</script>

<template>
  <div class="md-editor">
    <div class="md-toolbar">
      <div class="md-toolbar-actions">
        <button
          v-for="item in toolbarItems"
          :key="item.key"
          class="md-toolbar-btn"
          :title="item.title"
          @click="insertMarkdown(item.key)"
        >{{ item.label }}</button>
      </div>
      <button
        :class="['md-toggle', { active: showPreview }]"
        @click="showPreview = !showPreview"
      >{{ showPreview ? '编辑' : '预览' }}</button>
    </div>

    <div class="md-body" :class="{ preview: showPreview }">
      <textarea
        v-if="!showPreview"
        ref="textareaRef"
        class="md-textarea"
        :value="modelValue"
        :placeholder="placeholder || '开始编写...'"
        @input="emit('update:modelValue', ($event.target as HTMLTextAreaElement).value)"
      />
      <div
        v-else
        class="md-preview"
        v-html="renderedHtml"
      />
    </div>
  </div>
</template>

<style scoped>
.md-editor {
  border: 1px solid var(--border);
  border-radius: 8px;
  overflow: hidden;
  background: var(--bg-surface);
  display: flex;
  flex-direction: column;
}

.md-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 8px;
  border-bottom: 1px solid var(--border);
  background: var(--editor-toolbar-bg);
}

.md-toolbar-actions {
  display: flex;
  gap: 2px;
}

.md-toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: var(--text-secondary);
  font-family: var(--font-mono);
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: all 100ms ease;
}

.md-toolbar-btn:hover {
  background: rgba(0, 0, 0, 0.06);
  color: #111111;
}

.md-toggle {
  padding: 4px 10px;
  border: 1px solid var(--border);
  border-radius: 4px;
  background: var(--bg-surface);
  color: var(--text-secondary);
  font-family: var(--font-sans);
  font-size: 11px;
  cursor: pointer;
  transition: all 100ms ease;
}

.md-toggle.active {
  background: var(--accent);
  color: var(--bg-surface);
  border-color: var(--accent);
}

.md-body {
  flex: 1;
  display: flex;
  min-height: 200px;
}

.md-body.preview {
  min-height: 200px;
}

.md-textarea {
  flex: 1;
  padding: 14px;
  border: none;
  font-family: var(--font-sans);
  font-size: 14px;
  line-height: 1.7;
  color: #111111;
  resize: vertical;
  outline: none;
  background: transparent;
}

.md-textarea::placeholder {
  color: var(--text-tertiary);
}

.md-preview {
  flex: 1;
  padding: 14px;
  overflow-y: auto;
  font-size: 14px;
  line-height: 1.7;
  color: #111111;
}

.md-preview :deep(h1) { font-size: 1.6em; margin: 0.6em 0 0.3em; font-weight: 600; }
.md-preview :deep(h2) { font-size: 1.3em; margin: 0.6em 0 0.3em; font-weight: 600; }
.md-preview :deep(h3) { font-size: 1.1em; margin: 0.5em 0 0.2em; font-weight: 600; }
.md-preview :deep(p) { margin: 0.4em 0; }
.md-preview :deep(ul), .md-preview :deep(ol) { padding-left: 1.5em; margin: 0.4em 0; }
.md-preview :deep(li) { margin: 0.2em 0; }
.md-preview :deep(code) {
  background: var(--tag-gray-bg);
  padding: 1px 5px;
  border-radius: 3px;
  font-family: var(--font-mono);
  font-size: 0.88em;
}
.md-preview :deep(pre) {
  background: var(--accent);
  color: #F9FAFB;
  padding: 14px;
  border-radius: 6px;
  overflow-x: auto;
  margin: 0.6em 0;
}
.md-preview :deep(pre code) {
  background: transparent;
  color: inherit;
  padding: 0;
}
.md-preview :deep(blockquote) {
  border-left: 3px solid #EAEAEA;
  padding-left: 12px;
  color: var(--text-secondary);
  margin: 0.6em 0;
}
.md-preview :deep(a) { color: #2563eb; }
.md-preview :deep(hr) { border: none; border-top: 1px solid var(--border); margin: 1em 0; }
.md-preview :deep(table) { border-collapse: collapse; width: 100%; margin: 0.6em 0; }
.md-preview :deep(th), .md-preview :deep(td) { border: 1px solid var(--border); padding: 6px 10px; text-align: left; font-size: 0.93em; }
</style>
