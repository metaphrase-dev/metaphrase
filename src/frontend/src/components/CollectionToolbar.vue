<template>
  <div class="collection-toolbar">
    <button @click="showAddNewKey"><IconPlus /> New key</button>
  </div>
</template>

<script>
import { IconPlus } from "../assets/Icons.jsx";

export default {
  name: "collection-toolbar",

  components: {
    IconPlus,
  },

  methods: {
    showAddNewKey() {
      this.$emit("showAddNewKey");
    },
  },
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
  display: inline-flex;
  align-items: center;
}

.collection-toolbar svg {
  margin-right: 5px;
}
</style>
