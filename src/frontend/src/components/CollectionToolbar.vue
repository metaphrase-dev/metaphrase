<template>
  <div
    class="collection-toolbar h-12 flex items-center pl-8 flex-row bg-indigo-700"
  >
    <Button @click="showAddNewKey">
      <template v-slot:icon>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-5 w-5"
          viewBox="0 0 20 20"
          fill="currentColor"
        >
          <path
            d="M10 5a1 1 0 011 1v3h3a1 1 0 110 2h-3v3a1 1 0 11-2 0v-3H6a1 1 0 110-2h3V6a1 1 0 011-1z"
          />
        </svg>
      </template>

      New key
    </Button>
  </div>
</template>

<script>
import { IconPlus } from "../assets/Icons.jsx";
import Button from "./ui/Button.vue";

export default {
  name: "collection-toolbar",

  components: {
    IconPlus,
    Button,
  },

  methods: {
    showAddNewKey() {
      this.$emit("showAddNewKey");
    },
  },
};
</script>

<style>
/*
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
*/
</style>
