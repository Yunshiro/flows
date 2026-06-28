import { tauriInvoke } from './adapter'
import type { Note, NoteTreeNode } from '../stores/useNotesStore'

export const noteApi = {
  listTree: (dir?: string) =>
    tauriInvoke<NoteTreeNode[]>('list_notes', { dir: dir || null }),

  read: (path: string) =>
    tauriInvoke<Note>('read_note', { path }),

  save: (path: string, content: string) =>
    tauriInvoke<Note>('save_note', { path, content }),

  create: (dir: string, title: string) =>
    tauriInvoke<Note>('create_note', { dir, title }),

  delete: (path: string) =>
    tauriInvoke<void>('delete_note', { path }),

  search: (query: string) =>
    tauriInvoke<Note[]>('search_notes', { query }),

  gitInit: () =>
    tauriInvoke<string>('git_init', {}),

  gitGetRemote: () =>
    tauriInvoke<string>('git_get_remote', {}),

  gitSetRemote: (url: string) =>
    tauriInvoke<string>('git_set_remote', { url }),

  gitCheckoutBranch: (branch: string) =>
    tauriInvoke<string>('git_checkout_branch', { branch }),

  gitListBranches: () =>
    tauriInvoke<string[]>('git_list_branches', {}),

  gitPush: (branch: string) =>
    tauriInvoke<string>('git_push', { branch }),

  gitPull: (branch: string) =>
    tauriInvoke<string>('git_pull', { branch }),

  gitStatus: () =>
    tauriInvoke<string>('git_status', {}),

  gitListFiles: () =>
    tauriInvoke<{ status: string; path: string }[]>('git_list_files', {}),
}
