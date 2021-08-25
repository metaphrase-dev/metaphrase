<template>
  <div id="navigation-panel" class="w-72 border-r border-indigo-800 bg-goose-100">
    <LocationBar :translationKeys="translationKeys" />
    <ul class="nav-translation-key-list">
      <NavigationKey
        v-for="{ key, count } in filteredTranslationsKeys"
        :key="key"
        :canonicalKey="key"
        :namespace="namespace"
        :forceCanonical="false"
        :count="count"
      />
    </ul>
  </div>
</template>

<script>
import NavigationKey from "./NavigationKey.vue";
import LocationBar from "./LocationBar.vue";

export default {
  props: {
    translationKeys: Array,
    namespace: String,
  },

  computed: {
    filteredTranslationsKeys() {
      return this.currentNamespaceTranslationKeys
        .sort((a, b) => {
          const aFolder = a.endsWith("..");
          const bFolder = b.endsWith("..");
          const eitherFolder = aFolder || bFolder;

          if ((aFolder && bFolder) || !eitherFolder) {
            return a.localeCompare(b);
          } else {
            return !aFolder;
          }
        })
        .map((key) => {
          const folder = key.slice(0, key.length - 2);
          return {
            key,
            count: key.endsWith("..")
              ? this.translationKeys.filter((tk) => tk.startsWith(folder)).length
              : 0,
          };
        });
    },

    currentNamespaceTranslationKeys() {
      let currentKeys = this.translationKeys.filter((key) => key.startsWith(this.namespace || ""));

      let truncated = currentKeys.map((key) => {
        let namespaceDepth = 0;
        if (this.namespace.length > 0) {
          namespaceDepth = (this.namespace.match(/\./g) || []).length + 1;
        }
        let keyWithoutNamespace = key.split(".").slice(namespaceDepth).join(".");
        let crumbs = keyWithoutNamespace.split(".");
        let initialDot = this.namespace.length > 0 ? "." : "";

        if (crumbs.length > 1) {
          return this.namespace + initialDot + crumbs[0] + "..";
        } else {
          return this.namespace + initialDot + keyWithoutNamespace;
        }
      });

      return [...new Set(truncated)].sort();
    },
  },

  methods: {
    setNamespace(namespace) {
      this.$emit("namespaceChanged", namespace);
    },
    goToParentNamespace(namespace) {
      this.$emit("namespaceChanged", namespace.split(".").slice(0, -1).join("."));
    },
  },

  components: {
    NavigationKey,
    LocationBar,
  },
};
</script>
