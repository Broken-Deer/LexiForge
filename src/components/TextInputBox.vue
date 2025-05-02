<template>
  <div class="input-box">
    <input
      type="text"
      :placeholder="placeholder"
      required
      v-model="model"
      :disabled="disabled"
      ref="input"
    />
  </div>
</template>

<script setup lang="ts">
import { useTemplateRef, watchEffect } from "vue";

const props = defineProps<{
  name?: string;
  placeholder?: string;
  error?: boolean;
  disabled: boolean;
}>();

const model = defineModel();

const input = useTemplateRef("input");

watchEffect(() => {
  if (props.disabled === false) {
    setTimeout(() => {
      input.value?.focus();
    }, 100);
  }
});
</script>

<style scoped>
.input-box {
  border-radius: 6px;
  overflow: hidden;
  height: 48px;
  width: 320px;
  padding: 0;
  transition: all 100ms ease;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.input-box input {
  border: none;
  background-color: #00000000;
  padding: 0;
  height: 100%;
  width: 100%;
  margin: 0;
  padding: 2px 12px;
  font-size: 20px;
  text-align: inherit;
}

.input-box input::placeholder {
  color: rgba(255, 255, 255, 0.8);
}

.input-box:hover {
  background: rgba(255, 255, 255, 0.08);
}

.input-box:focus-within {
  background-color: rgba(255, 255, 255, 0.05);
}
</style>
