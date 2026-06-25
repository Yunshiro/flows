import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { noteApi } from '../api/notes'

export interface Note {
  path: string
  title: string
  content: string
  tags: string[]
  links: string[]
  createdAt: string
  updatedAt: string
}

export interface NoteTreeNode {
  name: string
  path: string
  isDir: boolean
  children: NoteTreeNode[]
}

export const useNotesStore = defineStore('notes', () => {
  const notes = ref<Note[]>([])
  const tree = ref<NoteTreeNode[]>([])
  const currentNote = ref<Note | null>(null)
  const loading = ref(false)
  const searchResults = ref<Note[]>([])

  const recentNotes = computed(() => {
    return [...notes.value]
      .sort((a, b) => b.updatedAt.localeCompare(a.updatedAt))
      .slice(0, 10)
  })

  async function fetchNotes(dir?: string) {
    loading.value = true
    try {
      tree.value = await noteApi.listTree(dir)
    } catch (e) {
      console.error('fetchNotes failed:', e)
    } finally {
      loading.value = false
    }
  }

  async function readNote(path: string) {
    try {
      const note = await noteApi.read(path)
      currentNote.value = note
      return note
    } catch (e) {
      console.error('readNote failed:', e)
      return null
    }
  }

  async function saveNote(path: string, content: string) {
    try {
      const note = await noteApi.save(path, content)
      const idx = notes.value.findIndex(n => n.path === path)
      if (idx !== -1) {
        notes.value[idx] = note
      } else {
        notes.value.push(note)
      }
      currentNote.value = note
      return note
    } catch (e) {
      console.error('saveNote failed:', e)
      return null
    }
  }

  async function createNote(dir: string, title: string) {
    try {
      const note = await noteApi.create(dir, title)
      notes.value.push(note)
      return note
    } catch (e) {
      console.error('createNote failed:', e)
      return null
    }
  }

  async function deleteNote(path: string) {
    try {
      await noteApi.delete(path)
      notes.value = notes.value.filter(n => n.path !== path)
      if (currentNote.value?.path === path) currentNote.value = null
    } catch (e) {
      console.error('deleteNote failed:', e)
    }
  }

  async function searchNotes(query: string) {
    try {
      searchResults.value = await noteApi.search(query)
      return searchResults.value
    } catch (e) {
      console.error('searchNotes failed:', e)
      return []
    }
  }

  return {
    notes,
    tree,
    currentNote,
    loading,
    searchResults,
    recentNotes,
    fetchNotes,
    readNote,
    saveNote,
    createNote,
    deleteNote,
    searchNotes,
  }
})
