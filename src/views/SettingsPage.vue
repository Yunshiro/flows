<script setup lang="ts">
import { useSettingsStore } from '../stores/useSettingsStore'
import { computed, onMounted, onUnmounted, ref, inject } from 'vue'
import { noteApi } from '../api/notes'
import { settingsApi } from '../api/settings'

const toast = inject<(msg: string, type: 'success' | 'error' | 'info') => void>('toast', () => {})
import { llmApi, type LlmConfig } from '../api/llm'
import GitPushDialog from '../components/GitPushDialog.vue'

const store = useSettingsStore()
const saved = ref(false)
const gitStatus = ref('')
const gitInited = ref(false)
const gitHasRemote = ref(false)
const syncing = ref(false)
const showPushDialog = ref(false)
const branchOptions = ref<string[]>([])
let syncTimer: ReturnType<typeof setInterval> | null = null

// LLM configs
const llmConfigs = ref<LlmConfig[]>([])
const editingConfig = ref<Partial<LlmConfig> | null>(null)
const llmTesting = ref(false)
const llmTestResult = ref('')

const emptyConfig = (): Partial<LlmConfig> => ({
  name: '',
  baseUrl: 'https://api.openai.com/v1',
  apiKey: '',
  model: 'deepseek-chat',
  isDefault: false,
})

const syncBranch = computed(() => store.settings.gitBranch.trim())
const canSync = computed(() => gitHasRemote.value && !!syncBranch.value)
const themeOptions = [
  { value: 'system', label: '跟随系统' },
  { value: 'light', label: '亮色' },
  { value: 'dark', label: '暗色' },
] as const

onMounted(async () => {
  await store.loadSettings()
  await checkGitState()
  await loadLlmConfigs()
  if (store.settings.autoSyncEnabled) startAutoSync()
})

onUnmounted(() => stopAutoSync())

async function loadLlmConfigs() {
  try { llmConfigs.value = await llmApi.list() } catch { /* */ }
}

async function checkGitState() {
  try {
    const s = await noteApi.gitStatus()
    gitStatus.value = s
    gitInited.value = !s.includes('未初始化')
    gitHasRemote.value = false
    if (gitInited.value) {
      const remoteUrl = await noteApi.gitGetRemote()
      gitHasRemote.value = !!remoteUrl
      if (remoteUrl && !store.settings.gitRemoteUrl) store.settings.gitRemoteUrl = remoteUrl
      branchOptions.value = await noteApi.gitListBranches()
      if (!store.settings.gitBranch && branchOptions.value.length > 0) {
        store.settings.gitBranch = branchOptions.value[0]
      }
    } else {
      branchOptions.value = []
    }
  } catch {
    gitStatus.value = 'Git 不可用'
  }
}

async function handleSave() {
  await store.saveSettings(store.settings)
  saved.value = true
  setTimeout(() => saved.value = false, 2000)
  if (store.settings.autoSyncEnabled) startAutoSync(); else stopAutoSync()
}

async function handleThemeChange(theme: 'system' | 'light' | 'dark') {
  store.settings.theme = theme
  await store.saveSettings({ theme })
}

async function handleInit() {
  try { const msg = await noteApi.gitInit(); gitStatus.value = msg; gitInited.value = true; await checkGitState() } catch (e) { gitStatus.value = '失败: ' + String(e) }
}

async function handleConfigRemote() {
  const url = store.settings.gitRemoteUrl.trim()
  if (!url) return
  try { await noteApi.gitSetRemote(url); await checkGitState() } catch (e) { gitStatus.value = '失败: ' + String(e) }
}

async function handleSwitchBranch() {
  const branch = syncBranch.value
  if (!branch) {
    toast('请先选择或填写同步分支', 'error')
    return
  }
  syncing.value = true
  try {
    await store.saveSettings({ gitBranch: branch })
    const msg = await noteApi.gitCheckoutBranch(branch)
    gitStatus.value = msg
    await checkGitState()
  } catch (e) {
    toast('切换分支失败: ' + String(e), 'error')
  } finally {
    syncing.value = false
  }
}

async function handlePull() {
  syncing.value = true
  try { await noteApi.gitPull(syncBranch.value); await checkGitState() } catch (e) { toast('拉取失败: ' + String(e), 'error') } finally { syncing.value = false }
}

function onPushed() { showPushDialog.value = false; checkGitState() }

