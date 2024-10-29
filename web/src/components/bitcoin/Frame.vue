<script setup lang="ts">
import {computed} from "vue";

const props = defineProps<{
  title: string,
  titleAdjust: 'left' | 'center',
  titleSize: string,
}>();

let titleCentered = computed(() => {
  return props.titleAdjust === 'center'
});
</script>

<template>
  <div id="root-Frame">
    <div class="title-wrapper" :class="titleCentered ? 'center' : 'left'">
      <span class="title">{{ props.title }}</span>
    </div>
    <slot/>
  </div>
</template>

<style scoped>
/*
Must use unique id here, to prevent id collision.
Scoped CSS doesn't work here. It also applies to children components.
This is a bug.

Maybe related: https://github.com/vuejs/core/issues/12159

The bugfix PR is two weeks ago, and not been merged YET.
I'm wondering if this was also present in older versions, or just
a regression recently.

Thank you, Vue.js.
*/
#root-Frame {
  border: solid black 1px;
  border-radius: 5px;
  padding: 5px;
}

.title-wrapper {
  display: flex;
}

.title-wrapper.center {
  justify-content: center;
}

.title {
  font-size: v-bind(titleSize);
  font-weight: bold;
}
</style>
