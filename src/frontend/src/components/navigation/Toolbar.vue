<template>
  <div id="toolbar" class="bg-indigo-1000 grid grid-cols-main items-center h-12 text-white">
    <div class="flex flex-row items-center">
      <MetaphraseLogo class="flex-none mx-3 fill-current overflow-visible" />

      <h1 class="flex-none">
        <a href="/">Metaphrase</a>
      </h1>

      <!--
      <button
        class="ml-2 p-1 pl-2 flex flex-row items-center bg-indigo-900 rounded text-sm w-max max-w-xs"
      >
        <span>Project name</span>
        <IconDownChevron class="fill-current ml-1" />
      </button>
      -->
    </div>
    <div class="flex flex-row justify-center">
      <!--
      <input
        class="bg-indigo-900 h-8 w-full max-w-lg mx-5 rounded-md border border-indigo-700 px-3 placeholder-indigo-400 placeholder-opacity-50 placeholder-italic focus:border-indigo-400 focus:bg-indigo-800 outline-none"
        placeholder="Search everywhere for a key or a translation…"
      />
      -->
    </div>
    <div class="flex flex-row justify-end">
      <button
        class="flex-none flex h-12 items-center pl-6 pr-5 bg-indigo-1100 hover:bg-indigo-900 focus:bg-indigo-900 bz-left-15px"
        @click="logout"
      >
        <IconLogout class="mr-2" />Logout
      </button>
    </div>
  </div>
</template>

<script>
import { IconLogout } from "../../assets/Icons24.jsx";
import { IconDownChevron } from "../../assets/Icons20.jsx";
import { MetaphraseLogo } from "../../assets/Logo.jsx";
import Store from "../../store.js";

const $store = new Store();

export default {
  name: "toolbar",

  components: {
    IconLogout,
    IconDownChevron,
    MetaphraseLogo,
  },

  methods: {
    logout() {
      $store
        .callApi("/api/v1/logout", "POST")
        .then(() => $store.resetToken())
        .then(() => this.$router.push({ name: "Login" }));
    },
  },
};
</script>
