<template>
  <div id="navigation-bar">
    <ul class="nav-translation-key-list">
      <li v-for="key in translationKeys" class="nav-translation-key">
        <a href="#" @click.prevent>
          <i class="fa fa-fw fa-file-o"></i>
          <span>{{ key }}</span>
          <i class="fa fa-fw fa-chevron-right"></i>
        </a>
      </li>
    </ul>
  </div>
</template>

<script>
  export default ({
    name: 'navigation-bar',

    props: {
      translationKeys: Array
    }
  });
</script>