function startAutoSync() {
  stopAutoSync()
  if (!syncBranch.value) return
  const minutes = Math.max(1, store.settings.autoSyncIntervalMinutes || 30)
  syncTimer = setInterval(async () => {
    try { await noteApi.gitPull(syncBranch.value); await noteApi.gitPush(syncBranch.value); gitStatus.value = '自动同步 ' + new Date().toLocaleTimeString('zh-CN', { hour:'2-digit', minute:'2-digit' }) } catch { /* */ }
  }, minutes * 60 * 1000)
}

function stopAutoSync() { if (syncTimer) { clearInterval(syncTimer); syncTimer = null } }

// LLM config actions
function startAdd() { editingConfig.value = emptyConfig() }
function startEdit(cfg: LlmConfig) { editingConfig.value = { ...cfg } }
function cancelEdit() { editingConfig.value = null; llmTestResult.value = '' }

async function handleSaveConfig() {
  const c = editingConfig.value
  if (!c || !c.name?.trim()) return
  try {
    await llmApi.save({
      id: c.id,
      name: c.name!,
      baseUrl: c.baseUrl || 'https://api.openai.com/v1',
      apiKey: c.apiKey || '',
      model: c.model || 'deepseek-chat',
      isDefault: c.isDefault || false,
    })
    editingConfig.value = null
    await loadLlmConfigs()
  } catch (e) { toast('保存失败: ' + String(e), 'error') }
}

async function handleDeleteConfig(id: string) {
  try { await llmApi.delete(id); await loadLlmConfigs() } catch (e) { toast('删除失败: ' + String(e), 'error') }
}

async function handleSetDefault(id: string) {
  const cfg = llmConfigs.value.find(c => c.id === id)
  if (!cfg) return
  try {
    await llmApi.save({ ...cfg, isDefault: true })
    await loadLlmConfigs()
  } catch (e) { toast('设置失败: ' + String(e), 'error') }
}

async function handleTestLlm() {
  const c = editingConfig.value
  if (!c) return
  llmTesting.value = true; llmTestResult.value = ''
  try {
    const msg = await settingsApi.testLlm(c.baseUrl || '', c.apiKey || '', c.model || '')
    llmTestResult.value = msg
  } catch (e) { llmTestResult.value = '连接失败: ' + String(e) } finally { llmTesting.value = false }
}
</script>

