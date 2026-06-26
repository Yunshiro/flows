<script setup lang="ts">
import { ref, provide } from 'vue'
import AppSidebar from './components/AppSidebar.vue'
import AppToast from './components/AppToast.vue'

const toastRef = ref<InstanceType<typeof AppToast>>()

function toast(msg: string, type: 'success' | 'error' | 'info' = 'info') {
  toastRef.value?.show(msg, type)
}

provide('toast', toast)
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
  --bg-canvas: #F8F7F4;
  --bg-surface: #FFFFFF;
  --bg-sidebar: #FAFAF8;
  --border: #EDECEA;
  --text-primary: #111111;
  --text-secondary: #6B6B6B;
  --text-tertiary: #9C9C9C;
  --text-disabled: #D0D0D0;
  --accent: #4B6BF5;
  --accent-soft: #EEF1FD;
  --accent-hover: #3B53D9;

  --tag-red-bg: #FEF1F1;
  --tag-red-text: #C53030;
  --tag-yellow-bg: #FCF6E8;
  --tag-yellow-text: #92600A;
  --tag-blue-bg: #EDF3FE;
  --tag-blue-text: #2B6CB0;
  --tag-gray-bg: #F2F2F0;
  --tag-gray-text: #6B6B6B;
  --tag-green-bg: #EDF5EE;
  --tag-green-text: #2F6E34;

  --editor-toolbar-bg: #FAFAF8;
  --code-block-text: #F9FAFB;

  --shadow-xs: 0 1px 2px rgba(0,0,0,0.03);
  --shadow-sm: 0 1px 3px rgba(0,0,0,0.04), 0 1px 2px rgba(0,0,0,0.03);
  --shadow-md: 0 4px 12px rgba(0,0,0,0.05), 0 1px 2px rgba(0,0,0,0.03);
  --shadow-dropdown: 0 4px 16px rgba(0,0,0,0.06);
  --shadow-toast: 0 4px 20px rgba(0,0,0,0.08);
  --shadow-dialog: 0 12px 40px rgba(0,0,0,0.10);
  --shadow-sidebar: 1px 0 0 rgba(0,0,0,0.04), 4px 0 16px rgba(0,0,0,0.02);

  --radius-xs: 4px;
  --radius-sm: 6px;
  --radius-md: 8px;
  --radius-lg: 12px;

  --font-sans: 'SF Pro Display', 'Geist Sans', 'Helvetica Neue', 'PingFang SC', 'Microsoft YaHei', sans-serif;
  --font-mono: 'SF Mono', 'Geist Mono', 'JetBrains Mono', 'Cascadia Code', monospace;
  font-family: var(--font-sans);
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
}
</style>

<style scoped>
.app-shell {
  display: flex;
  height: 100vh;
  overflow: hidden;
}

.app-main {
  flex: 1;
  overflow-y: auto;
  padding: 28px;
}
</style>
