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
  --bg-canvas: #F7F6F3;
  --bg-surface: #FFFFFF;
  --border: #EAEAEA;
  --text-primary: #111111;
  --text-secondary: #787774;
  --text-tertiary: #9E9E9E;
  --font-sans: 'SF Pro Display', 'Geist Sans', 'Helvetica Neue', 'PingFang SC', 'Microsoft YaHei', sans-serif;
  --font-mono: 'Geist Mono', 'SF Mono', 'JetBrains Mono', 'Cascadia Code', monospace;
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
