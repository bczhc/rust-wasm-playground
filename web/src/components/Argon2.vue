<script setup lang="ts">
import {ref} from "vue";
import {useWasm} from "../lib.ts";

const Argon2 = useWasm().Argon2;

let dataInput = ref<string>('')
let saltInput = ref<string>('')
let methodInput = ref<string>("argon2id")
let mCostInput = ref<string>('4096')
let tCostInput = ref<string>('30')
let pCostInput = ref<string>('1')
let hashLenInput = ref<string>('32')
let output = ref<string>('')

let timeTookMs = ref<number | null>(null)

function generate() {
  return Argon2.hash(
      methodInput.value,
      Argon2.string_to_utf8(dataInput.value),
      saltInput.value,
      parseInt(mCostInput.value),
      parseInt(tCostInput.value),
      parseInt(pCostInput.value),
      parseInt(hashLenInput.value),
  )
}

function generateWithTimer() {
  try {
    let start = Date.now();
    output.value = generate();
    let end = Date.now();
    timeTookMs.value = end - start;
  } catch (e) {
    console.error(e);
    output.value = `${e}`
  }
}

</script>

<template>
  <label for="data">Plain Text Input</label><br>
  <input type="text" id="data" placeholder="Data" v-model="dataInput">
  <br><br>

  <label for="method">Method</label><br>
  <select name="method" id="method" v-model="methodInput">
    <option value="argon2i">Argon2i</option>
    <option value="argon2d">Argon2d</option>
    <option value="argon2id">Argon2id</option>
  </select>
  <br><br>

  <label for="salt">Salt</label>
  <input type="text" id="salt" name="salt" v-model="saltInput">
  <input type="button" value="Random" id="random-salt-btn" @click="saltInput = Argon2.random_salt()">
  <br><br>

  <label for="m-cost">Memory Cost</label>
  <input type="number" id="m-cost" v-model="mCostInput">
  <br><br>

  <label for="t-cost">Iterations</label>
  <input type="number" id="t-cost" v-model="tCostInput">
  <br><br>

  <label for="p-cost">Parallelism Degree</label>
  <input type="number" id="p-cost" v-model="pCostInput">
  <br><br>

  <label for="hash-length">Output Hash Length</label>
  <input type="number" id="hash-length" v-model="hashLenInput">
  <br><br>

  <input type="button" value="Generate Hash" id="generate" @click="generateWithTimer">
  <br><br>

  <p id="output">{{ output }}</p>
  <p id="time-took" v-if="timeTookMs">Time took: {{ timeTookMs }} ms</p>
</template>

<style scoped>

</style>
