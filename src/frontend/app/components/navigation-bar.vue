<template>
  <div id="navigation-bar">
    <div class="search-bar">
      <input type="search" placeholder="Search for a key…" v-model="needle">
      <i class="fa fa-fw fa-search" />
    </div>
    <ul class="nav-translation-key-list">
      <li v-for="key in filteredTranslationsKeys" class="nav-translation-key">
        <a href="#" @click.prevent>
          <i class="fa fa-fw fa-file-o"></i>
          <span>{{ key }}</span>
          <i class="fa fa-fw fa-chevron-right"></i>
        </a>
      </li>
    </ul>
  </div>
</template>

<script>
  import _ from "lodash";

  export default ({
    name: 'navigation-bar',

    data() {
      return {
        needle: ""
      };
    },

    props: {
      translationKeys: Array
    },

    computed: {
      filteredTranslationsKeys() {
        return _.filter(this.translationKeys, (key) => {
          return key.includes(this.$data.needle);
        });
      }
    }
  });
</script>
