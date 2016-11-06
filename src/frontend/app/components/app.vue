<template>
  <div id="main">
    <toolbar />
    <div id="workspace">
      <navigation-bar
        :translation-keys="translationKeys"
        :namespace="relevantNamespace"
        @namespaceChanged="updateNamespace" />
      <div id="translation-list">
        <translation-group
          v-for="key in filteredTranslationKeys"
          :translation-key="key"
          :translations="store.groupedTranslations[key]" />
      </div>
    </div>
  </div>
</template>

<script>
  import Toolbar from "./toolbar.vue";
  import TranslationGroup from "./translation-group.vue";
  import NavigationBar from "./navigation-bar.vue";
  import _ from "lodash";

  export default ({
    name: 'app',

    props: {
      store: Object
    },

    computed: {
      translationKeys() {
        return _.keys(this.store.groupedTranslations);
      },

      filteredTranslationKeys() {
        // FIXME: Implement paging instead of limiting to 50st first keys

        return _.filter(this.translationKeys, (key) => {
          return _.startsWith(key, this.store.namespace);
        }).sort().slice(0, 50);
      },

      relevantNamespace() {
        let keysCount = this.filteredTranslationKeys.length;
        let namespace = this.store.namespace;

        if (keysCount === 1 && this.filteredTranslationKeys[0] == namespace) {
          if (namespace.includes('.')) {
            return namespace.split('.').slice(0, -1).join('.');
          } else {
            return '';
          }
        } else {
          return namespace;
        }
      }
    },

    components: {
      toolbar: Toolbar,
      translationGroup: TranslationGroup,
      navigationBar: NavigationBar
    },

    methods: {
      updateNamespace(namespace) {
        window.location.hash = namespace;
      }
    }
  });
</script>
