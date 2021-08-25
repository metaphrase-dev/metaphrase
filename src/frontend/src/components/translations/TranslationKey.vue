<template>
  <div class="translation-key h-8 text-sm text-gray-600 flex items-stretch">
    <span class="w-16" />
    <router-link
      v-for="(namespace, index) in namespaces"
      class="translation-crumb flex items-center hover:bg-indigo-200"
      :key="index"
      :to="{
        name: 'Translation',
        params: { namespace: namespaces.slice(0, Number(index) + 1).join('.') },
      }"
    >
      <span class="text-0" v-if="Number(index) > 0">.</span>
      <span>{{ namespace }}</span>
    </router-link>
    <span class="translation-crumb flex items-center flex-grow">
      <span class="text-0" v-if="translationKey.split('.').length > 1">.</span>
      <span>{{ keyName }}</span>
    </span>
    <button class="px-2 hover:text-indigo-900" @click="deleteTranslation" tabindex="-1">
      <IconTrash />
    </button>
  </div>
</template>

<script type="ts">
import Store from "../../store";
import { IconTrash } from "../../assets/Icons20.jsx";

const $store = new Store();

export default {
  name: "translation-key",

  props: {
    translationKey: String,
  },

  components: {
    IconTrash
  },

  computed: {
    namespaces() {
      const crumbs = this.translationKey.split(".");
      crumbs.pop();
      return crumbs;
    },
    keyName() {
      return this.translationKey.split(".").pop();
    },
  },

  methods: {
    deleteTranslation() {
      let confirmation = window.confirm(
        `You will destroy ${this.translationKey}; are you sure?`
      );

      if (confirmation) {
        $store
          .callApi(`/api/v1/translations/${this.translationKey}`, "DELETE")
          .then((response) => {
            if (response.ok) {
              return response.json();
            } else {
              window.alert("An error occured while deleting the translation.");
            }
          })
          .then((data) => {
            $store.fetchTranslations();
          });
      }
    },
  },
};
</script>

<style>
.translation-crumb {
  clip-path: polygon(0 0, calc(100% - 16px) 0, 100% 50%, calc(100% - 16px) 100%, 0 100%);
  padding-left: 12px;
  padding-right: 24px;
}

.translation-crumb + .translation-crumb,
.translation-name {
  margin-left: -16px;
  background-image: linear-gradient(to right, white, transparent);
  background-size: 50px 100%;
  background-repeat: no-repeat;
  padding-left: 28px;
  clip-path: polygon(0 0, calc(100% - 16px) 0, 100% 50%, calc(100% - 16px) 100%, 0 100%, 16px 50%);
}

a.translation-crumb:hover {
  background-image: none;
}
</style>
