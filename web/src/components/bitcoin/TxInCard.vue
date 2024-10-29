<script setup lang="ts">
import Frame from "./Frame.vue";
import {TxIn} from "../../bitcoin.ts";
import {safeParseInt} from "../../lib.ts";

let valueModel = defineModel<TxIn>('value');
let emit = defineEmits(['close']);
</script>

<template>
  <Frame title="TxIn" title-adjust="left" title-size="normal" show-close-icon @close="emit('close')">
    <div class="cell">
      <span class="label">Outpoint</span>
      <div id="outpoint-line">
        <n-input size="small" class="input1" placeholder="TxId" v-model:value="valueModel.outpointTxId"/>
        <n-input size="small"
                 placeholder="Idx"
                 :allow-input="x => /^\d*$/.test(x)" class="input2"
                 :value="valueModel.outpointIndex"
                 @update:value="x => valueModel.outpointIndex = safeParseInt(x)"/>
      </div>
    </div>
    <div class="cell">
      <span class="label">Sequence:</span>
      <n-input size="small" style="min-width: 50%; margin: 0 .25em"
               autosize placeholder=""
               :value="valueModel.sequence"
               @update:value="x => valueModel.sequence = safeParseInt(x)"
      />
    </div>
    <div class="cell">
      <span class="label">ScriptSig</span>
      <n-input size="small" type="textarea" placeholder=""
               v-model:value="valueModel.scriptSig"/>
    </div>
  </Frame>
</template>

<style scoped>
#outpoint-line {
  display: flex;

  .input2 {
    max-width: 5em;
  }
}

.cell {
  margin: .25em 0;
}
</style>
