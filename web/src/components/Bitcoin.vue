<script setup lang="ts">
import {computed, ref} from "vue";
import {stringifyFallible, useWasm} from "../lib.ts";

const wasm = useWasm();

let scriptHexInput = ref('');
let scriptAsmOutput = computed(() => {
  return stringifyFallible(() => wasm.Bitcoin.parse_script_hex(scriptHexInput.value))
});
let ripemd160Input = ref('');
let ripemd160Output = computed(() => {
  return stringifyFallible(() => wasm.Bitcoin.ripemd160(ripemd160Input.value))
});
let base58CheckInput = ref('');
let base58CheckOutput = computed(() => {
  return stringifyFallible(() => wasm.Bitcoin.base58_check(base58CheckInput.value));
})
</script>

<template>
  <div id="root">
    script hex
    <n-input v-model:value="scriptHexInput"/>
    assembly output:
    <div class="auto-wrap">{{ scriptAsmOutput }}</div>
    <n-divider/>

    hex
    <n-input v-model:value="ripemd160Input"/>
    ripemd160:
    <div class="auto-wrap">{{ ripemd160Output }}</div>

    hex
    <n-input v-model:value="base58CheckInput"/>
    base58Check:
    <div class="auto-wrap">{{ base58CheckOutput }}</div>
  </div>
</template>

<style scoped>
#root {
  padding: .5em;
}

.auto-wrap {
  word-break: break-word;
  white-space: normal;
}
</style>
