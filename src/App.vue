<script setup lang="ts">
import { onMounted, onUnmounted, provide, ref, watch } from 'vue'
import AppSidebar from './components/AppSidebar.vue'
import AppToast from './components/AppToast.vue'
import { useSettingsStore } from './stores/useSettingsStore'

const toastRef = ref<InstanceType<typeof AppToast>>()
const settingsStore = useSettingsStore()
let mediaQuery: MediaQueryList | null = null
const handleSystemThemeChange = () => {
  if (settingsStore.settings.theme === 'system') applyTheme('system')
}

function toast(msg: string, type: 'success' | 'error' | 'info' = 'info') {
  toastRef.value?.show(msg, type)
}

provide('toast', toast)

function applyTheme(theme: string) {
  const resolvedTheme = theme === 'system'
    ? (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light')
    : theme
  document.documentElement.dataset.theme = resolvedTheme === 'dark' ? 'dark' : 'light'
  document.documentElement.style.colorScheme = resolvedTheme === 'dark' ? 'dark' : 'light'
}

onMounted(async () => {
  mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
  mediaQuery.addEventListener('change', handleSystemThemeChange)
  await settingsStore.loadSettings()
  applyTheme(settingsStore.settings.theme)
})

onUnmounted(() => {
  mediaQuery?.removeEventListener('change', handleSystemThemeChange)
})

watch(() => settingsStore.settings.theme, (theme) => applyTheme(theme), { immediate: true })
</script>

<template>
  <div class="app-shell">
    <AppSidebar />
    <main class="app-main">
      <router-view />
    </main>
    <AppToast ref="toastRef" />
  </div>
</template>

<style>
*,
*::before,
*::after {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  --bg-canvas: #f3f0e8;
  --bg-canvas-rgb: 243, 240, 232;
  --bg-surface: #fffdf8;
  --bg-surface-rgb: 255, 253, 248;
  --bg-surface-raised: #ffffff;
  --bg-sidebar: rgba(255, 253, 248, 0.78);
  --bg-muted: #ebe6d9;
  --border: rgba(42, 38, 31, 0.11);
  --border-strong: rgba(42, 38, 31, 0.18);
  --text-primary: #181713;
  --text-secondary: #686154;
  --text-tertiary: #918979;
  --text-disabled: #bdb4a4;
  --accent: #2f7d68;
  --accent-rgb: 47, 125, 104;
  --accent-soft: rgba(47, 125, 104, 0.12);
  --accent-hover: #276a58;
  --accent-contrast: #fffdf7;
  --surface-hover: rgba(42, 38, 31, 0.055);
  --surface-active: rgba(42, 38, 31, 0.085);
  --focus-ring: rgba(47, 125, 104, 0.28);

  --tag-red-bg: #f8e6e2;
  --tag-red-text: #a94a3e;
  --tag-yellow-bg: #f6edcf;
  --tag-yellow-text: #816326;
  --tag-blue-bg: #e3edf0;
  --tag-blue-text: #2e6f7c;
  --tag-gray-bg: rgba(42, 38, 31, 0.075);
  --tag-gray-text: #686154;
  --tag-green-bg: rgba(47, 125, 104, 0.12);
  --tag-green-text: #2f7d68;

  --editor-toolbar-bg: rgba(247, 244, 236, 0.78);
  --code-block-text: #F9FAFB;

  --shadow-xs: 0 1px 2px rgba(36, 30, 20, 0.04);
  --shadow-sm: 0 10px 30px rgba(44, 38, 27, 0.055), 0 1px 1px rgba(44, 38, 27, 0.04);
  --shadow-md: 0 18px 48px rgba(44, 38, 27, 0.085), 0 2px 4px rgba(44, 38, 27, 0.05);
  --shadow-dropdown: 0 16px 36px rgba(44, 38, 27, 0.12);
  --shadow-toast: 0 16px 38px rgba(44, 38, 27, 0.16);
  --shadow-dialog: 0 24px 70px rgba(44, 38, 27, 0.22);
  --shadow-sidebar: inset -1px 0 0 var(--border), 10px 0 30px rgba(44, 38, 27, 0.045);

  --radius-xs: 4px;
  --radius-sm: 6px;
  --radius-md: 8px;
  --radius-lg: 12px;
  --radius-xl: 16px;

  --font-sans: ui-rounded, 'Avenir Next', 'SF Pro Display', 'Helvetica Neue', 'PingFang SC', 'Microsoft YaHei', sans-serif;
  --font-mono: 'SF Mono', 'Geist Mono', 'JetBrains Mono', 'Cascadia Code', monospace;
  font-family: var(--font-sans);
}

:root[data-theme="dark"] {
  --bg-canvas: #111512;
  --bg-canvas-rgb: 17, 21, 18;
  --bg-surface: #191e1a;
  --bg-surface-rgb: 25, 30, 26;
  --bg-surface-raised: #202620;
  --bg-sidebar: rgba(22, 27, 23, 0.78);
  --bg-muted: #242b25;
  --border: rgba(229, 224, 211, 0.105);
  --border-strong: rgba(229, 224, 211, 0.18);
  --text-primary: #f3efe5;
  --text-secondary: #b4ad9f;
  --text-tertiary: #857f73;
  --text-disabled: #5f5a51;
  --accent: #76c7aa;
  --accent-rgb: 118, 199, 170;
  --accent-soft: rgba(118, 199, 170, 0.15);
  --accent-hover: #91d6bd;
  --accent-contrast: #102018;
  --surface-hover: rgba(229, 224, 211, 0.065);
  --surface-active: rgba(229, 224, 211, 0.105);
  --focus-ring: rgba(118, 199, 170, 0.32);

  --tag-red-bg: rgba(198, 92, 76, 0.16);
  --tag-red-text: #ee9c8f;
  --tag-yellow-bg: rgba(205, 169, 84, 0.16);
  --tag-yellow-text: #e2c47e;
  --tag-blue-bg: rgba(86, 154, 169, 0.16);
  --tag-blue-text: #8bc4ce;
  --tag-gray-bg: rgba(229, 224, 211, 0.08);
  --tag-gray-text: #b4ad9f;
  --tag-green-bg: rgba(118, 199, 170, 0.15);
  --tag-green-text: #9be0c7;

  --editor-toolbar-bg: rgba(22, 27, 23, 0.82);
  --shadow-xs: 0 1px 2px rgba(0, 0, 0, 0.2);
  --shadow-sm: 0 14px 34px rgba(0, 0, 0, 0.22), 0 1px 0 rgba(255, 255, 255, 0.02) inset;
  --shadow-md: 0 22px 58px rgba(0, 0, 0, 0.34), 0 1px 0 rgba(255, 255, 255, 0.03) inset;
  --shadow-dropdown: 0 18px 42px rgba(0, 0, 0, 0.38);
  --shadow-toast: 0 16px 42px rgba(0, 0, 0, 0.42);
  --shadow-dialog: 0 28px 90px rgba(0, 0, 0, 0.48);
  --shadow-sidebar: inset -1px 0 0 var(--border), 12px 0 36px rgba(0, 0, 0, 0.2);
}

body {
  background: var(--bg-canvas);
  color: var(--text-primary);
  font-family: var(--font-sans);
  font-size: 14px;
  line-height: 1.6;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  overflow: hidden;
  font-variant-numeric: tabular-nums;
}

body::before {
  content: '';
  position: fixed;
  inset: 0;
  pointer-events: none;
  z-index: -1;
  background:
    radial-gradient(circle at 14% 10%, rgba(var(--accent-rgb), 0.11), transparent 32rem),
    radial-gradient(circle at 86% 14%, rgba(214, 158, 91, 0.12), transparent 28rem),
    linear-gradient(135deg, rgba(var(--bg-surface-rgb), 0.25), transparent 42%);
}

button,
input,
textarea,
select {
  font: inherit;
}

button,
input,
textarea,
select,
a {
  transition: background-color 180ms ease, border-color 180ms ease, color 180ms ease, box-shadow 180ms ease, transform 180ms ease, opacity 180ms ease;
}

button:active {
  transform: translateY(1px);
}

button:focus-visible,
input:focus-visible,
textarea:focus-visible,
select:focus-visible {
  outline: 2px solid var(--focus-ring);
  outline-offset: 2px;
}

::selection {
  background: var(--accent-soft);
  color: var(--text-primary);
}
</style>

<style scoped>
.app-shell {
  display: flex;
  height: 100dvh;
  overflow: hidden;
  background: var(--bg-canvas);
}

.app-main {
  flex: 1;
  overflow-y: auto;
  padding: 32px;
  scroll-behavior: smooth;
}
</style>
