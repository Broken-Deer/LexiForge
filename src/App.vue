<template>
  <div class="window" data-tauri-drag-region>
    <div class="title-bar" data-tauri-drag-region>
      <p style="margin-left: 32px" data-tauri-drag-region>
        <i
          class="fa-pro book"
          style="margin-right: 8px; font-weight: 200"
          data-tauri-drag-region
        ></i
        ><span data-tauri-drag-region>单词本</span>
      </p>
      <div class="page-buttons">
        <TitlebarItem
          title="背单词"
          icon="book-open-reader"
          @click="changePage('lessons')"
          id="titlebar-study"
        >
        </TitlebarItem>
        <TitlebarItem
          title="生词本"
          icon="bookmark"
          @click="changePage('history')"
          id="titlebar-review"
        >
        </TitlebarItem>
        <TitlebarItem
          title="关于"
          icon="circle-info"
          @click="changePage('about')"
          id="titlebar-about"
        >
        </TitlebarItem>
      </div>
      <div class="window-buttons">
        <div class="min" @click="window.getCurrentWindow().minimize()">
          <i></i>
        </div>
        <div class="close" @click="window.getCurrentWindow().close()">
          <i></i>
        </div>
      </div>
    </div>
  </div>
  <main class="main">
    <Transition :name="transitionName" mode="out-in">
      <component :is="currentComponent" @jump="jumpTo"></component>
    </Transition>
  </main>
</template>

<script setup lang="ts">
import { markRaw, reactive, ref, shallowRef } from "vue";
import About from "./pages/About.vue";
import TitlebarItem from "./components/TitlebarItem.vue";
import { window } from "@tauri-apps/api";
import LessonList from "./pages/study/LessonList.vue";
import WordsList from "./pages/study/WordsList.vue";
import Exercise from "./pages/study/Exercise.vue";
import History from "./pages/review/History.vue";

const pages = reactive({
  lessons: markRaw(LessonList),
  wordsList: markRaw(WordsList),
  exercise: markRaw(Exercise),
  history: markRaw(History),
  about: markRaw(About),
});
const currentComponent: any = shallowRef(pages.lessons);
const transitionName = ref("entrance");
function changePage(
  component: "lessons" | "wordsList" | "history" | "about" | "exercise",
) {
  if (
    component === "wordsList" &&
    currentComponent.value === pages["lessons"]
  ) {
    transitionName.value = "zoom-out";
  } else {
    transitionName.value = "entrance";
  }

  currentComponent.value = pages[component];
}

function jumpTo(
  name: "lessons" | "wordsList" | "history" | "about" | "exercise",
) {
  changePage(name);
}
</script>

<style>
main.main {
  position: absolute;
  right: 0;
  bottom: 0;
  height: calc(100vh - 56px);
  width: 100vw;
  border: 1px solid rgba(56, 59, 65, 1);
  border-bottom: unset;
  border-right: unset;
  background: rgba(44, 48, 57, 0.6);
}

.title-bar {
  height: 56px;
  width: 100%;
  position: absolute;
  display: flex;
  align-items: center;
  justify-content: space-between;

  .page-buttons {
    display: flex;
  }

  .window-buttons {
    display: flex;
    align-items: center;
    margin-right: 20px;
  }

  .window-buttons > div {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    margin-left: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform 100ms;
  }

  .window-buttons > div > i {
    font-style: normal;
    font-family: "fa-pro";
    font-weight: 500;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .window-button > i::before {
    line-height: 1;
    color: #ffffffb7;
    opacity: 0;
  }

  .window-buttons > div:hover > i::before {
    opacity: 1;
  }

  .window-buttons > div:active {
    transform: scale(0.9);
  }

  .window-buttons > div:active > i {
    opacity: 0.9;
  }

  .window-buttons > div {
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.12);
  }

  .window-buttons > div.min > i::before {
    content: "\f068";
    font-size: 12px;
    margin-top: 1px;
  }

  .window-buttons > div.close > i::before {
    content: "\f00d";
    font-size: 14px;
    margin-top: 1px;
    margin-left: 0.6px;
  }
}
</style>
