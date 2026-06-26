<script setup lang="ts">
import { ref } from 'vue'

interface Toast {
  id: number
  message: string
  type: 'success' | 'error' | 'info'
}

const toasts = ref<Toast[]>([])
let nextId = 0

function show(message: string, type: 'success' | 'error' | 'info' = 'info') {
  const id = nextId++
  toasts.value.push({ id, message, type })
  setTimeout(() => {
    toasts.value = toasts.value.filter(t => t.id !== id)
  }, 3000)
}

defineExpose({ show })
</script>

<template>
  <div class="toast-container">
    <div
      v-for="t in toasts"
      :key="t.id"
      :class="['toast', `toast--${t.type}`]"
    >
      <span class="toast-icon">
        <svg v-if="t.type === 'success'" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M5 13l4 4L19 7"/></svg>
        <svg v-else-if="t.type === 'error'" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M18 6L6 18M6 6l12 12"/></svg>
        <svg v-else width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4M12 8h0"/></svg>
      </span>
      <span class="toast-msg">{{ t.message }}</span>
    </div>
  </div>
</template>

<style scoped>
.toast-container {
  position: fixed;
  bottom: 24px;
  right: 24px;
  z-index: 1000;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.toast {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  border-radius: 7px;
  font-size: 13px;
  font-weight: 400;
  box-shadow: 0 2px 12px rgba(0,0,0,0.08);
  animation: slideIn 300ms cubic-bezier(0.16, 1, 0.3, 1);
  max-width: 360px;
}

.toast--success { background: var(--tag-green-bg); color: var(--tag-green-text); border: 1px solid #c8e6c9; }
.toast--error { background: var(--tag-red-bg); color: var(--tag-red-text); border: 1px solid #ffcdd2; }
.toast--info { background: var(--tag-blue-bg); color: var(--tag-blue-text); border: 1px solid #b3e5fc; }

.toast-icon { display: flex; align-items: center; flex-shrink: 0; }
.toast-msg { flex: 1; line-height: 1.4; }

@keyframes slideIn {
  from { opacity: 0; transform: translateY(12px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>
