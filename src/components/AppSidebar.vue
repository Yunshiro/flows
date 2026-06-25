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

const activeKey = computed(() => {
  const seg = route.path.split('/')[1]
  return seg || 'todos'
})

function navigate(path: string) {
  router.push(path)
}
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
      >
        <span class="nav-dot" />
        {{ item.label }}
      </button>
    </nav>

    <div class="sidebar-footer">
      <button class="nav-item settings-btn" @click="navigate('/settings')">
        <span class="nav-dot" />
        设置
      </button>
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
  padding: 20px 16px;
  border-right: 1px solid #EAEAEA;
  background: #FFFFFF;
  user-select: none;
}

.sidebar-brand {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 8px 24px;
  color: #111111;
  cursor: pointer;
}

.brand-icon {
  flex-shrink: 0;
}

.brand-name {
  font-size: 17px;
  font-weight: 620;
  letter-spacing: -0.02em;
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: 2px;
  flex: 1;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 8px 10px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: #787774;
  font-family: 'SF Pro Display', 'Geist Sans', 'Helvetica Neue', 'PingFang SC', 'Microsoft YaHei', sans-serif;
  font-size: 14px;
  font-weight: 450;
  cursor: pointer;
  transition: background 150ms ease, color 150ms ease;
  text-align: left;
}

.nav-item:hover {
  background: rgba(0, 0, 0, 0.04);
  color: #111111;
}

.nav-item.active {
  background: rgba(0, 0, 0, 0.06);
  color: #111111;
  font-weight: 520;
}

.nav-dot {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background: transparent;
  transition: background 150ms ease;
  flex-shrink: 0;
}

.nav-item.active .nav-dot {
  background: #111111;
}

.sidebar-footer {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding-top: 12px;
  border-top: 1px solid #EAEAEA;
}

.settings-btn {
  font-size: 13px;
}

.sidebar-version {
  font-family: 'Geist Mono', 'SF Mono', 'JetBrains Mono', monospace;
  font-size: 11px;
  color: #9E9E9E;
  padding: 4px 10px 0;
}
</style>