<template>
  <div class="page">
    <header class="page-header">
      <h2 class="page-title">设置</h2>
      <button class="btn-save" @click="handleSave">{{ saved ? '已保存' : '保存设置' }}</button>
    </header>

    <!-- Notes storage -->
    <section class="card">
      <div class="card-header-row compact">
        <div>
          <h3 class="card-title">外观</h3>
          <p class="card-desc">选择 Flows 的界面主题</p>
        </div>
        <div class="theme-switch" role="group" aria-label="主题切换">
          <button
            v-for="option in themeOptions"
            :key="option.value"
            :class="['theme-option', { active: store.settings.theme === option.value }]"
            @click="handleThemeChange(option.value)"
          >
            {{ option.label }}
          </button>
        </div>
      </div>
    </section>

    <section class="card">
      <h3 class="card-title">笔记存储</h3>
      <p class="card-desc">Markdown 笔记保存在本地 <code>~/flows-notes/</code> 目录</p>
    </section>

    <!-- Git Sync -->
    <section class="card">
      <h3 class="card-title">Git 同步</h3>
      <p class="card-desc">连接远程仓库同步笔记：初始化仓库 → 配置地址 → 拉取/推送</p>

      <div class="git-status-bar">
        <span class="step-dot" :class="{ done: gitInited }">1</span>
        <span class="step-line" :class="{ done: gitInited }" />
        <span class="step-dot" :class="{ done: gitHasRemote }">2</span>
        <span class="step-line" :class="{ done: gitHasRemote }" />
        <span class="step-dot" :class="{ done: gitHasRemote }">3</span>
        <span class="step-text">{{ gitInited ? (gitHasRemote ? '已就绪' : '需配置地址') : '需初始化' }}</span>
      </div>

      <div class="git-steps">
        <div class="git-step">
          <span class="git-step-num">1</span>
          <div class="git-step-body">
            <span class="git-step-label">初始化本地仓库</span>
          </div>
          <button v-if="!gitInited" class="btn-step" @click="handleInit">初始化</button>
          <span v-else class="step-done-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--tag-green-text)" stroke-width="2.5"><path d="M5 13l4 4L19 7"/></svg></span>
        </div>
        <div class="git-step" :class="{ disabled: !gitInited }">
          <span class="git-step-num">2</span>
          <div class="git-step-body">
            <span class="git-step-label">配置远程仓库与分支</span>
            <div class="git-remote-row">
              <input v-model="store.settings.gitRemoteUrl" class="git-remote-input" placeholder="https://github.com/you/notes.git" :disabled="!gitInited" />
              <button class="btn-step" :disabled="!gitInited || !store.settings.gitRemoteUrl.trim()" @click="handleConfigRemote">配置</button>
            </div>
            <div class="git-remote-row">
              <input
                v-model.trim="store.settings.gitBranch"
                class="git-remote-input"
                list="git-branch-options"
                placeholder="选择或输入分支名，例如 master / notes"
                :disabled="!gitInited"
              />
              <datalist id="git-branch-options">
                <option v-for="branch in branchOptions" :key="branch" :value="branch" />
              </datalist>
              <button class="btn-step" :disabled="!gitInited || !syncBranch || syncing" @click="handleSwitchBranch">切换</button>
            </div>
          </div>
        </div>
        <div class="git-step" :class="{ disabled: !gitHasRemote }">
          <span class="git-step-num">3</span>
          <div class="git-step-body">
            <span class="git-step-label">同步笔记</span>
            <span class="git-step-desc" v-if="gitStatus">{{ gitStatus }}</span>
          </div>
          <div class="git-step-actions">
            <button class="btn-step" :disabled="!canSync || syncing" @click="handlePull">拉取</button>
            <button class="btn-step primary" :disabled="!canSync || syncing" @click="showPushDialog = true">推送</button>
          </div>
        </div>
      </div>

      <label class="auto-sync-row">
        <input type="checkbox" v-model="store.settings.autoSyncEnabled" :disabled="!canSync" />
        <span class="auto-sync-label">自动同步（每 {{ store.settings.autoSyncIntervalMinutes }} 分钟）</span>
      </label>
    </section>

    <!-- LLM Configs -->
    <section class="card">
      <div class="card-header-row">
        <div>
          <h3 class="card-title">AI 模型配置</h3>
          <p class="card-desc">支持 OpenAI 兼容接口（DeepSeek、OpenAI、Ollama 等），可配置多个模型随时切换</p>
        </div>
        <button v-if="!editingConfig" class="btn-add" @click="startAdd">+ 添加配置</button>
      </div>

      <!-- Edit form -->
      <div v-if="editingConfig" class="config-form">
        <label class="field">
          <span class="field-label">配置名称</span>
          <input v-model="editingConfig.name" class="field-input" placeholder="例如: DeepSeek、OpenAI" />
        </label>
        <label class="field">
          <span class="field-label">API 地址</span>
          <input v-model="editingConfig.baseUrl" class="field-input" placeholder="https://api.deepseek.com/v1" />
        </label>
        <label class="field">
          <span class="field-label">API Key</span>
          <input v-model="editingConfig.apiKey" type="password" class="field-input" placeholder="sk-..." />
        </label>
        <label class="field">
          <span class="field-label">模型名称</span>
          <input v-model="editingConfig.model" class="field-input" placeholder="deepseek-chat" />
        </label>
        <label class="field field--row">
          <input type="checkbox" v-model="editingConfig.isDefault" />
          <span class="field-label">设为默认</span>
        </label>
        <div class="config-form-actions">
          <button class="btn-test" @click="handleTestLlm" :disabled="llmTesting">{{ llmTesting ? '测试中...' : '测试连接' }}</button>
          <div class="config-form-right">
            <button class="btn-cancel" @click="cancelEdit">取消</button>
            <button class="btn-step primary" @click="handleSaveConfig">保存配置</button>
          </div>
        </div>
        <div v-if="llmTestResult" :class="['test-result', { error: llmTestResult.includes('失败') }]">{{ llmTestResult }}</div>
      </div>

      <!-- Config list -->
      <div v-if="!editingConfig && llmConfigs.length > 0" class="config-list">
        <div v-for="cfg in llmConfigs" :key="cfg.id" class="config-item">
          <div class="config-item-main">
            <div class="config-item-header">
              <span class="config-name">{{ cfg.name }}</span>
              <span v-if="cfg.isDefault" class="config-badge">默认</span>
            </div>
            <span class="config-detail">{{ cfg.model }} @ {{ cfg.baseUrl }}</span>
          </div>
          <div class="config-item-actions">
            <button v-if="!cfg.isDefault" class="btn-ghost-sm" @click="handleSetDefault(cfg.id)">设为默认</button>
            <button class="btn-ghost-sm" @click="startEdit(cfg)">编辑</button>
            <button class="btn-ghost-sm danger" @click="handleDeleteConfig(cfg.id)">删除</button>
          </div>
        </div>
      </div>

      <p v-if="!editingConfig && llmConfigs.length === 0" class="empty-hint">暂无配置，点击「添加配置」开始</p>
    </section>

    <section class="card card--about">
      <h3 class="card-title">关于</h3>
      <p class="about-text">Flows v0.1.0 — Tauri v2 + Vue 3 + TypeScript + Rust<br>SQLite: ~/.flows/flows.db | 笔记: ~/flows-notes/</p>
    </section>

    <GitPushDialog v-if="showPushDialog" :branch="syncBranch" @close="showPushDialog = false" @pushed="onPushed" />
  </div>
