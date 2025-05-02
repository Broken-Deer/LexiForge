import { defineStore } from "pinia";

export type Word = {
  word: string;
  translations: {
    translation: string;
    type: string;
  }[];
  phrases?: {
    phrase: string;
    translation: string;
  }[];
}
export type State = {
  currentLesson: number
  currentWordsList: Word[]
  currentVocabulary: string
}

export const useStatesStore = defineStore("global_state", {
  state: (): State => {
    return {
      currentLesson: 0,
      currentWordsList: [],
      currentVocabulary: "高中"
    }
  }
})
