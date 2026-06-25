<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { noteApi } from '../api/notes'

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
    await noteApi.gitPush()
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
        没有需要推送的文件变更。
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
  background: rgba(0, 0, 0, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.dialog {
  width: 480px;
  max-height: 70vh;
  background: #FFFFFF;
  border: 1px solid #EAEAEA;
  border-radius: 10px;
  display: flex;
  flex-direction: column;
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
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
  color: #787774;
  cursor: pointer;
}

.dialog-close:hover {
  background: rgba(0, 0, 0, 0.05);
}

.dialog-hint {
  padding: 20px;
  text-align: center;
  color: #9E9E9E;
  font-size: 13px;
}

.dialog-body {
  flex: 1;
  overflow-y: auto;
  padding: 0 20px;
}

.file-toolbar {
  padding: 8px 0;
  border-bottom: 1px solid #EAEAEA;
  margin-bottom: 4px;
}

.select-all {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: #787774;
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
  border-radius: 4px;
}

.file-item:hover {
  background: rgba(0, 0, 0, 0.02);
}

.file-status {
  font-size: 11px;
  padding: 1px 5px;
  border-radius: 3px;
  min-width: 40px;
  text-align: center;
}

.status-M, .status-A { background: #EDF3EC; color: #346538; }
.status-D { background: #FDEBEC; color: #9F2F2D; }
.status-R { background: #E1F3FE; color: #1F6C9F; }
.status-\? { background: #FBF3DB; color: #956400; }

.file-path {
  font-family: var(--font-mono);
  font-size: 12px;
  color: #111111;
}

.dialog-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 8px;
  padding: 14px 20px;
  border-top: 1px solid #EAEAEA;
}

.btn-cancel {
  padding: 7px 16px;
  border: 1px solid #EAEAEA;
  border-radius: 6px;
  background: #FFFFFF;
  color: #787774;
  font-family: var(--font-sans);
  font-size: 13px;
  cursor: pointer;
}

.btn-cancel:hover {
  background: rgba(0, 0, 0, 0.03);
}

.btn-push {
  padding: 7px 18px;
  border: none;
  border-radius: 6px;
  background: #111111;
  color: #FFFFFF;
  font-family: var(--font-sans);
  font-size: 13px;
  font-weight: 520;
  cursor: pointer;
}

.btn-push:hover:not(:disabled) {
  background: #333333;
}

.btn-push:disabled {
  opacity: 0.4;
  cursor: default;
}
</style>
