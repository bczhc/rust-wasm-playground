<script setup lang="ts">
import * as wasm from 'wasm-lib';
import {ref} from "vue";

wasm.init();
let string = wasm.build_string('world');
wasm.greet()

let h1String = ref(string);
</script>

<template>
  <h1>{{ h1String }}</h1>
</template>

<style scoped>
</style>
