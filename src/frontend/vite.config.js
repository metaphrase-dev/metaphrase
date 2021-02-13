import vue from "@vitejs/plugin-vue";
import vueJsx from "@vitejs/plugin-vue-jsx";

/**
 * https://vitejs.dev/config/
 * @type {import('vite').UserConfig}
 */
export default {
  plugins: [vue(), vueJsx()],
  server: {
    port: 3100,
    proxy: {
      "/api/": "http://localhost:3000/",
    },
  },
};
