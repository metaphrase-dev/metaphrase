<template>
  <li class="nav-translation-key bg-goose-50 odd:bg-goose-100 group">
    <router-link
      :to="{
        name: 'Translation',
        params: { namespace: actualKey },
      }"
      :class="[
        'flex items-center text-sm pl-2 pr-1 h-9 group-hover:bg-blue-200',
        current ? 'bg-indigo-200' : '',
      ]"
    >
      <template v-if="isFolder">
        <IconFolder class="h-5 w-5 mr-2 flex-none" />
      </template>
      <template v-else>
        <IconDash class="h-5 w-5 mr-2 flex-none" />
      </template>

      <span class="flex-grow">{{ label }}</span>
      <template v-if="isFolder">
        <span
          class="bg-indigo-200 text-indigo-700 rounded-3xl font-semibold w-8 text-center text-sm group-hover:bg-blue-300 group-hover:text-blue-700"
          >{{ count }}</span
        >
        <IconRightChevron
          class="h-5 w-5 ml-2 mr-1 flex-none text-indigo-500 group-hover:text-blue-500"
        />
      </template>
    </router-link>
  </li>
</template>

<script>
import { IconFolder, IconDash, IconRightChevron } from "../../assets/Icons20.jsx";

export default {
  name: "navigation-key",

  props: {
    canonicalKey: String,
    namespace: String,
    forceCanonical: Boolean,
    count: Number,
  },

  components: {
    IconFolder,
    IconDash,
    IconRightChevron,
  },

  computed: {
    label() {
      if (this.forceCanonical) {
        return this.canonicalKey;
      } else {
        let key = this.canonicalKey;

        // Remove .. at the end of folders key
        if (this.isFolder) {
          key = key.slice(0, -2);
        }

        // Remove namespace from canonical (including dot)
        if (this.namespace.length > 0) {
          key = key.substring(this.namespace.length + 1);
        }

        return key;
      }
    },

    actualKey() {
      return this.canonicalKey.replace(/\.\.$/, "");
    },

    isFolder() {
      return this.canonicalKey.slice(-2) == "..";
    },

    current() {
      return this.actualKey === this.$route.params.namespace;
    },
  },
};
</script>
