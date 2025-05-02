<template>
  <div class="page-exercise">
    <Transition name="zoom-out" mode="out-in">
      <div v-if="currentWord < wordList.length" class="container">
        <div class="spell">
          <TextInputBox :error="buttonType == 'xmark'" :disabled="buttonType != 'submit'" v-model="inputWord"
            @keyup="onInputKeyup"></TextInputBox>
          <ButtonVue :disabled="buttonType != 'submit'" style="height: 50px; width: 50px; margin-left: 8px"
            @click="checkSpell">
            <Transition name="zoom-out" mode="out-in">
              <i v-if="buttonType === 'check'" class="fa-pro check"
                style="font-size: 20px; color: rgb(40, 233, 40)"></i>
              <i v-else-if="buttonType === 'xmark'" class="fa-pro xmark"
                style="font-size: 24px; color: rgb(233, 40, 40)"></i>
              <i v-else class="fa-pro arrow-right" style="font-size: 20px"></i>
            </Transition>
          </ButtonVue>
        </div>
        <h4 class="translation" v-html="getTranslationHTML(wordList[currentWord])"></h4>
      </div>
      <div v-else class="container-finish">
        <i class="fa-pro party-horn" style="font-weight: 200; font-size: 50px" </i>
          <p v-if="state.currentLesson > 0" style="margin-top: 16px">
            太棒了！第 {{ state.currentLesson }} 组单词已完成
          </p>
          <p v-else>
            太棒了！本次练习已完成
          </p>
          <p v-if="wrongWords.length === 0" style="margin-top: 8px">
            本次练习中所有单词都拼写正确
          </p>
          <p v-else style="margin-top: 8px">
            你可以在生词本中继续练习刚才拼错的词
          </p>
          <ButtonVue @click="$emit('jump', 'lessons')" style="margin-top: 16px">返回首页</ButtonVue>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { onUnmounted, ref, watch } from "vue";
import { useStatesStore, Word } from "../../storage/states";
import TextInputBox from "../../components/TextInputBox.vue";
import ButtonVue from "../../components/Button.vue";
import { invoke } from "@tauri-apps/api/core";

defineEmits<{
  jump: [name: "lessons"];
}>();

const currentWord = ref(0);

const state = useStatesStore();
const wordList = ref(state.currentWordsList);

function getTranslationHTML(word: Word) {
  let result = "";
  word.translations.forEach((value) => {
    result += `<i>${value.type}.</i> ${value.translation}<br>`;
  });
  return result;
}

const inputWord = ref("");
const buttonType = ref<"check" | "xmark" | "submit">("submit");

function onInputKeyup(event: any) {
  deadline = 50;
  if (buttonType.value != "submit") {
    return;
  }
  event.preventDefault();
  if (event.keyCode === 13) {
    checkSpell();
  }
}

let deadline = 50;
const timer = setInterval(() => {
  if (deadline >= 0) {
    deadline--;
  } else {
    deadline = 1000000;
    checkSpell();
  }
}, 100);

onUnmounted(() => {
  clearInterval(timer);
});

const wrongWords = ref<Word[]>([]);
function checkSpell() {
  const correctly = inputWord.value === wordList.value[currentWord.value].word;
  if (correctly) {
    buttonType.value = "check";
    setTimeout(() => {
      buttonType.value = "submit";
      currentWord.value++;
      inputWord.value = "";
      deadline = 50;
    }, 800);
  } else {
    inputWord.value = wordList.value[currentWord.value].word;
    buttonType.value = "xmark";
    let counter = 0;
    wordList.value.forEach((value, index) => {
      if (
        value === wordList.value[currentWord.value] &&
        index > currentWord.value
      ) {
        counter++;
      }
    });
    if (counter <= 5) {
      wordList.value.splice(
        currentWord.value + 3,
        0,
        wordList.value[currentWord.value],
      );
      wordList.value.splice(
        currentWord.value + 10,
        0,
        wordList.value[currentWord.value],
      );
      wordList.value.push(wordList.value[currentWord.value]);
    }
    wrongWords.value.push(wordList.value[currentWord.value]);
    setTimeout(() => {
      inputWord.value = "";
      buttonType.value = "submit";
      deadline = 50;
    }, 2000);
  }
}

watch(currentWord, (value) => {
  if (value >= wordList.value.length) {
    if (wrongWords.value.length && state.currentLesson > 0) {
      const words = Array.from(new Set(wrongWords.value));
      invoke("save_unskilled_words", { words });

    }
  }
})

</script>

<style lang="less" scoped>
.page-exercise {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;

  .container {
    width: 560px;
    display: flex;
    flex-direction: column;
    align-items: center;

    .spell {
      display: flex;
    }

    .translation {
      margin-top: 16px;
      font-weight: normal;
      font-size: 18px;
      line-height: 1.5;
    }
  }

  .container-finish {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
}
</style>
