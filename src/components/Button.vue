<template>
  <button class="button" :style="style" v-bind="$attrs">
    <slot></slot>
  </button>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  disabled?: boolean;
  color?: string;
}>();
const style = computed(() => {
  let result = "";
  if (props.disabled) {
    result += "pointer-events: none;";
  }
  if (props.color) {
    result += `color: ${props.color}`;
  }
  return result;
});
</script>

<style lang="less" scoped>
button.button {
  width: 100%;
  height: 30px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 6px;
  margin-left: auto;
  padding: 10px 16px;
  display: flex;
  align-items: center;
  justify-content: center;
}

button.button:hover {
  background: rgba(255, 255, 255, 0.1);
}

button.button:active {
  opacity: 0.8;
}
</style>
