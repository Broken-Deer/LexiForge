<template>
  <div class="page-about">
    <div class="container">
      <div
        style="
          display: flex;
          flex-direction: column;
          align-items: center;
          background: rgba(255, 255, 255, 0.08);
          padding: 24px 18px 16px 14px;
          border-radius: 8px;
          margin-bottom: 16px;
        "
      >
        <i class="fa-pro book" style="font-size: 40px; margin-bottom: 16px"></i>
        <p style="font-size: 20px">单词本</p>
        <p style="font-size: 14px; opacity: 0.7">版本：0.1.0 作者：陈子璐</p>
      </div>
      <p>用到的第三方库或框架</p>
      <div class="group">
        <ListItem
          title="Tauri"
          description="Copyright 2022 Tauri Programme within The Commons Conservancy<br> Licensed under the Apache-2.0 License or MIT Licenses"
          icon="tauri"
        ></ListItem>
        <ListItem
          title="Vue"
          description="Copyright (c) 2018-present, Yuxi (Evan) You and Vue contributors<br> Licensed under the MIT License"
          icon="vuejs"
        ></ListItem>
        <ListItem
          title="Serde & Serde JSON"
          description="Licensed under Apache-2.0 or MIT license."
          icon="serde"
        >
        </ListItem>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import ListItem from "../components/ListItem.vue";
</script>

<style lang="less" scoped>
.page-about {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  display: flex;
  justify-content: center;
  padding-top: 24px;

  padding-bottom: 24px;

  .container {
    width: 580px;

    p {
      margin-bottom: 8px;
    }

    div.group {
      border-radius: 8px;
      overflow: hidden;
      height: fit-content;
      margin-bottom: 16px;
    }
  }
}
</style>
