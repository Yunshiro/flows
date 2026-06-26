<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'
import { computed } from 'vue'

const route = useRoute()
const router = useRouter()

const navItems = [
  { key: 'todos', label: '今日待办', path: '/todos' },
  { key: 'review', label: '每日复盘', path: '/review' },
  { key: 'weekly', label: '每周总览', path: '/weekly' },
  { key: 'notes', label: '笔记', path: '/notes' },
]

const activeKey = computed(() => route.path.split('/')[1] || 'todos')
function navigate(path: string) { router.push(path) }
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-brand" @click="navigate('/')">
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
        :class="['nav-item', { active: activeKey === item.key }]"
        @click="navigate(item.path)"
      >{{ item.label }}</button>
    </nav>

    <div class="sidebar-footer">
      <button :class="['nav-item', { active: activeKey === 'settings' }]" @click="navigate('/settings')">设置</button>
      <div class="sidebar-version">v0.1.0</div>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  width: 200px;
  height: 100vh;
  padding: 20px 14px;
  background: var(--bg-sidebar);
  box-shadow: var(--shadow-sidebar);
  position: relative;
  z-index: 10;
  user-select: none;
}

.sidebar-brand {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 8px 20px;
  color: var(--text-primary);
  cursor: pointer;
}

.brand-icon { flex-shrink: 0; }
.brand-name { font-size: 18px; font-weight: 700; letter-spacing: -0.03em; }

.sidebar-nav { display: flex; flex-direction: column; gap: 1px; flex: 1; }

.nav-item {
  display: flex;
  align-items: center;
  width: 100%;
  padding: 7px 10px;
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-secondary);
  font-family: var(--font-sans);
  font-size: 13.5px;
  font-weight: 500;
  cursor: pointer;
  transition: all 150ms ease;
  text-align: left;
}

.nav-item:hover { background: rgba(0,0,0,0.04); color: var(--text-primary); }

.nav-item.active {
  background: var(--accent-soft);
  color: var(--accent);
  font-weight: 600;
}

.sidebar-footer {
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding-top: 12px;
  border-top: 1px solid var(--border);
}

.sidebar-version {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--text-disabled);
  padding: 6px 10px 0;
}
</style>