</template>

<style scoped>
.page { max-width: 720px; }
.page-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 20px; }
.page-title { font-size: 32px; font-weight: 760; letter-spacing: -0.02em; line-height: 1.08; }

.btn-save { padding: 8px 18px; border: none; border-radius: 10px; background: var(--accent); color: var(--accent-contrast); font-family: var(--font-sans); font-size: 13px; font-weight: 600; cursor: pointer; box-shadow: 0 10px 22px rgba(var(--accent-rgb), 0.18); }
.btn-save:hover { background: var(--accent-hover); }

.card { background: rgba(var(--bg-surface-rgb), 0.78); border: 1px solid var(--border); border-radius: var(--radius-xl); padding: 20px; margin-bottom: 16px; box-shadow: var(--shadow-sm); backdrop-filter: blur(18px); }
.card-title { font-size: 14px; font-weight: 680; margin-bottom: 4px; color: var(--text-primary); }
.card-desc { font-size: 12px; color: var(--text-tertiary); margin-bottom: 16px; }
.card-desc code { font-family: var(--font-mono); font-size: 11px; background: var(--tag-gray-bg); padding: 1px 4px; border-radius: 3px; }
.card-header-row { display: flex; align-items: flex-start; justify-content: space-between; margin-bottom: 8px; }
.card-header-row.compact { align-items: center; margin-bottom: 0; gap: 18px; }

.theme-switch { display: inline-flex; gap: 3px; padding: 3px; border: 1px solid var(--border); border-radius: 10px; background: var(--bg-muted); }
.theme-option { padding: 6px 10px; border: none; border-radius: 7px; background: transparent; color: var(--text-secondary); font-size: 12px; font-family: var(--font-sans); cursor: pointer; }
.theme-option:hover { color: var(--text-primary); background: var(--surface-hover); }
.theme-option.active { color: var(--accent); background: var(--bg-surface); box-shadow: var(--shadow-xs); font-weight: 650; }

.btn-add { padding: 6px 14px; border: 1px solid var(--border); border-radius: 9px; background: var(--bg-surface); color: var(--text-primary); font-family: var(--font-sans); font-size: 13px; cursor: pointer; white-space: nowrap; }
.btn-add:hover { background: var(--surface-hover); }

/* Git */
.git-status-bar { display: flex; align-items: center; gap: 0; margin-bottom: 16px; padding: 10px 14px; background: var(--bg-muted); border-radius: 10px; }
.step-dot { width: 22px; height: 22px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 11px; font-weight: 600; background: var(--tag-gray-bg); color: var(--text-tertiary); flex-shrink: 0; }
.step-dot.done { background: var(--tag-green-text); color: var(--accent-contrast); }
.step-line { flex: 1; height: 2px; background: var(--tag-gray-bg); margin: 0 4px; }
.step-line.done { background: var(--tag-green-text); }
.step-text { margin-left: 10px; font-size: 12px; color: var(--text-secondary); flex-shrink: 0; }

.git-steps { display: flex; flex-direction: column; }
.git-step { display: flex; align-items: flex-start; gap: 12px; padding: 12px 0; border-bottom: 1px solid var(--tag-gray-bg); }
.git-step.disabled { opacity: 0.4; pointer-events: none; }
.git-step-num { width: 20px; height: 20px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 11px; font-weight: 600; background: var(--tag-gray-bg); color: var(--text-secondary); flex-shrink: 0; }
.git-step-body { flex: 1; min-width: 0; }
.git-step-label { font-size: 13px; font-weight: 500; display: block; }
.git-step-desc { font-size: 11px; color: var(--text-tertiary); margin-top: 2px; display: block; }
.step-done-icon { display: flex; align-items: center; flex-shrink: 0; }
.git-remote-row { display: flex; gap: 6px; margin-top: 6px; }
.git-remote-input { flex: 1; padding: 6px 9px; border: 1px solid var(--border); border-radius: 8px; font-family: var(--font-mono); font-size: 12px; color: var(--text-primary); background: var(--bg-muted); outline: none; }
.git-remote-input:focus { border-color: var(--accent); background: var(--bg-surface); }
.git-remote-input:disabled { opacity: 0.5; }
.git-remote-input::placeholder { color: var(--text-disabled); }
.git-step-actions { display: flex; gap: 6px; flex-shrink: 0; }

