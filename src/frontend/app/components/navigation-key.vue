<template>
  <li class="nav-translation-key">
    <a href="#" @click.prevent="setNamespace(canonicalKey)">
      <i v-if="!isFolder" class="fa fa-fw fa-file-o"></i>
      <i v-if="isFolder" class="fa fa-fw fa-folder"></i>
      <span>{{ label }}</span>
      <i class="fa fa-fw fa-chevron-right"></i>
    </a>
  </li>
</template>

<script>
  import _ from "lodash";

  export default ({
    name: 'navigation-key',

    props: {
      canonicalKey: String,
      namespace: String,
      forceCanonical: Boolean
    },

    computed: {
      label() {
        if (this.forceCanonical) {
          return this.canonicalKey;
        } else {
          let key = this.canonicalKey;

          // Remove .. at the end of folders key
          if (this.isFolder) {
            key = key.slice(0, -2);
          }

          // Remove namespace from canonical (including dot)
          if (this.namespace.length > 0) {
            key = key.substring(this.namespace.length + 1);
          }

          return key;
        }
      },

      isFolder() {
        return this.canonicalKey.slice(-2) == '..';
      }
    },

    methods: {
      setNamespace(namespace) {
        this.$emit('namespaceChanged', namespace.replace(/\.\.$/, ''));
      }
    }
  });
</script>
