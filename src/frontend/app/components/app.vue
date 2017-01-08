<template>
  <div id="main" :class="{'with-modal': modalShown}">
    <template v-if="loggedIn">
      <toolbar @logout="didLogOut" />
      <div id="workspace">
        <navigation-bar
          :translation-keys="translationKeys"
          :namespace="relevantNamespace"
          @namespaceChanged="updateNamespace" />
        <div id="translation-list">
          <collection-toolbar
            @showAddNewKey="showModal('add-new-key')" />
          <translation-group
            v-for="key in filteredTranslationKeys"
            :translation-key="key"
            :translations="store.groupedTranslations[key]" />
        </div>
      </div>
      <div id="modal-background" v-if="modalShown">
        <component :is="`${modalName}-modal`"
          :store="store"
          :currentNamespace="relevantNamespace"
          @submitModal="modalSubmitted"
          @closeModal="resetModal"></component>
      </div>
    </template>
    <login-prompt v-else @didLogIn="didLogIn">
  </div>
</template>

<script>
  import Toolbar from "./toolbar.vue";
  import TranslationGroup from "./translation-group.vue";
  import CollectionToolbar from "./collection-toolbar.vue";
  import NavigationBar from "./navigation-bar.vue";
  import LoginPrompt from "./login-prompt.vue";

  import AddNewKeyModal from "./modals/add-new-key-modal.vue";

  import _ from "lodash";

  export default ({
    name: 'app',

    created() {
      if (this.store.token !== null) {
        this.fetchTranslations();
      }
    },

    props: {
      store: Object
    },

    data() {
      return {
        modalName: ''
      }
    },

    computed: {
      translationKeys() {
        return _.keys(this.store.groupedTranslations);
      },

      filteredTranslationKeys() {
        // FIXME: Implement paging instead of limiting to 50st first keys

        return _.filter(this.translationKeys, (key) => {
          return _.startsWith(key, this.store.namespace);
        }).sort().slice(0, 50);
      },

      relevantNamespace() {
        let keysCount = this.filteredTranslationKeys.length;
        let namespace = this.store.namespace;

        if (keysCount === 1 && this.filteredTranslationKeys[0] == namespace) {
          if (namespace.includes('.')) {
            return namespace.split('.').slice(0, -1).join('.');
          } else {
            return '';
          }
        } else {
          return namespace;
        }
      },

      loggedIn() {
        return this.store.token !== null;
      },

      modalShown() {
        return this.modalName.length > 0;
      },

      headers() {
        let token = this.store.token;

        return new Headers({
          Authorization: `Bearer ${token}`,
          'Content-Type': 'application/json'
        });
      }
    },

    components: {
      toolbar: Toolbar,
      translationGroup: TranslationGroup,
      navigationBar: NavigationBar,
      loginPrompt: LoginPrompt,
      collectionToolbar: CollectionToolbar,

      // Modals
      addNewKeyModal: AddNewKeyModal
    },

    methods: {
      updateNamespace(namespace) {
        window.location.hash = namespace;
      },

      didLogIn(token, userId, expiredAt) {
        this.applyToken(token, userId, expiredAt);
        this.fetchTranslations();
      },

      didLogOut() {
        fetch("/api/v1/logout", {
          method: 'POST',
          headers: this.headers
        }).then(_ => this.resetToken());
      },

      applyToken(token, userId, expiredAt) {
        this.store.token = token;
        this.store.userId = userId;
        this.store.expiredAt = expiredAt;

        window.localStorage.setItem('token', token);
        window.localStorage.setItem('userId', userId);
        window.localStorage.setItem('expiredAt', expiredAt);
      },

      resetToken() {
        this.store.token = null;
        this.store.userId = null;
        this.store.expiredAt = null;

        window.localStorage.removeItem('token');
        window.localStorage.removeItem('userId');
        window.localStorage.removeItem('expiredAt');
      },

      fetchTranslations() {
        fetch("/api/v1/translations", {
          headers: this.headers
        })
          .then(response => {
            if(response.ok) {
              return response.json();
            } else if (response.status == 401) {
              this.resetToken();
            } else {
              console.error(`Received ${reponse.status} status while fetching translations, aborting`);
            }
          })
          .then(data => {
            this.store.groupedTranslations = data;
          });
      },

      showModal(modal) {
        this.modalName = modal;
      },

      resetModal() {
        this.modalName = '';
      },

      modalSubmitted(modalName, data) {
        if (modalName == 'add-new-key') {
          fetch("/api/v1/translations", {
            method: "POST",
            headers: this.headers,
            body: JSON['stringify'](
              // FIXME: It would be better to bootstrap the content of this
              //        empty translation with content from fields in the modal.
              {
                key: data.newKey,
                locale: 'fr',
                content: '' // FIXME:
              }
            )
          }).then(response => response.json())
            .then(data => {
              // TODO: Add incremental fetch here not to refresh all the
              //       translation store.
              this.fetchTranslations();

              this.updateNamespace(data.key);
            });

          this.resetModal();
        }
      }
    }
  });
</script>
