<template>
  <div :class="['translation', { edited: modified }]" :t-id="translation.id">
    <div class="translation-locale-label">
      <span class="edited-bullet" v-if="modified">⚫</span>
      <Flag :locale="translation.locale" />
      {{ translation.locale }}
    </div>
    <div class="translation-editor">
      <textarea
        :lang="translation.locale"
        v-model="localContent"
        ref="translationTextarea"
        @keyup.esc="revertOrBlur"
        @keyup.ctrl.enter="save"
        @keyup.meta.enter="save"
      ></textarea>
      <transition name="hint-animation">
        <span class="keyboard-hint" v-if="modified">
          <b>Ctrl-Enter</b> saves your changes. <b>Escape</b> discards them.
        </span>
      </transition>
      <transition name="hint-animation">
        <span class="just-saved" v-if="justSaved">
          <IconCheck /> Saved successfully!
        </span>
      </transition>
    </div>
  </div>
</template>

<script>
import { IconCheck } from "../assets/Icons.jsx";
import Flag from "./Flag.vue";

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
      return (
        String(this.localContent).valueOf() !==
        String(this.translation.content).valueOf()
      );
    },
  },

  components: {
    Flag,
    IconCheck,
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

    save() {
      this.$root.$props.store
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
          this.$root.$props.store.fetchTranslations();

          // Show a little confirmation that disappear after 2 seconds
          this.justSaved = true;
          window.setTimeout(() => {
            this.justSaved = false;
          }, 2000);
        });
    },
  },
};
</script>

<style>
.translation {
  background-color: white;
  display: flex;
  align-items: top;
  border-bottom: 1px solid #e0e0e0;
}

.translation.edited .translation-locale-label {
  font-weight: bold;
  color: black;
}

.translation.edited textarea {
  color: #111;
}

.translation:nth-child(2n),
.translation:nth-child(2n) textarea {
  background-color: #f2f6fb;
}

.translation-locale-label {
  width: 70px;
  text-align: right;
  padding-right: 10px;
  vertical-align: center;
  font-size: 9pt;
  font-weight: normal;
  line-height: 1.8;
  margin-top: 5px;
  font-family: monospace;
  color: #545454;
}

.translation .edited-bullet {
  color: #0085c7;
  font-family: sans-serif;
  font-size: 7pt;
}

.translation-editor textarea {
  flex: 1 0 auto;
  border: 0 none;
  padding: 6px 5px;
  min-height: 55px;
  margin: 0;
  font-family: inherit;
  font-size: 10.5pt;
  color: #555;
  outline: 0 none;
}

.translation:last-of-type {
  border-bottom: 0 none;
}

.translation-editor {
  flex: 1 0 100px;
  display: flex;
  flex-direction: column;
  border-style: solid;
  border-width: 0 0 0 1px;
  border-color: #e0e0e0;
}

.keyboard-hint,
.just-saved {
  font-size: 8pt;
  padding: 4px 5px;
  animation: 0.2s show-hint;
  line-height: 1.5;
}

.keyboard-hint {
  color: #0085c7;
}

.just-saved {
  color: #37a10e;
}

.hint-animation-leave-active {
  animation: 0.2s hide-hint;
}

@keyframes show-hint {
  0% {
    transform: translateY(1.2em) scaleY(0);
    opacity: 0;
    height: 0;
    padding: 0 5px;
  }

  100% {
    transform: none;
    opacity: 1;
    padding: 4px 5px;
    height: 25px;
  }
}

@keyframes hide-hint {
  100% {
    transform: translateY(-1.2em) scaleY(0);
    opacity: 0;
    height: 0;
    padding: 0 5px;
  }

  0% {
    transform: none;
    opacity: 1;
    padding: 4px 5px;
    height: 25px;
  }
}
</style>
