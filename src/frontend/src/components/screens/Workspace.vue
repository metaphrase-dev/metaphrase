<template>
  <Toolbar />
  <div id="workspace" class="flex w-full items-stretch">
    <NavigationPanel
      :translation-keys="translationKeys"
      :namespace="relevantNamespace"
      @namespaceChanged="updateNamespace"
    />
    <div id="translation-list" class="flex-grow">
      <CollectionToolbar />
      <TranslationGroup
        v-for="group in translationGroups"
        :key="group.key"
        :translation-key="group.key"
        :translations="group.translations"
      />
    </div>
  </div>
</template>

<script>
import NavigationPanel from "../navigation/NavigationPanel.vue";
import Toolbar from "../navigation/Toolbar.vue";
import TranslationGroup from "../translations/TranslationGroup.vue";
import CollectionToolbar from "../translations/CollectionToolbar.vue";
import Store from "../../store.js";

const $store = new Store();

export default {
  components: {
    NavigationPanel,
    Toolbar,
    TranslationGroup,
    CollectionToolbar,
  },

  created() {
    $store.fetchTranslations();
  },

  computed: {
    allTranslations() {
      return $store.groupedTranslations;
    },

    translationKeys() {
      return Object.keys($store.groupedTranslations ?? {});
    },

    translationGroups() {
      return this.filteredTranslationKeys
        .map((key) => ({
          key,
          translations: $store.groupedTranslations[key],
        }))
        .sort((a, b) => {
          const level = this.relevantNamespace.split(".").length + 1;
          const aHere = a.key.split(".").length === level;
          const bHere = b.key.split(".").length === level;

          if ((aHere && bHere) || (!aHere && !bHere)) {
            return a.key.localeCompare(b.key);
          } else {
            return !aHere;
          }
        });
    },

    filteredTranslationKeys() {
      // FIXME: Implement paging instead of limiting to 50st first keys

      return this.translationKeys
        .filter((key) => key.startsWith(this.$route.params.namespace ?? ""))
        .sort()
        .slice(0, 50);
    },

    relevantNamespace() {
      let keysCount = this.filteredTranslationKeys.length;
      let namespace = this.$route.params.namespace ?? "";

      if (keysCount === 1 && this.filteredTranslationKeys[0] === namespace) {
        if (namespace.includes(".")) {
          return namespace.split(".").slice(0, -1).join(".");
        } else {
          return "";
        }
      } else {
        return namespace;
      }
    },
  },

  methods: {
    updateNamespace(namespace) {
      if (namespace?.length) {
        window.location.hash = namespace;
      }
    },
  },
};
</script>
