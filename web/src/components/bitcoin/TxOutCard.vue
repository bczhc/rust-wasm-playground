<script setup lang="ts">
import Frame from "./Frame.vue";

let valueModel = defineModel('value');
</script>

<template>
  <Frame title="TxOut" title-adjust="left" title-size="normal" show-close-icon>
    <div>
      <span class="label">Amount:</span>
      <n-input size="small" style="min-width: 50%; margin: 0 .25em"
               autosize placeholder=""
               v-model:value="valueModel.amount"
      />
    </div>
    <div style="margin-top: .25em">
      <span class="label">ScriptPubKey</span>
      <n-input type="textarea" size="small" placeholder=""
               v-model:value="valueModel.scriptPubKey"
      />
    </div>
  </Frame>
</template>

<style scoped>

</style>
