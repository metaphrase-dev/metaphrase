<template>
  <div
    id="toolbar"
    class="bg-indigo-1000 grid grid-cols-main items-center h-12 text-white"
  >
    <div class="flex flex-row items-center">
      <svg
        class="flex-none mx-3 fill-current overflow-visible"
        xmlns="http://www.w3.org/2000/svg"
        width="33"
        height="26"
      >
        <path
          d="M26 3.2c0-1 .8-2 1.8-2s2 1 2 2-1 2-2 2a2 2 0 01-1.9-2zM27.7 0a3.2 3.2 0 00-3.1 3.2c0 1.8 1.4 3.2 3.1 3.2C29.6 6.4 31 5 31 3.2 31 1.4 29.6 0 27.8 0zM17.4 11.9c.4-.5.9-.7 1.2-.7a51.7 51.7 0 01-1.6 3.4 18 18 0 00-.7 2 8.2 8.2 0 00-.1 3.4c0 .4.2.7.4 1 .2.3.5.6.8.7.3.2.8.3 1.3.3s1-.1 1.4-.4c.5-.2 1-.4 1.3-.8v-.3c0-.2 0-.3-.2-.4-.1 0-.3 0-.5.2l-.7.1c-.2 0-.4 0-.6-.2l-.1-.9c0-.7.1-1.6.5-2.6a47.6 47.6 0 012.5-5.7l.9-2.3c-.1-.5-.4-.8-.8-1a2 2 0 00-1.2-.3c-.6 0-1.2.2-2 .6l-2.1 2a119.2 119.2 0 00-5.5 6.9 65.6 65.6 0 013.1-6.5l.8-1.8-.3-.3a4.1 4.1 0 00-1.4-.7 3 3 0 00-.9 0c-.6 0-1.4.3-2.2 1C9.8 9.1 9 10 8 11a55.7 55.7 0 00-4.6 6.1c.2-.8.4-1.7.8-2.6A28.3 28.3 0 017 9.7c.5-.7.8-1.4 1-2a5.2 5.2 0 00-3-1.8C4.1 7 3.4 8.4 2.6 9.9a32 32 0 00-2.5 7L0 19l.5 1.5c.2.5.6 1 1.1 1.3a74.4 74.4 0 017-9.3 14 14 0 011.7-1.6c.5-.5 1-.7 1.3-.7a29 29 0 00-2.8 5 21.3 21.3 0 00-1.2 4.4l.7 1.4 1.2.8 1.3-1.6a1158.7 1158.7 0 015-6.7c.6-.7 1-1.2 1.6-1.6z"
        />
      </svg>

      <h1 class="flex-none">
        <a href="/">Metaphrase</a>
      </h1>

      <a
        href="#"
        @click.capture="() => {}"
        class="ml-2 p-1 pl-2 flex flex-row items-center bg-indigo-900 rounded text-sm"
      >
        <span>Project name</span>
        <svg
          class="fill-current ml-1"
          width="20"
          height="20"
          viewBox="0 0 20 20"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M5.293 7.29299C5.48053 7.10552 5.73484 7.0002 6 7.0002C6.26516 7.0002 6.51947 7.10552 6.707 7.29299L10 10.586L13.293 7.29299C13.3852 7.19748 13.4956 7.1213 13.6176 7.06889C13.7396 7.01648 13.8708 6.98889 14.0036 6.98774C14.1364 6.98659 14.2681 7.01189 14.391 7.06217C14.5139 7.11245 14.6255 7.1867 14.7194 7.28059C14.8133 7.37449 14.8875 7.48614 14.9378 7.60904C14.9881 7.73193 15.0134 7.86361 15.0123 7.99639C15.0111 8.12917 14.9835 8.26039 14.9311 8.38239C14.8787 8.5044 14.8025 8.61474 14.707 8.70699L10.707 12.707C10.5195 12.8945 10.2652 12.9998 10 12.9998C9.73484 12.9998 9.48053 12.8945 9.293 12.707L5.293 8.70699C5.10553 8.51946 5.00021 8.26515 5.00021 7.99999C5.00021 7.73483 5.10553 7.48052 5.293 7.29299Z"
          />
        </svg>
      </a>
    </div>
    <div class="flex flex-row justify-center">
      <input
        class="bg-indigo-900 h-8 w-full max-w-lg mx-5 rounded-md border border-indigo-700 px-3 placeholder-indigo-400 placeholder-opacity-50 placeholder-italic focus:border-indigo-400 focus:bg-indigo-800 outline-none"
        placeholder="Search everywhere for a key or a translation…"
      />
    </div>
    <div class="flex flex-row justify-end">
      <button
        class="flex-none flex h-12 items-center pl-6 pr-5 bg-indigo-1100 hover:bg-indigo-900 focus:bg-indigo-900 bz-left-15px"
        @click="logout"
      >
        <IconLogout class="mr-2" /> Logout
      </button>
    </div>
  </div>
</template>

<script>
import { IconLogout } from "../assets/Icons.jsx";

export default {
  name: "toolbar",

  components: {
    IconLogout,
  },

  methods: {
    logout() {
      this.$emit("logout");
    },
  },
};
</script>
