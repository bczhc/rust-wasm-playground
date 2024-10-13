<script setup lang="ts">
import {computed, ref} from "vue";
import {to_hex_padded, useWasm} from "../lib.ts";
import {LegEmulationResult} from "wasm-lib";

let wasm = useWasm();

let code = ref('');
let assemblyHex = ref('');
let programInput = ref('');
let maxCpuCycles = ref('');

let emulationResult = ref<LegEmulationResult | null>(null);

function runClick() {
  let cyclesLimit: BigInt | null = null;
  try {
    if (maxCpuCycles.value !== '') {
      cyclesLimit = BigInt(maxCpuCycles.value);
    }
  } catch (_) {
  }
  try {
    let target = wasm.LegCpu.assemble(code.value);
    assemblyHex.value = target.commented_binary;
    emulationResult.value = wasm.LegCpu.emulate(target.binary, programInput.value, cyclesLimit);
  } catch (e) {
    assemblyHex.value = e.toString();
  }
}

let output_binary_hex = computed(() => {
  let r = emulationResult.value as LegEmulationResult | null;
  if (r == null) {
    return null;
  }
  // Uint8Array's map differs from the Array one. It returns Uint8Array still instead of an array of new type. 😡
  return Array.from(r.output).map(x => to_hex_padded(x, 2)).join(' ');
});
</script>

<template>
  <div class="outer">
    <div class="top-view">
      <n-input placeholder="Code" type="textarea" class="side-by-side" :resizable="false"
               v-model:value="code"
      />
      <n-input placeholder="Hex" type="textarea" class="side-by-side" :resizable="false"
               :allow-input="() => false"
               v-model:value="assemblyHex"
      />
    </div>
    <div class="middle-view">
      <n-form
          label-placement="left"
          :label-width="120"
      >
        <n-form-item label="Program Input">
          <n-input v-model:value="programInput"/>
        </n-form-item>
        <n-form-item label="Max CPU cycles">
          <n-input v-model:value="maxCpuCycles"/>
        </n-form-item>
        <n-button type="primary" @click="runClick">Run</n-button>
      </n-form>
    </div>
    <n-divider style="padding: 0; margin: 0"/>
    <div class="bottom-view">
      <div class="output-line-wrapper" v-if="emulationResult">
        <span>Output: {{ emulationResult!!.output_lossy_string }}</span>
        <span>Output (hex): {{ output_binary_hex }} </span>
        <span>CPU cycles: {{ emulationResult!!.cpu_cycles }}</span>
        <span>Interrupted: {{ emulationResult!!.interrupted }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.top-view {
  height: 60%;
}

.outer {
  padding: 0.5em;
  height: 100vh;
}

.output-line-wrapper > span {
  display: block;
}

.side-by-side {
  height: 100%;
  width: 50%;
}

.middle-view {
  margin: 0.5em auto;
}
</style>
