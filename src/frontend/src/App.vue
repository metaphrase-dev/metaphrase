<template>
  <div id="main">
    <router-view></router-view>
  </div>
</template>

<script>
import LoginPrompt from "./components/screens/LoginPrompt.vue";
import Workspace from "./components/screens/Workspace.vue";

export default {
  name: "app",

  components: {
    LoginPrompt,
    Workspace,
  },
};
</script>

<style>
#workspace {
  min-height: calc(100vh - 3rem);
}
</style>
