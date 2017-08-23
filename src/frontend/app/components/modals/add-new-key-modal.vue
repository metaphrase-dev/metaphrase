<template>
  <div class="modal">
    <a href="#close-modal" class="close-modal" @click.prevent="closeModal">
      <i class="fa fa-fw fa-times"></i>
    </a>

    <h2><i class="fa fa-fw fa-plus"></i> Add a new key</h2>

    <form @submit.prevent="submit">
      <label for="new-key-input" class="label-standalone">
        Please provide the new <strong>key path and name</strong>, joined by a dot.
      </label>

      <i class="fa fa-fw fa-key icon-prepend"></i>
      <input id="new-key-input" ref="newKeyInput"
             v-model="newKey"
             class="input-form input-standalone" type="text" />

      <footer>
        <button class="primary">
          Save
        </button>
      </footer>
    </form>
  </div>
</template>

<script>
  export default {
    name: 'add-new-key-modal',

    data() {
      return {
        newKey: this.currentNamespace.length > 0 ? `${this.currentNamespace}.` : ''
      };
    },

    props: {
      currentNamespace: String,
      store: Object,
    },

    created() {
      // Focus key input on component creation
      this.$nextTick(() => {
        this.$refs.newKeyInput.focus()
        this.$refs.newKeyInput.selectionStart =
          this.$refs.newKeyInput.selectionEnd =
          this.$refs.newKeyInput.value.length;
      });
    },

    methods: {
      closeModal() {
        this.$emit('closeModal');
      },

      submit() {
        this.$emit('submitModal', 'add-new-key', {
          newKey: this.newKey
        });
      }
    }
  };
</script>
