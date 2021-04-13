<template>
  <div :class="['translation', { edited: modified }]" :t-id="translation.id">
    <div class="translation-locale-label">
      <span class="edited-bullet" v-if="modified">⚫</span>
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
