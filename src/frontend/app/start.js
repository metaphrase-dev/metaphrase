import Vue from 'vue';
import App from 'components/app.vue';
import _ from 'lodash';
import "whatwg-fetch";

var store = {
  groupedTranslations: []
};

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

fetch("/api/translations")
  .then(response => response.json())
  .then(data => {
    store.groupedTranslations = data;
  });
