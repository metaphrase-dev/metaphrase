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
