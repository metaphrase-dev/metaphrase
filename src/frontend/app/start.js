import Vue from 'vue';
import App from 'components/app.vue';
import _ from 'lodash';
import "whatwg-fetch";

var store = {
  groupedTranslations: [],
  namespace: window.location.hash.substring(1),
  token: null,
  userId: null,
  expiredAt: null
};

window.onhashchange = function() {
  store.namespace = window.location.hash.substring(1);
}

// Fill store on login based on localStorage content
if (localStorage.getItem('token') !== null) {
  store.token = localStorage.getItem('token');
  store.userId = localStorage.getItem('userId');
  store.expiredAt = localStorage.getItem('expiredAt');
}

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
