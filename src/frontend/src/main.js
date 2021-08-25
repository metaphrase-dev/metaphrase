import { createApp } from "vue";
import App from "./App.vue";
import Store from "./store.js";
import router from "./router.js";
import "./index.css";

const store = new Store();

store.applyLocalStorage();

const app = createApp(App, { store });

app.use(router);
app.mount("#app");
