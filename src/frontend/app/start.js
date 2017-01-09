import Vue from 'vue';
import App from 'components/app.vue';
import Store from './store.js';
import _ from 'lodash';

var store = new Store();

window.onhashchange = function() {
  store.namespace = window.location.hash.substring(1);
}

store.applyLocalStorage();

window.vue = new Vue({
  el: '#app',
  data: store,
  render(createElement) {
    return createElement('app', {
      props: {
        store: this.$data
      }
    });
  },
  components: {
    'app': App
  }
});
