<template>
  <div class="page-lessons-list">
    <div class="back">
      <i class="fa-pro"></i>
    </div>
    <div class="lessons-list">
      <div class="lesson" v-for="group in groups" @click="chooseGroup(group)">
        <i
          class="fa-pro xmark icon"
          style="
            position: absolute;
            right: 8px;
            top: 8px;
            font-size: 16px;
            z-index: 100;
          "
          @click.stop="deleteUnskilledWordsGroup(group)"
        ></i>
        <i class="fa-pro book-bookmark icon"></i>
        <p style="text-align: center; line-height: 1.2">
          {{ group.getFullYear() }}-{{ group.getMonth() }}-{{ group.getDate()
          }}<br />
          {{ group.getHours() }}:{{ group.getMinutes() }}:{{
            group.getSeconds()
          }}
        </p>
      </div>
      <div
        v-if="groups.length === 0 && !loading"
        style="width: 100%; text-align: center"
      >
        生词本为空，需要先进行拼写练习，出错的词将会显示在这里
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useStatesStore, Word } from "../../storage/states";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits<{
  jump: [name: "wordsList"];
}>();
const loading = ref(true);
const groups = ref<Date[]>([]);
const state = useStatesStore();

onMounted(() => {
  refresh();
});
function refresh() {
  invoke("load_unskilled_words_groups").then((res) => {
    let data = res as number[];
    loading.value = false;
    groups.value = data.map((value) => {
      return new Date(value);
    });
  });
}
function chooseGroup(group: Date) {
  invoke("load_unskilled_words", { time: group.getTime().toString() }).then(
    (res) => {
      state.currentLesson = 0;
      state.currentWordsList = res as Word[];
    },
  );
  emit("jump", "wordsList");
}

function deleteUnskilledWordsGroup(group: Date) {
  invoke("delete_unskilled_words_group", {
    group: group.getTime().toString(),
  }).then(() => {
    refresh();
  });
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
      position: relative;
      width: 120px;
      height: 100px;
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

      .xmark:hover {
        color: rgb(233, 40, 40);
      }

      .xmark:active {
        opacity: 0.9;
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
    }
  }
}
</style>
