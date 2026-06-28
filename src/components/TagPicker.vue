<script setup lang="ts">
import { ref, computed } from 'vue'

const props = defineProps<{
  modelValue: string[]
  suggestions?: string[]
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string[]]
}>()

const input = ref('')
const showSuggestions = ref(false)

const defaultSuggestions = ['dev', 'ui', 'rust', 'docs', 'research', 'bug', 'feature', 'design', 'review']
const allSuggestions = computed(() => props.suggestions || defaultSuggestions)

const filtered = computed(() => {
  if (!input.value.trim()) return allSuggestions.value.filter(s => !props.modelValue.includes(s))
  return allSuggestions.value.filter(s =>
    s.toLowerCase().includes(input.value.toLowerCase()) && !props.modelValue.includes(s)
  )
})

function addTag(tag: string) {
  const t = tag.trim().toLowerCase()
  if (t && !props.modelValue.includes(t)) {
    emit('update:modelValue', [...props.modelValue, t])
  }
  input.value = ''
}

function removeTag(tag: string) {
  emit('update:modelValue', props.modelValue.filter(t => t !== tag))
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter' || e.key === ',') {
    e.preventDefault()
    addTag(input.value)
  }
  if (e.key === 'Backspace' && !input.value && props.modelValue.length > 0) {
    removeTag(props.modelValue[props.modelValue.length - 1])
  }
}
</script>

<template>
  <div class="tag-picker">
    <div class="tags-row">
      <span
        v-for="tag in modelValue"
        :key="tag"
        class="tag tag--selected"
      >
        {{ tag }}
        <button class="tag-remove" @click="removeTag(tag)">&times;</button>
      </span>
      <input
        v-model="input"
        class="tag-input"
        placeholder="添加标签..."
        @keydown="onKeydown"
        @focus="showSuggestions = true"
        @blur="showSuggestions = false"
      />
    </div>
    <div v-if="showSuggestions && filtered.length > 0" class="tag-suggestions">
      <button
        v-for="s in filtered.slice(0, 8)"
        :key="s"
        class="tag-suggestion"
        @mousedown.prevent="addTag(s)"
      >{{ s }}</button>
    </div>
  </div>
</template>

<style scoped>
.tag-picker {
  position: relative;
}

.tags-row {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 4px;
  padding: 5px 8px;
  border: 1px solid var(--border);
  border-radius: 8px;
  background: var(--bg-muted);
  min-height: 32px;
}

.tag--selected {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  padding: 1px 6px;
  border-radius: 6px;
  background: var(--tag-gray-bg);
  color: var(--text-secondary);
  font-size: 11px;
  font-weight: 500;
  text-transform: lowercase;
}

.tag-remove {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 14px;
  height: 14px;
  border: none;
  border-radius: 50%;
  background: transparent;
  color: var(--text-tertiary);
  font-size: 13px;
  cursor: pointer;
  line-height: 1;
}

.tag-remove:hover {
  background: var(--surface-active);
  color: var(--text-primary);
}

.tag-input {
  flex: 1;
  min-width: 80px;
  border: none;
  background: transparent;
  font-family: var(--font-sans);
  font-size: 12px;
  color: var(--text-primary);
  outline: none;
}

.tag-input::placeholder {
  color: var(--text-tertiary);
}

.tag-suggestions {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  z-index: 10;
  display: flex;
  flex-wrap: wrap;
  gap: 3px;
  padding: 6px 8px;
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 10px;
  margin-top: 3px;
  box-shadow: var(--shadow-dropdown);
}

.tag-suggestion {
  padding: 2px 8px;
  border: 1px solid var(--border);
  border-radius: 7px;
  background: var(--bg-surface);
  color: var(--text-secondary);
  font-family: var(--font-sans);
  font-size: 11px;
  cursor: pointer;
  transition: all 100ms ease;
}

.tag-suggestion:hover {
  border-color: var(--accent);
  color: var(--text-primary);
  background: var(--accent-soft);
}
</style>
