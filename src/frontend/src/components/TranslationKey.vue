<template>
  <div class="translation-key">
    <!-- Spaceless to prevent unexpected spaces in copy-paste -->
    <span
      v-for="(crumb, index) in crumbs"
      class="translation-crumb"
      :key="index"
      ><!--
    --><IconChevronRight
        class="translation-crumb-separator"
        v-if="index > 0"
      /><span>.</span
      ><!--
    --><IconFolder
        style="margin-right: 3px"
        v-if="index < crumbs.length - 1"
      /><!--
    --><IconFile
        style="margin-right: 3px"
        v-if="index == crumbs.length - 1"
      /><!--
    -->{{ crumb
      }}<!--
    --></span>
    <button class="translation-key-delete" @click="deleteTranslation">
      <IconTrash />
    </button>
  </div>
</template>

<script>
import {
  IconFile,
  IconFolder,
  IconChevronRight,
  IconTrash,
} from "../assets/Icons.jsx";

export default {
  name: "translation-key",

  props: {
    translationKey: String,
  },

  components: {
    IconChevronRight,
    IconFile,
    IconFolder,
    IconTrash,
  },

  computed: {
    crumbs() {
      return this.translationKey.split(".");
    },
  },

  methods: {
    deleteTranslation() {
      let confirmation = window.confirm(
        `You will destroy ${this.translationKey}; are you sure?`
      );

      if (confirmation) {
        this.$$root.$props.store
          .callApi(`/api/v1/translations/${this.translationKey}`, "DELETE")
          .then((response) => {
            if (response.ok) {
              return response.json();
            } else {
              window.alert("An error occured while deleting the translation.");
            }
          })
          .then((data) => {
            this.$root.$props.store.fetchTranslations();
          });
      }
    },
  },
};
</script>
