import { createApp } from "vue";
import App from "./App.vue";
import Store from "./store.js";
import "./index.css";

const store = new Store();

window.onhashchange = function () {
  store.namespace = window.location.hash.substring(1);
};

store.applyLocalStorage();

createApp(App, { store }).mount("#app");
