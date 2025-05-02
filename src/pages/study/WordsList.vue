<template>
  <div class="page-words-list">
    <div class="words">
      <ListItem v-for="word in state.currentWordsList" :title="word.word" :description="getTranslationHTML(word)">
      </ListItem>
      <div class="buttons">
        <ButtonVue @click="$emit('jump', 'exercise')">练习</ButtonVue>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import ListItem from "../../components/ListItem.vue";
import ButtonVue from "../../components/Button.vue";
import { useStatesStore, Word } from "../../storage/states";

const state = useStatesStore();

defineEmits<{
  jump: [name: "lessons" | "exercise"];
}>();

function getTranslationHTML(word: Word) {
  let result = "";
  word.translations.forEach((value) => {
    result += `<i>${value.type}.</i> ${value.translation} `;
  });
  return result;
}
</script>

<style lang="less" scoped>
.page-words-list {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  display: flex;
  justify-content: center;

  padding-top: 24px;

  .words {
    width: 580px;
    height: fit-content;
    border-radius: 8px;
    overflow: hidden;
  }

  .buttons {
    display: flex;
    margin-top: 16px;
    margin-bottom: 24px;
  }
}
</style>
