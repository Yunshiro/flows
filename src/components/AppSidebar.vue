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
      <span class="brand-mark">
        <svg class="brand-icon" width="25" height="25" viewBox="0 0 28 28" aria-hidden="true">
          <defs>
            <linearGradient id="brandFlowLine" x1="5" y1="5" x2="23" y2="20" gradientUnits="userSpaceOnUse">
              <stop stop-color="#BEEB7D" />
              <stop offset="0.5" stop-color="#5FD0A6" />
              <stop offset="1" stop-color="#18A7A0" />
            </linearGradient>
          </defs>
          <path class="brand-icon-frame" d="M7.2 2.9h13.6c2.6 0 4.3 1.8 4.3 4.3v13.6c0 2.6-1.8 4.3-4.3 4.3H7.2c-2.6 0-4.3-1.8-4.3-4.3V7.2c0-2.6 1.8-4.3 4.3-4.3Z" />
          <path class="brand-flow brand-flow-top" d="M6.8 9.6c3-3 6.9-4.3 11.5-3.8 1.2.1 2.2-.4 2.9-1.4-.3 3.6-2.8 6-7.4 7.2-2.5.6-4.9.6-7.4 0" />
          <path class="brand-flow brand-flow-bottom" d="M6.5 18.3c2-4.8 5.6-7.8 10.8-8.9 1.5-.3 2.8-.9 3.9-1.9-.5 4.1-3.3 7.3-8.4 9.5-2 .9-4.1 1.4-6.3 1.5" />
          <path class="brand-note" d="M16.6 14.6h5.1c.7 0 1.2.5 1.2 1.2v4.3l-2.1 2h-4.2c-.7 0-1.2-.5-1.2-1.2v-5.1c0-.7.5-1.2 1.2-1.2Z" />
          <path class="brand-note-fold" d="M20.8 22.1v-1.7c0-.3.2-.5.5-.5h1.6" />
          <path class="brand-note-line" d="M17.6 16.9h3.2M17.6 19h2.4" />
          <path class="brand-node-line" d="M6.3 22.3h8.4" />
          <circle class="brand-node brand-node-muted" cx="6.3" cy="22.3" r="1.05" />
          <circle class="brand-node brand-node-hot" cx="10.5" cy="22.3" r="1.05" />
          <circle class="brand-node brand-node-cool" cx="14.7" cy="22.3" r="1.05" />
        </svg>
      </span>
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
  width: 218px;
  height: 100dvh;
  padding: 18px 12px;
  background: var(--bg-sidebar);
  backdrop-filter: blur(18px);
  box-shadow: var(--shadow-sidebar);
  position: relative;
  z-index: 10;
  user-select: none;
}

.sidebar-brand {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 5px 8px 22px;
  color: var(--text-primary);
  cursor: pointer;
}

.brand-mark {
  width: 36px;
  height: 36px;
  display: grid;
  place-items: center;
  border: 1px solid rgba(var(--accent-rgb), 0.18);
  border-radius: 12px;
  background:
    radial-gradient(circle at 30% 18%, rgba(var(--accent-rgb), 0.16), transparent 44%),
    linear-gradient(145deg, rgba(var(--bg-surface-rgb), 0.9), rgba(var(--accent-rgb), 0.08));
  box-shadow: inset 0 1px 0 rgba(255,255,255,0.16), 0 10px 22px rgba(var(--accent-rgb), 0.1);
}
.brand-icon { flex-shrink: 0; overflow: visible; }
.brand-icon-frame {
  fill: none;
  stroke: rgba(var(--accent-rgb), 0.22);
  stroke-width: 1.15;
}

.brand-flow {
  fill: none;
  stroke: url(#brandFlowLine);
  stroke-linecap: round;
  stroke-linejoin: round;
  filter: drop-shadow(0 1px 2px rgba(var(--accent-rgb), 0.18));
}

.brand-flow-top { stroke-width: 2.35; }
.brand-flow-bottom { stroke-width: 2.55; }

.brand-note,
.brand-note-fold,
.brand-note-line,
.brand-node-line {
  fill: none;
  stroke: var(--text-tertiary);
  stroke-linecap: round;
  stroke-linejoin: round;
}

.brand-note { stroke-width: 1.25; }
.brand-note-fold,
.brand-note-line,
.brand-node-line {
  stroke-width: 1.05;
  opacity: 0.82;
}

.brand-node {
  stroke: var(--bg-sidebar);
  stroke-width: 0.85;
}

.brand-node-muted { fill: #82908a; }
.brand-node-hot { fill: #e2a33a; }
.brand-node-cool { fill: #62c196; }
.brand-name { font-size: 18px; font-weight: 720; letter-spacing: -0.02em; }

.sidebar-nav { display: flex; flex-direction: column; gap: 1px; flex: 1; }

.nav-item {
  display: flex;
  align-items: center;
  width: 100%;
  padding: 8px 10px;
  border: none;
  border-radius: var(--radius-md);
  background: transparent;
  color: var(--text-secondary);
  font-family: var(--font-sans);
  font-size: 13.5px;
  font-weight: 500;
  cursor: pointer;
  text-align: left;
}

.nav-item:hover { background: var(--surface-hover); color: var(--text-primary); }

.nav-item.active {
  background: var(--accent-soft);
  color: var(--accent);
  font-weight: 600;
  box-shadow: inset 0 0 0 1px rgba(var(--accent-rgb), 0.12);
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
