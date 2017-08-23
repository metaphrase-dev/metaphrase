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
    <button class="translation-key-delete" @click="deleteTranslation">
      <i class="fa fa-fw fa-trash-o" />
    </button>
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
    },

    methods: {
      deleteTranslation() {
        let confirmation = window.confirm(`You will destroy ${this.translationKey}; are you sure?`);

        if (confirmation) {
          this.$root.$data.callApi(`/api/v1/translations/${this.translationKey}`, 'DELETE').then(response => {
            if (response.ok) {
              return response.json();
            } else {
              window.alert("An error occured while deleting the translation.");
            }
          }).then(data => {
            this.$root.$data.fetchTranslations();
          });
        }
      }
    }
  });
</script>

<style>
  .translation-key {
    padding: 10px;
    font-weight: bold;
    font-size: 10pt;
    border-bottom: 1px solid grey;
    background-color: #ececec;
  }

  .translation-crumb {
    display: inline-block;
  }

  .translation-crumb-separator span {
    font-size: 0;
  }

  .translation-key-delete {
    float: right;
    border: 1px solid currentColor;
    background: transparent;
    padding: 2px;
    margin-top: -2px;
    margin-bottom: -2px;
    vertical-align: bottom;
    border-radius: 2px;

    color: #850101;
  }

  .translation-key-delete:focus,
  .translation-key-delete:active {
    box-shadow: 0 0 0 1px #850101 inset;
  }
</style>
