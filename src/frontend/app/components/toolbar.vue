<template>
  <div id="toolbar">
    <h1>Lugh</h1>
    <button @click="logout"><i class="fa fa-fw fa-sign-out"></i> Logout</button>
  </div>
</template>

<script>
  export default ({
    name: 'toolbar',

    methods: {
      logout() {
        this.$emit('logout');
      }
    }
  });
</script>
