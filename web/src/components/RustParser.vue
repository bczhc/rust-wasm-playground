<script setup lang="ts">
import {computed, ref} from "vue";
import {useWasm} from "../lib.ts";

let code = ref('');
const wasm = useWasm();

let syntax = computed(() => {
  try {
    return wasm.RustParser.parse_file(code.value).syntax;
  } catch (e: any) {
    return e.toString();
  }
});

let reassembled = computed(() => {
  try {
    return wasm.RustParser.parse_file(code.value).reassembled;
  } catch (e: any) {
    return e.toString();
  }
});
</script>

<template>
  <n-input type="textarea" id="code-input" v-model:value="code" placeholder="Rust code"/>
  <div style="height: 50vh">
    <n-input placeholder="Syntax" type="textarea" class="bottom-left" :value="syntax"/>
    <n-input placeholder="Reassembled" type="textarea" class="bottom-right" :value="reassembled"/>
  </div>
</template>

<style scoped>
#code-input {
  height: 50vh;
}

.bottom-left, .bottom-right {
  float: left;
  width: 50%;
  box-sizing: border-box;
  height: 50vh;
}
</style>
