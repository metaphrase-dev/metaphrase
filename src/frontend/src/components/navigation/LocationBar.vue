<script setup>
import { computed } from "@vue/reactivity";
import { useRoute } from "vue-router";
import { IconHome, IconFolder, IconOpenFolder, IconDotsHorizontal } from "../../assets/Icons20.jsx";

const props = defineProps({
  translationKeys: Array,
});

const route = useRoute();
const namespace = computed(() => route.params.namespace ?? "");

const crumbs = computed(() => {
  const split = namespace.value.split(".");
  const objs = split.map((crumb, index, allCrumbs) => ({
    label: crumb,
    key: allCrumbs.slice(0, Number(index) + 1).join("."),
  }));
  return objs.filter((crumb) => !props.translationKeys.includes(crumb.key));
});

const MAX_CRUMBS = 2;
const lastCrumbs = computed(() => crumbs.value.slice(-MAX_CRUMBS));
const areCrumbsTruncated = computed(() => crumbs.value.length > MAX_CRUMBS);
</script>

<template>
  <div
    class="location-bar flex flex-row bg-indigo-800 items-center h-12 bz-right-15px mr-negative-15px"
  >
    <button
      v-if="areCrumbsTruncated"
      class="flex flex-row items-center bg-indigo-500 px-2 rounded-r-md text-white z-20 h-8"
    >
      <IconDotsHorizontal />
    </button>
    <router-link
      class="flex flex-row items-center bg-indigo-300 px-2 rounded-r-md text-indigo-900 z-20 h-8"
      :to="{ name: 'Home' }"
      v-else
    >
      <IconHome />
    </router-link>
    <template v-if="namespace !== ''">
      <router-link
        v-for="({ label, key }, index) in lastCrumbs"
        :to="{ name: 'Translation', params: { namespace: key || '' } }"
        :class="[
          'flex flex-row items-center px-2 py-1 rounded-r-md text-indigo-900 text-sm -ml-1 pl-3 italic h-8 tracking-tighter',
          Number(index) > 0 ? 'bg-indigo-100' : 'bg-indigo-200 z-10',
        ]"
      >
        <IconFolder class="mr-1" v-if="Number(index) + 1 < lastCrumbs.length" />
        <IconOpenFolder class="mr-1" v-else />
        {{ label }}
      </router-link>
    </template>
  </div>
</template>
