<template>
  <i :class="'flag-icon flag-icon-' + countryFlag" />
</template>

<script>
  export default ({
    name: 'flag',

    props: {
      locale: String
    },

    computed: {
      countryFlag() {
        switch (this.locale) {
          case 'en':
            return 'gb';
          default:
            return this.locale;
        }
      }
    }
  });
</script>
