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
import IconFile from "/@vite-icons/mdi/file.vue";
import IconFolder from "/@vite-icons/mdi/folder.vue";
import IconChevronRight from "/@vite-icons/mdi/chevron-right.vue";
import IconTrash from "/@vite-icons/mdi/trash.vue";

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

<style>
.translation-key {
  padding: 10px;
  font-weight: bold;
  font-size: 10pt;
  border-bottom: 1px solid grey;
  background-color: #ececec;
}

.translation-crumb {
  display: inline-block;
}

.translation-crumb-separator + span {
  font-size: 0;
}

.translation-key-delete {
  float: right;
  border: 1px solid currentColor;
  background: transparent;
  padding: 2px;
  margin-top: -2px;
  margin-bottom: -2px;
  vertical-align: bottom;
  border-radius: 2px;

  color: #850101;
}

.translation-key-delete:focus,
.translation-key-delete:active {
  box-shadow: 0 0 0 1px #850101 inset;
}
</style>
