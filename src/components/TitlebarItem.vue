<template>
  <li class="titlebar-item">
    <i class="fa-pro icon" :class="icon"></i>
    <div class="text" v-if="title">{{ title }}</div>
  </li>
</template>

<script setup lang="ts">
defineProps<{
  title?: string;
  icon: string;
}>();
</script>

<style lang="less" scoped>
.titlebar-item {
  width: 88px;
  height: 26px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-left: 12px;
  border-radius: 100px;
  transition: all 100ms;
  background: rgba(255, 255, 255, 0.08);
  border: 1px solid rgba(255, 255, 255, 0.12);
}

.titlebar-item:hover {
  background: rgba(255, 255, 255, 0.08);
}

.titlebar-item:active {
  opacity: 0.6;
  transform: scale(0.97);
}

.titlebar-item .icon {
  font-size: 12px;
  display: inline;
  width: 30px;
}

.titlebar-item .text {
  font-size: 12px;
  margin-left: 2px;
  width: fit-content;
}

.titlebar-item i {
  transform: scale(1.2);
  font-family: "fa-pro";
  font-style: normal;
  height: 1em;
  width: 100%;
  text-align: center;
}
</style>
