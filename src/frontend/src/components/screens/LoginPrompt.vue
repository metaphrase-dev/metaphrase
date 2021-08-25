<template>
  <div class="min-h-screen flex items-center justify-center bg-goose-50">
    <div class="login-dialog w-full max-w-md m-auto">
      <img class="mx-auto w-auto mb-6" src="../../assets/login-logo.svg" alt />
      <h1 class="text-3xl text-center mb-4">Metaphrase</h1>
      <h2 class="text-xl text-center mb-8">Please login to continue.</h2>
      <form action="./" method="POST" @submit.prevent="logIn">
        <label class="flex flex-row items-center">
          <div
            class="position-relative z-20 flex-none w-28 -mr-28 text-sm font-medium text-goose-500 text-right pr-2"
          >
            Email:
          </div>
          <input
            class="position-relative pl-28 focus:z-10 flex-grow min-w-0 w-full py-2 px-3 border border-goose-300 placeholder-gray-700 placeholder-opacity-40 focus:border-indigo-500 focus:border-w-2 outline-none rounded-t-md"
            name="email"
            placeholder="e.g. jean.doe@acme-international.com"
            v-model="loginEmail"
          />
        </label>
        <label class="flex flex-row items-center -mt-0.5">
          <span
            class="position-relative z-20 flex-none w-28 -mr-28 text-sm font-medium text-goose-500 text-right pr-2"
            >Password:</span
          >
          <input
            class="position-relative pl-28 focus:z-10 flex-grow min-w-0 w-full py-2 px-3 border border-goose-300 placeholder-gray-700 placeholder-opacity-40 focus:border-indigo-500 focus:border-w-2 outline-none rounded-b-md shadow-sm"
            name="password"
            type="password"
            placeholder="e.g. •••••••••"
            v-model="loginPassword"
          />
        </label>
        <p
          class="error flex px-2 py-1 mt-4 -mb-3 text-sm bg-red-100 border rounded-full border-red-400 text-red-700"
          v-if="loginError"
        >
          <IconWarning class="h-5 w-5 text-red-600 mr-2" />
          <span>{{ loginError }}</span>
        </p>
        <p class="controls">
          <button
            class="flex transform transition-all mx-auto mt-8 bg-indigo-700 text-white pl-2 pr-3 py-2 rounded hover:bg-indigo-600 hover:-translate-y-0.5 shadow hover:shadow-md"
          >
            <IconLogin class="mr-2" />Login
          </button>
        </p>
      </form>
    </div>
  </div>
</template>

<script>
import { IconUser, IconLock, IconLogin } from "../../assets/Icons24.jsx";
import { IconWarning } from "../../assets/Icons20.jsx";
import Store from "../../store.js";

const $store = new Store();

export default {
  name: "login-prompt",

  components: {
    IconUser,
    IconLock,
    IconWarning,
    IconLogin,
  },

  data() {
    return {
      loginEmail: "",
      loginPassword: "",
      loginError: "",
    };
  },

  methods: {
    logIn() {
      this.loginError = "";

      console.log("Logging in…");

      $store
        .callApi("/api/v1/login", "POST", {
          email: this.loginEmail,
          password: this.loginPassword,
        })
        .then((response) => {
          if (response.ok) {
            response.json().then((data) => {
              $store.saveToken(data.token, data.user_id, data.expired_at);

              this.$router.push("/");
            });
          } else if (response.status == 404) {
            this.loginError = "Unknown email address.";
          } else if (response.status == 401) {
            this.loginError = "Incorrect password. Try again.";
          } else {
            response.text().then((text) => {
              this.loginError = text;
            });
          }
        });
    },
  },
};
</script>
