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

<style>
  .flag-icon {
    box-shadow: inset 0 0 0 1px rgba(0, 0, 0, 0.1);
    border-radius: 2px;
    vertical-align: 0;
    opacity: 0.75;
  }
</style>
