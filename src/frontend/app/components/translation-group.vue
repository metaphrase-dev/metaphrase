<template>
  <div class="translation-group">
    <translation-key :translation-key="translationKey" />
    <translation-locale v-for="translation in translations"
      :translation="translation"
      :translation-key="translationKey" />
  </div>
</template>

<script>
  import TranslationKey from "./translation-key.vue";
  import TranslationLocale from "./translation-locale.vue";

  export default ({
    name: 'translation-group',

    props: {
      translations: Array,
      translationKey: String
    },

    components: {
      translationKey: TranslationKey,
      translationLocale: TranslationLocale
    }
  });
</script>
