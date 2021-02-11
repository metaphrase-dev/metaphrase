<template>
  <div class="collection-toolbar">
    <button @click="showAddNewKey">
      <IconPlus /> New key
    </button>
  </div>
</template>

<script>
  import IconPlus from '/@vite-icons/mdi/plus.vue'

  export default {
    name: 'collection-toolbar',

    components: {
      IconPlus
    },

    methods: {
      showAddNewKey() {
        this.$emit('showAddNewKey');
      }
    }
  };
</script>

<style>
  .collection-toolbar {
    display: flex;
    background-color: lightgrey;
    border-bottom: 1px solid grey;
    height: 39px;
    align-items: center;
    padding-left: 6px;
  }

  .collection-toolbar button {
    border: 1px solid grey;
    border-radius: 3px;
    font-family: inherit;
    font-size: 14px;
    padding: 4px;
  }
</style>
