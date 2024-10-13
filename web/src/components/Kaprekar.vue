<script setup lang="ts">
import {useWasm} from "../lib.ts";
import {ref} from "vue";
import {useMessage} from 'naive-ui';

let start = ref('0');
let end = ref('1000000000000');
let output = ref('');
let onProgress = ref(false);

const wasm = useWasm();
const message = useMessage();

function calculateClick() {
  onProgress.value = true;
  output.value = '';
  setTimeout(() => {
    try {
      wasm.Kaprekar.kaprekar(
          BigInt(start.value),
          BigInt(end.value),
          (a: bigint, b: bigint, c: bigint) => {
            output.value += `${a} ${b} ${c}\n`;
          })
    } catch (e: any) {
      message.error(`${e}`);
    } finally {
      onProgress.value = false;
    }
  }, 10);
}

</script>

<template>
  <n-input placeholder="Start" v-model:value="start"/>
  <n-input placeholder="End" v-model:value="end"/>
  <n-button type="primary" @click="calculateClick" :disabled="onProgress">Calculate</n-button>
  <n-input type="textarea" rows="20" :value="output"/>
</template>

<style scoped>

</style>
