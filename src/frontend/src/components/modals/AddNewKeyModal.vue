<template>
  <div
    class="fixed z-10 inset-0 overflow-y-auto"
    aria-labelledby="modal-title"
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    @keyup.escape="closeModal"
  >
    <div
      class="flex items-center justify-center min-h-screen pt-4 px-4 pb-20 text-center"
    >
      <div
        class="fixed inset-0 bg-gray-900 bg-opacity-75 transition-opacity"
        aria-hidden="true"
      ></div>
      <div
        class="block bg-white rounded-lg text-left shadow-xl transform transition-all max-w-lg w-full"
      >
        <div class="px-4 pt-4 pb-4 flex">
          <div
            class="mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 rounded-full bg-indigo-100 sm:mx-0 sm:h-10 sm:w-10"
          >
            <IconPlus />
          </div>
          <div class="mt-2 pl-5 justify-start">
            <h3
              class="text-lg leading-6 font-medium text-gray-900"
              id="modal-title"
            >
              Add a new key
            </h3>
            <div class="mt-2">
              <label for="new-key-input" class="text-sm text-gray-500">
                Please provide the new <strong>key path and name</strong>,
                joined by a dot.
              </label>
            </div>
          </div>
        </div>
        <form @submit.prevent="submit" class="ml-5">
          <div class="flex ml-16 mr-4 items-center">
            <IconKey
              class="text-indigo-600 flex-none -mr-8 position-relative z-10"
            />
            <input
              autofocus
              id="new-key-input"
              ref="newKeyInput"
              v-model="newKey"
              class="flex-grow pl-10 min-w-0 w-full p-2 border border-indigo-300 rounded-md focus:border-indigo-700 outline-none"
              type="text"
            />
          </div>

          <footer>
            <div
              class="bg-gray-50 px-4 py-3 flex flex-row-reverse rounded-b-lg"
            >
              <button
                type="submit"
                class="inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-indigo-600 text-base font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 ml-3"
              >
                Save
              </button>
              <button
                type="button"
                @click.prevent="closeModal"
                class="inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
              >
                Cancel
              </button>
            </div>
          </footer>
        </form>
      </div>
    </div>
  </div>
</template>

<script>
import { IconClose, IconPlus, IconKey } from "../../assets/Icons.jsx";

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
