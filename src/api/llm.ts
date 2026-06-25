import { tauriInvoke } from './adapter'

export interface LlmConfig {
  id: string
  name: string
  baseUrl: string
  apiKey: string
  model: string
  isDefault: boolean
  createdAt: string
}

export const llmApi = {
  list: () =>
    tauriInvoke<LlmConfig[]>('list_llm_configs', {}),

  save: (config: Partial<LlmConfig> & { name: string; baseUrl: string; apiKey: string; model: string }) =>
    tauriInvoke<LlmConfig>('save_llm_config', {
      id: config.id || null,
      name: config.name,
      baseUrl: config.baseUrl,
      apiKey: config.apiKey,
      model: config.model,
      isDefault: config.isDefault || false,
    }),

  delete: (id: string) =>
    tauriInvoke<void>('delete_llm_config', { id }),

  get: (id: string) =>
    tauriInvoke<LlmConfig>('get_llm_config', { id }),
}
