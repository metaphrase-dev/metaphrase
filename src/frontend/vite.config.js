import vue from "@vitejs/plugin-vue";
import vueJsx from "@vitejs/plugin-vue-jsx";
import pluginRewriteAll from "vite-plugin-rewrite-all";

/**
 * https://vitejs.dev/config/
 * @type {import('vite').UserConfig}
 */
export default {
  plugins: [vue(), vueJsx(), pluginRewriteAll()],
  server: {
    port: 3100,
    proxy: {
      "/api/": "http://localhost:3000/",
    },
  },
};
