<template>
  <div class="translation-group">
    <TranslationKey :translation-key="translationKey" />
    <TranslationLocale
      v-for="translation in translations"
      :key="translation"
      :translation="translation"
      :translation-key="translationKey"
    />
  </div>
</template>

<script>
import TranslationKey from "./TranslationKey.vue";
import TranslationLocale from "./TranslationLocale.vue";

export default {
  name: "translation-group",

  props: {
    translations: Array,
    translationKey: String,
  },

  components: {
    TranslationKey,
    TranslationLocale,
  },
};
</script>
