<template>
  <div id="main" :class="{ 'with-modal': modalShown }">
    <template v-if="loggedIn">
      <Toolbar @logout="didLogOut" />
      <div id="workspace" class="flex w-full items-stretch">
        <NavigationPanel
          :translation-keys="translationKeys"
          :namespace="relevantNamespace"
          @namespaceChanged="updateNamespace"
        />
        <div id="translation-list" class="flex-grow">
          <CollectionToolbar @showAddNewKey="showModal('add-new-key')" />
          <TranslationGroup
            v-for="key in filteredTranslationKeys"
            :key="key"
            :translation-key="key"
            :translations="store.groupedTranslations[key]"
          />
        </div>
      </div>
      <div id="modal-background" v-if="modalShown">
        <component
          :is="`${modalName}-modal`"
          :store="store"
          :currentNamespace="relevantNamespace"
          @submitModal="modalSubmitted"
          @closeModal="resetModal"
        ></component>
      </div>
    </template>
    <LoginPrompt v-else @didLogIn="didLogIn" />
  </div>
</template>

<script>
import Toolbar from "./components/Toolbar.vue";
import TranslationGroup from "./components/TranslationGroup.vue";
import CollectionToolbar from "./components/CollectionToolbar.vue";
import NavigationPanel from "./components/NavigationPanel.vue";
import LoginPrompt from "./components/LoginPrompt.vue";

import AddNewKeyModal from "./components/modals/AddNewKeyModal.vue";

import _ from "lodash";

export default {
  name: "app",

  created() {
    if (this.store.token !== null) {
      this.store.fetchTranslations();
    }
  },

  props: {
    store: Object,
  },

  data() {
    return {
      modalName: "",
    };
  },

  computed: {
    translationKeys() {
      return _.keys(this.store.groupedTranslations);
    },

    filteredTranslationKeys() {
      // FIXME: Implement paging instead of limiting to 50st first keys

      return _.filter(this.translationKeys, (key) => {
        return _.startsWith(key, this.store.namespace);
      })
        .sort()
        .slice(0, 50);
    },

    relevantNamespace() {
      let keysCount = this.filteredTranslationKeys.length;
      let namespace = this.store.namespace;

      if (keysCount === 1 && this.filteredTranslationKeys[0] === namespace) {
        if (namespace.includes(".")) {
          return namespace.split(".").slice(0, -1).join(".");
        } else {
          return "";
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
  },

  components: {
    Toolbar,
    TranslationGroup,
    NavigationPanel,
    LoginPrompt,
    CollectionToolbar,

    // Modals
    AddNewKeyModal,
  },

  methods: {
    updateNamespace(namespace) {
      window.location.hash = namespace;
    },

    didLogIn(token, userId, expiredAt) {
      this.store.saveToken(token, userId, expiredAt);
      this.store
        .fetchTranslations()
        .then(this.$nextTick((_) => window.location.reload()));
    },

    didLogOut() {
      this.store
        .callApi("/api/v1/logout", "POST")
        .then((_) => this.store.resetToken())
        .then((_) => window.location.reload());
    },

    showModal(modal) {
      this.modalName = modal;
    },

    resetModal() {
      this.modalName = "";
    },

    modalSubmitted(modalName, data) {
      if (modalName == "add-new-key") {
        this.store
          .callApi(
            "/api/v1/translations",
            "POST",
            // FIXME: It would be better to bootstrap the content of this
            //        empty translation with content from fields in the modal.
            {
              key: data.newKey,
              locale: "fr",
              content: "", // FIXME:
            }
          )
          .then((response) => response.json())
          .then((data) => {
            // TODO: Add incremental fetch here not to refresh all the
            //       translation store.
            this.store.fetchTranslations();

            this.updateNamespace(data.translation.key);
          });

        this.resetModal();
      }
    },
  },
};
</script>

<style>
#workspace {
  min-height: calc(100vh - 3rem);
}
/*
body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Roboto", "Oxygen",
    "Ubuntu", "Cantarell", "Fira Sans", "Droid Sans", "Helvetica Neue",
    sans-serif;
  margin: 0;
  padding: 0;
}

* {
  box-sizing: border-box;
}

#main {
  height: 100vh;
  width: 100%;

  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: stretch;

  background-color: #eeeeee;
}

#workspace {
  flex: 1 1 300px;
  display: flex;
  background-color: #d5d8de;
  overflow: hidden;
}

#translation-list {
  flex: 1 1 100px;
  overflow-y: auto;
}

#modal-background {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: rgba(64, 64, 64, 0.5);
  display: flex;
  align-items: center;
}

.with-modal #toolbar,
.with-modal #workspace {
  transition: filter 1s;
  filter: saturate(10%) brightness(60%);
}

.modal {
  width: 600px;
  max-width: 75vw;
  min-height: 150px;
  margin: auto;
  background-color: white;
  border-radius: 6px;
  padding: 10px;
  box-shadow: 2px 2px 2px 2px rgba(0, 0, 0, 0.2);
  position: relative;
}

.modal h2 {
  margin-top: 0;
  background-color: lightgray;
  margin: -10px -10px 0 -10px;
  border-top-left-radius: 6px;
  border-top-right-radius: 6px;
  padding: 10px;
  font-size: 20px;
}

.modal form {
  font-size: 10pt;
  margin: 0;
}

.label-standalone {
  display: block;
  margin: 10px 0;
}

.input-form {
  border: 1px solid lightgrey;
  border-radius: 4px;
  font-size: 90%;
  line-height: 1.5;
  font-family: inherit;
  padding: 5px;
}

.input-form:hover {
  border-color: #aaa;
}

.input-form:focus {
  border-color: grey;
}

.input-standalone {
  display: block;
  width: 100%;
}

.icon-prepend {
  position: absolute;
  margin-left: 6px;
  margin-top: 8px;
  color: grey;
  width: 16px;
  height: 16px;
}

.icon-prepend + input {
  padding-left: 26px;
}

.close-modal {
  position: absolute;
  top: 10px;
  right: 10px;
  color: grey;
  padding: 3px;
}

.close-modal:hover,
.close-modal:focus,
.close-modal:active {
  background-color: rgba(255, 255, 255, 0.2);
  border-radius: 4px;
  outline: 0;
}

form button.primary {
  color: white;
  background-color: #18a1db;
  font-family: inherit;
  font-size: inherit;
  border: 0 none;
  border-radius: 3px;
  line-height: 1.5;
  padding: 3px 7px;
  border: 1px solid rgba(0, 0, 0, 0.05);
}

form button.primary:hover {
  border: 1px solid rgba(0, 0, 0, 0.4);
}

footer {
  margin: 0 -10px -10px -10px;
  padding: 10px;
  text-align: right;
}
*/
</style>
