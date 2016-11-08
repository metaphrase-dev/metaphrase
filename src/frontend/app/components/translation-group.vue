<template>
  <div class="translation-group">
    <translation-key :translation-key="translationKey" />
    <div class="translation" v-for="translation in translations"
      :t-id="translation.id">
      <div class="translation-locale">
        <flag :locale="translation.locale" />
        {{ translation.locale }}
      </div>
      <textarea>{{ translation.content }}</textarea>
    </div>
  </div>
</template>

<script>
  import TranslationKey from "./translation-key.vue";
  import Flag from "./flag.vue";

  export default ({
    name: 'translation-group',

    props: {
      translations: Object,
      translationKey: String
    },

    components: {
      translationKey: TranslationKey,
      flag: Flag
    }
  });
</script>
