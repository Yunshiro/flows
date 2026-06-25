<script setup lang="ts">
import { useSettingsStore } from '../stores/useSettingsStore'
import { onMounted, onUnmounted, ref } from 'vue'
import { noteApi } from '../api/notes'
import { settingsApi } from '../api/settings'
import { llmApi, type LlmConfig } from '../api/llm'
import GitPushDialog from '../components/GitPushDialog.vue'

const store = useSettingsStore()
const saved = ref(false)
const gitStatus = ref('')
const gitInited = ref(false)
const gitHasRemote = ref(false)
const syncing = ref(false)
const showPushDialog = ref(false)
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
    gitHasRemote.value = s.includes('干净') || s.includes('待同步')
    if (gitInited.value) {
      const remoteUrl = await noteApi.gitGetRemote()
      if (remoteUrl && !store.settings.gitRemoteUrl) store.settings.gitRemoteUrl = remoteUrl
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

async function handleInit() {
  try { const msg = await noteApi.gitInit(); gitStatus.value = msg; gitInited.value = true } catch (e) { gitStatus.value = '失败: ' + String(e) }
}

async function handleConfigRemote() {
  const url = store.settings.gitRemoteUrl.trim()
  if (!url) return
  try { await noteApi.gitSetRemote(url); await checkGitState() } catch (e) { gitStatus.value = '失败: ' + String(e) }
}

async function handlePull() {
  syncing.value = true
  try { await noteApi.gitPull(); await checkGitState() } catch (e) { alert('拉取失败: ' + String(e)) } finally { syncing.value = false }
}

function onPushed() { showPushDialog.value = false; checkGitState() }

function startAutoSync() {
  stopAutoSync()
  const minutes = Math.max(1, store.settings.autoSyncIntervalMinutes || 30)
  syncTimer = setInterval(async () => {
    try { await noteApi.gitPull(); await noteApi.gitPush(); gitStatus.value = '自动同步 ' + new Date().toLocaleTimeString('zh-CN', { hour:'2-digit', minute:'2-digit' }) } catch { /* */ }
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
  } catch (e) { alert('保存失败: ' + String(e)) }
}

async function handleDeleteConfig(id: string) {
  try { await llmApi.delete(id); await loadLlmConfigs() } catch (e) { alert('删除失败: ' + String(e)) }
}

async function handleSetDefault(id: string) {
  const cfg = llmConfigs.value.find(c => c.id === id)
  if (!cfg) return
  try {
    await llmApi.save({ ...cfg, isDefault: true })
    await loadLlmConfigs()
  } catch (e) { alert('设置失败: ' + String(e)) }
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
          <span v-else class="step-done-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#346538" stroke-width="2.5"><path d="M5 13l4 4L19 7"/></svg></span>
        </div>
        <div class="git-step" :class="{ disabled: !gitInited }">
          <span class="git-step-num">2</span>
          <div class="git-step-body">
            <span class="git-step-label">配置远程仓库地址</span>
            <div class="git-remote-row">
              <input v-model="store.settings.gitRemoteUrl" class="git-remote-input" placeholder="https://github.com/you/notes.git" :disabled="!gitInited" />
              <button class="btn-step" :disabled="!gitInited || !store.settings.gitRemoteUrl.trim()" @click="handleConfigRemote">配置</button>
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
            <button class="btn-step" :disabled="!gitHasRemote || syncing" @click="handlePull">拉取</button>
            <button class="btn-step primary" :disabled="!gitHasRemote || syncing" @click="showPushDialog = true">推送</button>
          </div>
        </div>
      </div>

      <label class="auto-sync-row">
        <input type="checkbox" v-model="store.settings.autoSyncEnabled" :disabled="!gitHasRemote" />
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

    <GitPushDialog v-if="showPushDialog" @close="showPushDialog = false" @pushed="onPushed" />
  </div>
</template>

<style scoped>
.page { max-width: 600px; }
.page-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 20px; }
.page-title { font-size: 26px; font-weight: 620; letter-spacing: -0.025em; }

.btn-save { padding: 7px 18px; border: none; border-radius: 6px; background: #111111; color: #FFFFFF; font-family: var(--font-sans); font-size: 13px; font-weight: 520; cursor: pointer; }
.btn-save:hover { background: #333333; }

.card { background: #FFFFFF; border: 1px solid #EAEAEA; border-radius: 8px; padding: 20px; margin-bottom: 16px; }
.card-title { font-size: 14px; font-weight: 580; margin-bottom: 4px; }
.card-desc { font-size: 12px; color: #9E9E9E; margin-bottom: 16px; }
.card-desc code { font-family: var(--font-mono); font-size: 11px; background: #F1F1EF; padding: 1px 4px; border-radius: 3px; }
.card-header-row { display: flex; align-items: flex-start; justify-content: space-between; margin-bottom: 8px; }

.btn-add { padding: 6px 14px; border: 1px solid #EAEAEA; border-radius: 6px; background: #FFFFFF; color: #111111; font-family: var(--font-sans); font-size: 13px; cursor: pointer; white-space: nowrap; }
.btn-add:hover { background: rgba(0,0,0,0.03); }

/* Git */
.git-status-bar { display: flex; align-items: center; gap: 0; margin-bottom: 16px; padding: 10px 14px; background: #F7F6F3; border-radius: 6px; }
.step-dot { width: 22px; height: 22px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 11px; font-weight: 600; background: #EAEAEA; color: #9E9E9E; flex-shrink: 0; }
.step-dot.done { background: #346538; color: #FFFFFF; }
.step-line { flex: 1; height: 2px; background: #EAEAEA; margin: 0 4px; }
.step-line.done { background: #346538; }
.step-text { margin-left: 10px; font-size: 12px; color: #787774; flex-shrink: 0; }

.git-steps { display: flex; flex-direction: column; }
.git-step { display: flex; align-items: flex-start; gap: 12px; padding: 12px 0; border-bottom: 1px solid #F1F1EF; }
.git-step.disabled { opacity: 0.4; pointer-events: none; }
.git-step-num { width: 20px; height: 20px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 11px; font-weight: 600; background: #F1F1EF; color: #787774; flex-shrink: 0; }
.git-step-body { flex: 1; min-width: 0; }
.git-step-label { font-size: 13px; font-weight: 520; display: block; }
.git-step-desc { font-size: 11px; color: #9E9E9E; margin-top: 2px; display: block; }
.step-done-icon { display: flex; align-items: center; flex-shrink: 0; }
.git-remote-row { display: flex; gap: 6px; margin-top: 6px; }
.git-remote-input { flex: 1; padding: 5px 8px; border: 1px solid #EAEAEA; border-radius: 5px; font-family: var(--font-mono); font-size: 12px; color: #111111; background: #F7F6F3; outline: none; }
.git-remote-input:focus { border-color: #787774; }
.git-remote-input:disabled { opacity: 0.5; }
.git-remote-input::placeholder { color: #CECECE; }
.git-step-actions { display: flex; gap: 6px; flex-shrink: 0; }

.btn-step { padding: 5px 12px; border: 1px solid #EAEAEA; border-radius: 5px; background: #FFFFFF; color: #111111; font-family: var(--font-sans); font-size: 12px; cursor: pointer; white-space: nowrap; transition: all 100ms ease; }
.btn-step:hover:not(:disabled) { background: rgba(0,0,0,0.04); }
.btn-step:disabled { opacity: 0.4; cursor: default; }
.btn-step.primary { background: #111111; color: #FFFFFF; border-color: #111111; }
.btn-step.primary:hover:not(:disabled) { background: #333333; }

.auto-sync-row { display: flex; align-items: center; gap: 8px; margin-top: 14px; padding-top: 12px; border-top: 1px solid #F1F1EF; cursor: pointer; }
.auto-sync-label { font-size: 12px; color: #787774; }

/* Config form */
.config-form { padding: 12px 0 0; }
.field { display: flex; flex-direction: column; gap: 4px; margin-bottom: 10px; }
.field--row { flex-direction: row; align-items: center; gap: 8px; }
.field-label { font-size: 12px; font-weight: 520; color: #787774; }
.field-input { padding: 7px 10px; border: 1px solid #EAEAEA; border-radius: 6px; background: #F7F6F3; font-family: var(--font-sans); font-size: 13px; color: #111111; outline: none; }
.field-input:focus { border-color: #787774; }
.field-input::placeholder { color: #9E9E9E; }

.config-form-actions { display: flex; align-items: center; justify-content: space-between; margin-top: 4px; }
.config-form-right { display: flex; gap: 6px; }

.btn-test { padding: 6px 14px; border: 1px solid #EAEAEA; border-radius: 5px; background: #FFFFFF; color: #111111; font-family: var(--font-sans); font-size: 12px; cursor: pointer; }
.btn-test:hover:not(:disabled) { background: rgba(0,0,0,0.03); }
.btn-test:disabled { opacity: 0.4; cursor: default; }

.btn-cancel { padding: 6px 14px; border: 1px solid #EAEAEA; border-radius: 5px; background: #FFFFFF; color: #787774; font-family: var(--font-sans); font-size: 12px; cursor: pointer; }
.btn-cancel:hover { background: rgba(0,0,0,0.03); }

.test-result { margin-top: 8px; padding: 8px 12px; border-radius: 5px; background: #EDF3EC; color: #346538; font-size: 12px; line-height: 1.5; white-space: pre-wrap; }
.test-result.error { background: #FDEBEC; color: #9F2F2D; }

/* Config list */
.config-list { display: flex; flex-direction: column; }
.config-item { display: flex; align-items: center; justify-content: space-between; padding: 12px 0; border-bottom: 1px solid #F1F1EF; }
.config-item:last-child { border-bottom: none; }
.config-item-main { flex: 1; min-width: 0; }
.config-item-header { display: flex; align-items: center; gap: 8px; }
.config-name { font-size: 13.5px; font-weight: 520; }
.config-badge { font-size: 10px; padding: 1px 6px; border-radius: 999px; background: #EDF3EC; color: #346538; }
.config-detail { font-size: 11px; color: #9E9E9E; font-family: var(--font-mono); margin-top: 2px; display: block; }
.config-item-actions { display: flex; gap: 4px; flex-shrink: 0; }

.btn-ghost-sm { padding: 4px 8px; border: 1px solid #EAEAEA; border-radius: 4px; background: #FFFFFF; color: #787774; font-family: var(--font-sans); font-size: 11px; cursor: pointer; }
.btn-ghost-sm:hover { background: rgba(0,0,0,0.03); color: #111111; }
.btn-ghost-sm.danger:hover { background: #FDEBEC; color: #9F2F2D; border-color: #FDEBEC; }

.empty-hint { font-size: 13px; color: #9E9E9E; text-align: center; padding: 20px 0; }

.card--about { opacity: 0.6; }
.about-text { font-size: 12px; color: #787774; line-height: 1.6; }
</style>
