<script setup lang="ts">
import { ref, nextTick } from 'vue'

const props = defineProps<{
  modelValue: string
  placeholder?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
  'save': [value: string]
  'cancel': []
}>()

const editing = ref(false)
const draft = ref('')
const inputRef = ref<HTMLInputElement>()

function startEdit() {
  draft.value = props.modelValue
  editing.value = true
  nextTick(() => inputRef.value?.focus())
}

function confirm() {
  const val = draft.value.trim()
  if (val) {
    emit('update:modelValue', val)
    emit('save', val)
  }
  editing.value = false
}

function cancel() {
  editing.value = false
  emit('cancel')
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') confirm()
  if (e.key === 'Escape') cancel()
}

defineExpose({ startEdit })
</script>

<template>
  <div class="inline-edit" @dblclick="startEdit">
    <input
      v-if="editing"
      ref="inputRef"
      v-model="draft"
      class="inline-input"
      :placeholder="placeholder"
      @blur="confirm"
      @keydown="onKeydown"
    />
    <span v-else class="inline-text"><slot>{{ modelValue }}</slot></span>
  </div>
</template>

<style scoped>
.inline-edit {
  cursor: text;
}

.inline-text {
  display: block;
}

.inline-input {
  width: 100%;
  padding: 2px 4px;
  margin: -2px 0;
  border: 1px solid #111111;
  border-radius: 4px;
  font-family: var(--font-sans);
  font-size: inherit;
  color: #111111;
  background: var(--bg-surface);
  outline: none;
}
</style>
