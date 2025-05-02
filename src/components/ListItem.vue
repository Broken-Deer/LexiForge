<template>
  <div class="list-item">
    <div class="title">
      <div class="icon" v-if="icon"><i :class="icon"></i></div>
      <div>
        <h4 id="text">{{ title }}</h4>
        <p v-if="description" id="text" style="max-width: 560px" v-html="description"></p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  title: string;
  description?: string;
  icon?: string;
}>();
</script>

<style lang="less" scoped>
.list-item {
  padding: 16px 18px 16px 14px;
  display: flex;
  align-items: center;
  margin: 0;
  margin-bottom: 1px;
  background: rgba(255, 255, 255, 0.08);
  transition: all 50ms ease;

  .title {
    display: flex;

    >div {
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
    }

    >div:last-child {
      align-items: flex-start;
    }

    h4 {
      font-weight: normal;
      height: 20.5px;
      font-size: 15px;
      line-height: 20.5px;
    }

    p {
      font-size: 12.5px;
      opacity: 0.8;
      margin-top: 4px;
      line-height: 1.4;
      word-break: break-all;
    }
  }

  .icon {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    width: 40px;
    height: inherit;
    margin-right: 8px;
  }

  .icon i {
    margin: 0;
  }
}
</style>
