<script setup lang="ts">
import Frame from "./Frame.vue";
import {safeParseInt, useWasm} from "../../lib.ts";
import {ref} from "vue";
import ScriptPubKeyHelper from "./ScriptPubKeyHelper.vue";

let valueModel = defineModel('value');
let emit = defineEmits(['close']);

let wasm = useWasm();
let showScriptPubKeyHelperModal = ref(false);
</script>

<template>
  <n-modal v-model:show="showScriptPubKeyHelperModal"
  >
    <n-card
        style="width: 600px"
        title="ScriptPubKey"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <ScriptPubKeyHelper/>
    </n-card>
  </n-modal>

  <Frame title="TxOut" title-adjust="left" title-size="normal" show-close-icon @close="emit('close')">
    <div>
      <span class="label">Amount:</span>
      <n-input size="small" style="min-width: 50%; margin: 0 .25em"
               autosize placeholder=""
               :value="valueModel.amount"
               @update:value="x => valueModel.amount = safeParseInt(x)"
      />
    </div>
    <div style="margin-top: .25em">
      <span class="label" style="cursor: pointer" @click="showScriptPubKeyHelperModal = true">ScriptPubKey</span>
      <n-input type="textarea" size="small" placeholder=""
               v-model:value="valueModel.scriptPubKey"
      />
    </div>
  </Frame>
</template>

<style scoped>

</style>
