<template>
  <div id="main">
    <toolbar />
    <div id="workspace">
      <navigation-bar :translation-keys="translationKeys" />
      <div id="translation-list">
        <translation-group
          v-for="(translations, key) in store.groupedTranslations"
          :translation-key="key"
          :translations="translations" />
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
      }
    },

    components: {
      toolbar: Toolbar,
      translationGroup: TranslationGroup,
      navigationBar: NavigationBar
    }
  });
</script>
