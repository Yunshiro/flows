<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { noteApi } from '../api/notes'

const props = defineProps<{
  branch: string
}>()

const emit = defineEmits<{
  close: []
  pushed: []
}>()

interface GitFile {
  status: string
  path: string
  checked: boolean
}

const files = ref<GitFile[]>([])
const loading = ref(true)
const pushing = ref(false)
const selectAll = ref(true)

onMounted(async () => {
  try {
    const list = await noteApi.gitListFiles()
    files.value = list.map(f => ({ ...f, checked: true }))
  } catch {
    // ignore
  }
  loading.value = false
})

const selectedCount = computed(() => files.value.filter(f => f.checked).length)

function toggleAll() {
  selectAll.value = !selectAll.value
  files.value.forEach(f => f.checked = selectAll.value)
}

function statusLabel(s: string): string {
  if (s.startsWith('M')) return '修改'
  if (s.startsWith('A')) return '新增'
  if (s.startsWith('D')) return '删除'
  if (s.startsWith('R')) return '重命名'
  if (s.startsWith('?')) return '新增'
  return s || '变更'
}

async function handlePush() {
  pushing.value = true
  try {
    await noteApi.gitPush(props.branch)
    emit('pushed')
  } catch (e) {
    alert('推送失败: ' + String(e))
  } finally {
    pushing.value = false
  }
}
</script>

<template>
  <div class="overlay" @click.self="emit('close')">
    <div class="dialog">
      <div class="dialog-header">
        <h3 class="dialog-title">推送到远程仓库</h3>
        <button class="dialog-close" @click="emit('close')">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 6L6 18M6 6l12 12"/>
          </svg>
        </button>
      </div>

      <p class="dialog-hint" v-if="files.length === 0 && !loading">
        没有需要推送到 {{ branch }} 的文件变更。
      </p>

      <div class="dialog-body" v-if="files.length > 0">
        <div class="file-toolbar">
          <label class="select-all">
            <input type="checkbox" :checked="selectAll" @change="toggleAll" />
            <span>全选 ({{ selectedCount }}/{{ files.length }})</span>
          </label>
        </div>
        <div class="file-list">
          <div v-for="f in files" :key="f.path" class="file-item">
            <input type="checkbox" v-model="f.checked" />
            <span :class="['file-status', 'status-' + (f.status[0] || '?')]">{{ statusLabel(f.status) }}</span>
            <span class="file-path">{{ f.path }}</span>
          </div>
        </div>
      </div>

      <div class="dialog-footer">
        <button class="btn-cancel" @click="emit('close')">取消</button>
        <button
          class="btn-push"
          :disabled="pushing || selectedCount === 0"
          @click="handlePush"
        >
          {{ pushing ? '推送中...' : `推送 (${selectedCount} 个文件)` }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  background: rgba(12, 14, 12, 0.42);
  backdrop-filter: blur(10px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.dialog {
  width: 480px;
  max-height: 70vh;
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: var(--radius-xl);
  display: flex;
  flex-direction: column;
  box-shadow: var(--shadow-dialog);
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 20px 12px;
}

.dialog-title {
  font-size: 16px;
  font-weight: 600;
}

.dialog-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 5px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
}

.dialog-close:hover {
  background: var(--surface-hover);
}

.dialog-hint {
  padding: 20px;
  text-align: center;
  color: var(--text-tertiary);
  font-size: 13px;
}

.dialog-body {
  flex: 1;
  overflow-y: auto;
  padding: 0 20px;
}

.file-toolbar {
  padding: 8px 0;
  border-bottom: 1px solid var(--border);
  margin-bottom: 4px;
}

.select-all {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--text-secondary);
  cursor: pointer;
}

.file-list {
  display: flex;
  flex-direction: column;
  padding: 4px 0;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 4px;
  border-radius: 8px;
}

.file-item:hover {
  background: var(--surface-hover);
}

.file-status {
  font-size: 11px;
  padding: 1px 5px;
  border-radius: 3px;
  min-width: 40px;
  text-align: center;
}

.status-M, .status-A { background: var(--tag-green-bg); color: var(--tag-green-text); }
.status-D { background: var(--tag-red-bg); color: var(--tag-red-text); }
.status-R { background: var(--tag-blue-bg); color: var(--tag-blue-text); }
.status-\? { background: var(--tag-yellow-bg); color: var(--tag-yellow-text); }

.file-path {
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--text-primary);
}

.dialog-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 8px;
  padding: 14px 20px;
  border-top: 1px solid var(--border);
}

.btn-cancel {
  padding: 7px 16px;
  border: 1px solid var(--border);
  border-radius: 9px;
  background: var(--bg-surface);
  color: var(--text-secondary);
  font-family: var(--font-sans);
  font-size: 13px;
  cursor: pointer;
}

.btn-cancel:hover {
  background: var(--surface-hover);
}

.btn-push {
  padding: 7px 18px;
  border: none;
  border-radius: 9px;
  background: var(--accent);
  color: var(--accent-contrast);
  font-family: var(--font-sans);
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
}

.btn-push:hover:not(:disabled) {
  background: var(--accent-hover);
}

.btn-push:disabled {
  opacity: 0.4;
  cursor: default;
}
</style>
