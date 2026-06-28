import { defineStore } from 'pinia'
import { ref } from 'vue'
import { settingsApi } from '../api/settings'

export interface AppSettings {
  notesPath: string
  gitRemoteUrl: string
  gitBranch: string
  autoSyncEnabled: boolean
  autoSyncIntervalMinutes: number
  theme: 'system' | 'light' | 'dark'
  llmBaseUrl: string
  llmApiKey: string
  llmModel: string
}

const defaults: AppSettings = {
  notesPath: '',
  gitRemoteUrl: '',
  gitBranch: '',
  autoSyncEnabled: false,
  autoSyncIntervalMinutes: 30,
  theme: 'system',
  llmBaseUrl: 'https://api.openai.com/v1',
  llmApiKey: '',
  llmModel: 'deepseek-chat',
}

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<AppSettings>({ ...defaults })
  const loaded = ref(false)

  async function loadSettings() {
    try {
      const saved = await settingsApi.get()
      if (saved) {
        settings.value = { ...defaults, ...saved }
      }
      loaded.value = true
    } catch (e) {
      console.error('loadSettings failed:', e)
      loaded.value = true
    }
  }

  async function saveSettings(changes: Partial<AppSettings>) {
    try {
      Object.assign(settings.value, changes)
      await settingsApi.save(settings.value)
    } catch (e) {
      console.error('saveSettings failed:', e)
    }
  }

  return {
    settings,
    loaded,
    defaults,
    loadSettings,
    saveSettings,
  }
})
