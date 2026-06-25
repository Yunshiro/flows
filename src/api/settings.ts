import { tauriInvoke } from './adapter'
import type { AppSettings } from '../stores/useSettingsStore'

export const settingsApi = {
  get: () =>
    tauriInvoke<AppSettings>('get_settings', {}),

  save: (settings: AppSettings) =>
    tauriInvoke<void>('save_settings', { settings }),

  testLlm: (baseUrl: string, apiKey: string, model: string) =>
    tauriInvoke<string>('test_llm_connection', { baseUrl, apiKey, model }),
}
