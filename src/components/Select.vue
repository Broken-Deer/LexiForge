<template>
  <div class="select" tabindex="0" @blur="opened = false">
    <div class="selected" @click="toggleOpened()">{{ selected }}</div>
    <div>
      <Transition @before-enter="beforeEnter" @enter="enter" @after-enter="afterEnter" @before-leave="beforeLeave"
        @leave="leave" @after-leave="afterLeave">
        <ul class="options" v-if="opened" @click="opened = false">
          <div v-if="opened">
            <li class="select-option" v-for="(_, index) in options" :key="index" @click="changeSelection(index)">
              {{ options[index] }}
            </li>
          </div>
        </ul>
      </Transition>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import $ from "jquery";
const props = defineProps<{
  options: string[];
}>();
const model = defineModel();
let selected = ref("");
props.options.map((value, index) => {
  if (value == model.value) {
    selected.value = props.options[index];
  }
});
function beforeEnter(element: Element) {
  const html_element = element as HTMLElement;
  $(html_element.firstElementChild!).removeClass("hidden");
  html_element.style.transition = transitionStyle;
  html_element.style.height = "0px";
}

const transitionStyle = "all 200ms ease";
function enter(element: Element) {
  const html_element = element as HTMLElement;
  const height = $(html_element.firstElementChild!).outerHeight(true);
  html_element.style.height = `${height}px`;
  html_element.style.overflow = "hidden";
}
function afterEnter(element: Element) {
  const html_element = element as HTMLElement;
  html_element.style.transition = "";
  html_element.style.height = "";
  html_element.style.overflow = "";
}
function beforeLeave(element: Element) {
  const html_element = element as HTMLElement;
  html_element.style.transition = transitionStyle;
  const height = $(html_element.firstElementChild!).outerHeight(true);
  html_element.style.height = `${height}px`;
  html_element.style.overflow = "hidden";
}
function leave(element: Element) {
  const html_element = element as HTMLElement;
  html_element.style.height = "0px";
}
function afterLeave(element: Element) {
  const html_element = element as HTMLElement;
  html_element.style.transition = "";
  html_element.style.height = "";
}
function changeSelection(index: number) {
  selected.value = props.options[index];
  model.value = props.options[index];
}
let opened = ref(false);
function toggleOpened() {
  opened.value = !opened.value;
}
</script>

<style lang="less" scoped>
.select {
  width: 240px;
  height: 26px;
  display: inline-flex;
  flex-direction: column;
  justify-content: flex-start;
  font-size: 14px;
}

.select li {
  list-style: none;
}

.selected {
  width: 100%;
  border-radius: 6px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  padding: 8px 12px;
  transition: opacity 100ms ease;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: all 70ms ease;
  background: rgba(255, 255, 255, 0.05);
}

.selected:hover {
  background: rgba(255, 255, 255, 0.08);
}

.selected::after {
  content: "\f107";
  font-family: "fa-pro";
  width: fit-content;
  height: fit-content;
  margin-right: 2px;
  transition: transform 100ms ease;
}

.selected:hover::after {
  transform: translate(0px, 1px);
}

.selected:active {
  opacity: 0.8;
}

.options {
  width: 240px;
  margin-top: 4px;
  border-radius: 10px;
  border: 1px solid rgb(56, 59, 65);
  background: rgb(27, 28, 31);
  box-shadow: 0px 0px 10px #4500611d;
  transform: scale3d(1, 1, 192.7);
  font-size: 14px;
  z-index: 100000;
  display: flex;
  align-items: flex-end;
}

.options>div:first-child {
  margin: 10px 12px;
  width: 100%;
}

.select-option {
  padding: 10px 16px;
  border-radius: 6px;
  z-index: 10001;
  transition: all 30ms ease;
}

.select-option:hover {
  background: #ffffff1f;
}

.select-option:active {
  background: #ffffff15;
}
</style>
