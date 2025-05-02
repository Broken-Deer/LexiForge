<template>
  <KeepAlive>
    <div class="page-lessons-list">
      <div class="lessons-list">
        <div style="margin-bottom: 16px">
          <span>词库：</span>
          <SelectVue
            :options="['初中', '高中', 'CET4', 'CET6', '考研', '托福']"
            v-model="state.currentVocabulary"
          ></SelectVue>
        </div>
        <div
          class="lesson"
          v-for="(lesson, index) in lessons"
          @click="chooseLesson(index + 1)"
        >
          {{ lesson }}
        </div>
      </div>
    </div>
  </KeepAlive>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { useStatesStore, Word } from "../../storage/states";
import { invoke } from "@tauri-apps/api/core";
import SelectVue from "../../components/Select.vue";

const emit = defineEmits<{
  jump: [name: "wordsList"];
}>();

const lessons = ref<string[]>([]);
const state = useStatesStore();

let oldVocabulary = "";
watch(state, async (value) => {
  let vocabulary = value.currentVocabulary;
  if (oldVocabulary === vocabulary) {
    return;
  }
  await refreshLessons(vocabulary);
  oldVocabulary = vocabulary;
});

async function refreshLessons(vocabulary: string) {
  lessons.value = [];
  const wordsCount = (await invoke("get_words_count", {
    vocabulary,
  })) as number;
  const lessonsCount = Math.ceil(wordsCount / 30);
  for (let index = 1; index <= lessonsCount; index++) {
    lessons.value.push(`第 ${index} 组`);
  }
}

refreshLessons(state.currentVocabulary);

async function chooseLesson(lessonId: number) {
  state.currentLesson = lessonId;
  const start = (state.currentLesson - 1) * 30;
  let end = state.currentLesson * 30 - 1;
  const wordsCount = (await invoke("get_words_count", {
    vocabulary: state.currentVocabulary,
  })) as number;
  if (end > wordsCount - 1) {
    end = wordsCount - 1;
  }
  invoke("get_words", {
    vocabulary: state.currentVocabulary,
    start,
    end,
  }).then((res) => {
    state.currentWordsList = res as Word[];
  });
  emit("jump", "wordsList");
}
</script>

<style lang="less" scoped>
.page-lessons-list {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  display: flex;
  justify-content: center;
  padding-top: 24px;

  .lessons-list {
    width: calc(8px * 10 + 10px + 120px * 5);
    height: fit-content;
    // display: flex;
    white-space: normal;

    .lesson {
      width: 120px;
      height: 60px;
      background: rgba(255, 255, 255, 0.03);
      border: 1px solid rgba(255, 255, 255, 0.1);
      border-radius: 8px;
      margin: 8px;
      display: inline-flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      transition: all 50ms ease;

      .icon {
        font-size: 22px;
      }

      p {
        margin-top: 8px;
        font-size: 14px;
      }
    }

    .lesson:hover {
      background: rgba(255, 255, 255, 0.06);
    }

    .lesson:active {
      background: rgba(255, 255, 255, 0.02);
      transform: scale(0.98);
    }
  }
}
</style>
