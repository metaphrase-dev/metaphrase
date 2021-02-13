<template>
  <div id="toolbar">
    <h1>Metaphrase</h1>
    <button @click="logout"><IconLogoutVariant /> Logout</button>
  </div>
</template>

<script>
import IconLogoutVariant from "/@vite-icons/mdi/logout-variant.vue";

export default {
  name: "toolbar",

  components: {
    IconLogoutVariant,
  },

  methods: {
    logout() {
      this.$emit("logout");
    },
  },
};
</script>
<style>
#toolbar {
  background-color: #3f4757;
  color: white;
  border-bottom: 1px solid black;
  display: flex;
  justify-content: space-between;
}

#toolbar h1 {
  font-size: 12pt;
  margin-left: 10px;
}

#toolbar button {
  border: 1px solid black;
  background-color: rgba(255, 255, 255, 0.2);
  color: white;
  font-size: 10pt;
  font-family: inherit;
  align-self: center;
  margin-right: 10px;
  line-height: 1.8em;
  border-radius: 3px;
  box-shadow: inset -2px -2px 1px rgba(0, 0, 0, 0.2),
    inset 1px 1px rgba(255, 255, 255, 0.1);
}

#toolbar button:active {
  box-shadow: inset 2px 2px 1px rgba(0, 0, 0, 0.2),
    inset -1px -1px rgba(255, 255, 255, 0.1);
}

#toolbar button:focus {
  outline: 1px dashed rgba(255, 255, 255, 0.2);
}

#toolbar button::-moz-focus-inner {
  border: 0;
}
</style>
