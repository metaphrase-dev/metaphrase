<template>
  <div class="collection-toolbar h-12 flex items-center pl-8 flex-row bg-indigo-700">
    <Button @click="newKeyModalShown = true">
      <template v-slot:icon>
        <IconPlus />
      </template>
      New key
    </Button>
    <teleport to="body">
      <AddNewKeyModal
        v-if="newKeyModalShown"
        @closeModal="newKeyModalShown = false"
        :currentNamespace="currentNamespace"
      />
    </teleport>
  </div>
</template>

<script>
import { IconPlus } from "../../assets/Icons20.jsx";
import Button from "../common/Button.vue";
import AddNewKeyModal from "../modals/AddNewKeyModal.vue";

export default {
  name: "collection-toolbar",

  components: {
    AddNewKeyModal,
    IconPlus,
    Button,
  },

  data() {
    return {
      newKeyModalShown: false,
    };
  },

  computed: {
    currentNamespace() {
      return `${this.$route.params.namespace ?? ""}`;
    },
  },
};
</script>
