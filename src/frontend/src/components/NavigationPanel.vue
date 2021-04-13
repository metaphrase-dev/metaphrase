<template>
  <div
    id="navigation-panel"
    class="w-72 border-r border-indigo-800 bg-goose-100"
  >
    <div
      class="location-bar flex flex-row bg-indigo-800 items-center pl-4 pr-4 h-12 bz-right-15px mr-negative-15px"
    >
      <input
        class="flex-grow flex-shrink min-w-0 w-auto"
        type="search"
        placeholder="Search for a key…"
        v-model="needle"
      />
      <IconSearch class="flex-none ml-2" />
    </div>
    <ul class="nav-translation-key-list">
      <li
        v-if="needle.length == 0 && namespace.length > 0"
        class="nav-translation-key nav-back"
      >
        <a href="#" @click.prevent="goToParentNamespace(namespace)">
          <IconChevronRight />
          <span
            ><b>{{ namespace }}</b></span
          >
        </a>
      </li>
      <template v-for="key in filteredTranslationsKeys" :key="key">
        <NavigationKey
          :canonicalKey="key"
          :namespace="namespace"
          :forceCanonical="needle.length > 0"
          @namespaceChanged="setNamespace(...arguments)"
        />
      </template>
    </ul>
  </div>
</template>

<script>
import { IconChevronRight, IconSearch } from "../assets/Icons.jsx";

import NavigationKey from "./NavigationKey.vue";
import _ from "lodash";

export default {
  name: "navigation-bar",

  data() {
    return {
      needle: "",
    };
  },

  props: {
    translationKeys: Array,
    namespace: String,
  },

  computed: {
    filteredTranslationsKeys() {
      if (this.$data.needle !== "") {
        return this.foundTranslationKeys;
      } else {
        return this.currentNamespaceTranslationKeys;
      }
    },

    currentNamespaceTranslationKeys() {
      let currentKeys = _.filter(this.translationKeys, (key) => {
        return key.startsWith(this.namespace || "");
      });

      let truncated = _.map(currentKeys, (key) => {
        let namespaceDepth = 0;
        if (this.namespace.length > 0) {
          namespaceDepth = (this.namespace.match(/\./g) || []).length + 1;
        }
        let keyWithoutNamespace = key
          .split(".")
          .slice(namespaceDepth)
          .join(".");
        let crumbs = keyWithoutNamespace.split(".");
        let initialDot = this.namespace.length > 0 ? "." : "";

        if (crumbs.length > 1) {
          return this.namespace + initialDot + crumbs[0] + "..";
        } else {
          return this.namespace + initialDot + keyWithoutNamespace;
        }
      });

      return _.uniq(truncated).sort();
    },

    foundTranslationKeys() {
      return _.filter(this.translationKeys, (key) => {
        return key.includes(this.$data.needle);
      }).sort();
    },
  },

  methods: {
    setNamespace(namespace) {
      this.$emit("namespaceChanged", namespace);
    },
    goToParentNamespace(namespace) {
      this.$emit(
        "namespaceChanged",
        namespace.split(".").slice(0, -1).join(".")
      );
    },
  },

  components: {
    NavigationKey,
    IconChevronRight,
    IconSearch,
  },
};
</script>
