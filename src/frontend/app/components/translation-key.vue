<template>
  <div class="translation-key">
    <!-- Spaceless to prevent unexpected spaces in copy-paste -->
    <span v-for="(crumb, index) in crumbs"
      class="translation-crumb"><!--
    --><i class="fa fa-fw fa-caret-right translation-crumb-separator" v-if="index > 0"><span>.</span></i><!--
    --><i class="fa fa-fw fa-folder" style="margin-right: 3px" v-if="index < crumbs.length - 1"></i><!--
    --><i class="fa fa-fw fa-file-o" style="margin-right: 3px" v-if="index == crumbs.length - 1"></i><!--
    -->{{ crumb }}<!--
    --></span>
  </div>
</template>

<script>
  export default ({
    name: 'translation-key',

    props: {
      translationKey: String
    },

    computed: {
      crumbs() {
        return this.translationKey.split('.');
      }
    }
  });
</script>
