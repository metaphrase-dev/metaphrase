<template>
  <div class="collection-toolbar">
    <button @click="showAddNewKey">
      <i class="fa fa-fw fa-plus"></i> New key
    </button>
  </div>
</template>

<script>
  export default {
    name: 'collection-toolbar',

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
