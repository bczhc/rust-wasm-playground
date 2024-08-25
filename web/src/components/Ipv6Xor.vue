<script setup lang="ts">
import {useWasm} from "../lib.ts";
import {ref} from "vue";

const wasm = useWasm();

let ipv6_xor_hex = ref<string>('');
let password = ref<string>('');
let output = ref<string>('');

function onInput() {
  let utf8 = wasm.Utf8.to_utf8(password.value);
  let xor_mask = wasm.Blake3.blake3_xof(utf8, 16);
  let hex = wasm.Hex.decode(ipv6_xor_hex.value);
  if (hex && hex.length == 16) {
    for (let i = 0; i < hex.length; i++) {
      hex[i] ^= xor_mask[i];
    }
    output.value = wasm.Ipv6.bytes_to_ipv6(hex)!!;
  } else {
    output.value = 'Input hex must be of length 16';
  }
}
</script>

<template>
  <n-a href="https://bczhc.gitlab.io/ipv6/xor.txt">xor.txt</n-a>
  <n-input placeholder="IPv6 XOR Hex" v-model:value="ipv6_xor_hex" @input="onInput"/>
  <n-input placeholder="Password" v-model:value="password" @input="onInput"/>
  <n-input placeholder="Output" :value="output"/>
</template>

<style scoped>

</style>