.btn-step { padding: 6px 12px; border: 1px solid var(--border); border-radius: 8px; background: var(--bg-surface); color: var(--text-primary); font-family: var(--font-sans); font-size: 12px; cursor: pointer; white-space: nowrap; }
.btn-step:hover:not(:disabled) { background: var(--surface-hover); border-color: var(--border-strong); }
.btn-step:disabled { opacity: 0.4; cursor: default; }
.btn-step.primary { background: var(--accent); color: var(--accent-contrast); border-color: var(--accent); }
.btn-step.primary:hover:not(:disabled) { background: var(--accent-hover); }

.auto-sync-row { display: flex; align-items: center; gap: 8px; margin-top: 14px; padding-top: 12px; border-top: 1px solid var(--tag-gray-bg); cursor: pointer; }
.auto-sync-label { font-size: 12px; color: var(--text-secondary); }

/* Config form */
.config-form { padding: 12px 0 0; }
.field { display: flex; flex-direction: column; gap: 4px; margin-bottom: 10px; }
.field--row { flex-direction: row; align-items: center; gap: 8px; }
.field-label { font-size: 12px; font-weight: 500; color: var(--text-secondary); }
.field-input { padding: 8px 10px; border: 1px solid var(--border); border-radius: 9px; background: var(--bg-muted); font-family: var(--font-sans); font-size: 13px; color: var(--text-primary); outline: none; }
.field-input:focus { border-color: var(--accent); background: var(--bg-surface); }
.field-input::placeholder { color: var(--text-tertiary); }

.config-form-actions { display: flex; align-items: center; justify-content: space-between; margin-top: 4px; }
.config-form-right { display: flex; gap: 6px; }

.btn-test { padding: 6px 14px; border: 1px solid var(--border); border-radius: 8px; background: var(--bg-surface); color: var(--text-primary); font-family: var(--font-sans); font-size: 12px; cursor: pointer; }
.btn-test:hover:not(:disabled) { background: var(--surface-hover); }
.btn-test:disabled { opacity: 0.4; cursor: default; }

.btn-cancel { padding: 6px 14px; border: 1px solid var(--border); border-radius: 8px; background: var(--bg-surface); color: var(--text-secondary); font-family: var(--font-sans); font-size: 12px; cursor: pointer; }
.btn-cancel:hover { background: var(--surface-hover); }

.test-result { margin-top: 8px; padding: 8px 12px; border-radius: 5px; background: var(--tag-green-bg); color: var(--tag-green-text); font-size: 12px; line-height: 1.5; white-space: pre-wrap; }
.test-result.error { background: var(--tag-red-bg); color: var(--tag-red-text); }

/* Config list */
.config-list { display: flex; flex-direction: column; }
.config-item { display: flex; align-items: center; justify-content: space-between; padding: 12px 0; border-bottom: 1px solid var(--tag-gray-bg); }
.config-item:last-child { border-bottom: none; }
.config-item-main { flex: 1; min-width: 0; }
.config-item-header { display: flex; align-items: center; gap: 8px; }
.config-name { font-size: 13.5px; font-weight: 500; }
.config-badge { font-size: 10px; padding: 1px 6px; border-radius: 999px; background: var(--tag-green-bg); color: var(--tag-green-text); }
.config-detail { font-size: 11px; color: var(--text-tertiary); font-family: var(--font-mono); margin-top: 2px; display: block; }
.config-item-actions { display: flex; gap: 4px; flex-shrink: 0; }

.btn-ghost-sm { padding: 4px 8px; border: 1px solid var(--border); border-radius: 7px; background: var(--bg-surface); color: var(--text-secondary); font-family: var(--font-sans); font-size: 11px; cursor: pointer; }
.btn-ghost-sm:hover { background: var(--surface-hover); color: var(--text-primary); }
.btn-ghost-sm.danger:hover { background: var(--tag-red-bg); color: var(--tag-red-text); border-color: var(--tag-red-bg); }

.empty-hint { font-size: 13px; color: var(--text-tertiary); text-align: center; padding: 20px 0; }

.card--about { opacity: 0.6; }
.about-text { font-size: 12px; color: var(--text-secondary); line-height: 1.6; }
</style>
