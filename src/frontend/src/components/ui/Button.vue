<template>
  <button
    class="transform transition-transform bg-gray-50 flex flex-row pl-1.5 pr-3 py-1.5 text-sm font-medium rounded shadow-sm hover:shadow-md hover:-translate-y-0.5"
  >
    <div class="text-indigo-700 mr-1">
      <slot name="icon" />
    </div>
    <slot></slot>
  </button>
</template>

<script>
export default {};
</script>