<template>
  <div class="translation-group bg-gray-100 odd:bg-indigo-100 mb-1px">
    <TranslationKey :translation-key="translationKey" />
    <TranslationLocale
      v-for="translation in translations"
      :key="translation.locale"
      :translation="translation"
      :translation-key="translationKey"
    />
  </div>
</template>

<script>
import TranslationKey from "../translations/TranslationKey.vue";
import TranslationLocale from "../translations/TranslationLocale.vue";

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
