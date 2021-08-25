import { createRouter, createWebHistory } from "vue-router";
import Workspace from "./components/screens/Workspace.vue";
import LoginPrompt from "./components/screens/LoginPrompt.vue";
import Store from "./store.js";

const $store = new Store();

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/sign-in/", component: LoginPrompt, name: "Login" },
    { path: "/t/:namespace(.*)", component: Workspace, name: "Translation" },
    { path: "/", component: Workspace, name: "Home" },
  ],
});

router.beforeEach((to) => {
  const loggedIn = $store.token !== null && $store.token !== undefined;

  if (to.name === "Login" && loggedIn) {
    return { name: "Home" };
  } else if (to.name !== "Login" && !loggedIn) {
    return { name: "Login" };
  } else {
    return true;
  }
});

export default router;
