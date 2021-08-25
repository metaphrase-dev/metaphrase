<template>
  <div :class="[`translation flex`, { edited: modified }]" :t-id="translation?.id">
    <div class="translation-locale-label w-16 px-2 py-2 text-right font-mono text-sm font-bold">
      {{ translation?.locale }}
    </div>
    <div class="translation-editor relative bg-white flex flex-col flex-grow">
      <textarea
        :class="`flex-grow px-3 py-2 pb-6 text-sm outline-none transition-colors ${
          modified ? 'bg-yellow-50' : ''
        }`"
        :lang="translation?.locale"
        v-model="localContent"
        ref="translationTextarea"
        @keyup.esc="revertOrBlur"
        @keydown.enter="saveIfCmdOrCtrl($event)"
        @keydown.s="saveIfCmdOrCtrl($event)"
      ></textarea>
      <transition name="hint-animation">
        <span
          class="keyboard-hint font-light text-xs absolute bottom-0 pb-2 pl-3 text-yellow-800"
          v-if="modified"
        >
          <b class="font-medium">{{ mac ? "⌘+" : "Ctrl-" }}Enter</b> saves your changes.
          <b class="font-medium">Escape</b> discards them.
        </span>
      </transition>
      <transition name="hint-animation">
        <span
          class="just-saved flex items-center font-light text-xs absolute bottom-0 pb-2 pl-3 text-green-400"
          v-if="justSaved"
        >
          <span>Saved successfully!</span>
        </span>
      </transition>
    </div>
  </div>
</template>

<script>
import Store from "../../store";

const $store = new Store();
const mac = /(^Mac|^iP)/i.test(navigator.platform);

export default {
  name: "translation-locale",

  beforeUpdate() {
    this.$nextTick(() => {
      if (this.contentForKey !== this.translationKey) {
        this.localContent = this.translation.content;
        this.contentForKey = this.translationKey;
      }
    });
  },

  data() {
    return {
      localContent: this.translation.content,
      contentForKey: this.translationKey,
      justSaved: false,
    };
  },

  props: {
    translation: Object,
    translationKey: String,
  },

  computed: {
    modified() {
      return String(this.localContent).valueOf() !== String(this.translation.content).valueOf();
    },

    mac() {
      return mac;
    },
  },

  methods: {
    revertOrBlur() {
      if (this.modified) {
        this.revert();
      } else {
        this.$refs.translationTextarea.blur();
      }
    },

    revert() {
      // Save cursor position and selection before revert
      let start = this.$refs.translationTextarea.selectionStart;
      let end = this.$refs.translationTextarea.selectionEnd;

      // Actual revert
      this.localContent = this.translation.content;

      // Restore cursor position and selection after revert
      this.$nextTick(() => {
        this.$refs.translationTextarea.selectionStart = start;
        this.$refs.translationTextarea.selectionEnd = end;
      });
    },

    logKey(event) {
      console.log({
        key: event.keyCode,
        meta: event.metaKey,
        ctrl: event.ctrlKey,
      });
    },

    saveIfCmdOrCtrl(event) {
      console.log({ isMac: mac, meta: event.metaKey, ctrl: event.ctrlKey });
      if ((mac && event.metaKey) || (!mac && event.ctrlKey)) {
        event.preventDefault();
        this.save();
      }
    },

    save() {
      console.log("asked for a save");
      // Save cursor position and selection before save
      let start = this.$refs.translationTextarea.selectionStart;
      let end = this.$refs.translationTextarea.selectionEnd;

      $store
        .callApi("/api/v1/translations", "POST", {
          key: this.translationKey,
          locale: this.translation.locale,
          content: this.localContent,
        })
        .then((response) => {
          if (response.ok) {
            return response.json();
          } else {
            window.alert("An error occured while saving the translation.");
          }
        })
        .then((data) => {
          // Apply modification locally for better UI feedback
          this.translation.content = this.localContent;

          // TODO: We should not refresh all locales here
          $store.fetchTranslations();

          // Restore cursor position and selection after revert
          this.$nextTick(() => {
            this.$refs.translationTextarea.selectionStart = start;
            this.$refs.translationTextarea.selectionEnd = end;
          });

          // Show a little confirmation that disappear after 2 seconds
          this.justSaved = true;
          window.setTimeout(() => {
            this.justSaved = false;
          }, 1000);
        });
    },
  },
};
</script>

<style>
.hint-animation-enter-active {
  animation: 0.2s show-hint;
}
.hint-animation-leave-active {
  animation: 0.2s hide-hint;
}
@keyframes show-hint {
  0% {
    transform: translateY(1.2em);
    opacity: 0;
  }
  100% {
    transform: none;
    opacity: 1;
  }
}
@keyframes hide-hint {
  0% {
    transform: none;
    opacity: 1;
  }
  100% {
    transform: translateY(1.2em);
    opacity: 0;
  }
}
</style>
