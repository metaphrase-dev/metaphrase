<template>
  <div class="modal" @keyup.escape="closeModal">
    <a href="#close-modal" class="close-modal" @click.prevent="closeModal">
      <IconClose />
    </a>

    <h2><IconPlus /> Add a new key</h2>

    <form @submit.prevent="submit">
      <label for="new-key-input" class="label-standalone">
        Please provide the new <strong>key path and name</strong>, joined by a
        dot.
      </label>

      <IconKey class="icon-prepend" />
      <input
        id="new-key-input"
        ref="newKeyInput"
        v-model="newKey"
        class="input-form input-standalone"
        type="text"
      />

      <footer>
        <button class="primary">Save</button>
      </footer>
    </form>
  </div>
</template>

<script>
import IconClose from "/@vite-icons/mdi/close.vue";
import IconPlus from "/@vite-icons/mdi/plus.vue";
import IconKey from "/@vite-icons/mdi/key.vue";

export default {
  name: "add-new-key-modal",

  components: {
    IconClose,
    IconPlus,
    IconKey,
  },

  data() {
    return {
      newKey:
        this.currentNamespace.length > 0 ? `${this.currentNamespace}.` : "",
    };
  },

  props: {
    currentNamespace: String,
    store: Object,
  },

  created() {
    // Focus key input on component creation
    this.$nextTick(() => {
      this.$refs.newKeyInput.focus();
      this.$refs.newKeyInput.selectionStart = this.$refs.newKeyInput.selectionEnd = this.$refs.newKeyInput.value.length;
    });
  },

  methods: {
    closeModal() {
      this.$emit("closeModal");
    },

    submit() {
      this.$emit("submitModal", "add-new-key", {
        newKey: this.newKey,
      });
    },
  },
};
</script>
