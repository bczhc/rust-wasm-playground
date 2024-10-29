<script setup lang="ts">
import Frame from "./Frame.vue";
</script>

<template>
  <Frame title="TxIn" title-adjust="left" title-size="normal">
    <div class="cell">
      <span class="label">Outpoint</span>
      <div id="outpoint-line">
        <n-input size="small" class="input1" placeholder="TxId"/>
        <n-input size="small"
                 placeholder="Idx"
                 :allow-input="x => /^\d*$/.test(x)" class="input2"/>
      </div>
    </div>
    <div class="cell">
      <span class="label">Sequence:</span>
      <n-input size="small" style="min-width: 50%; margin: 0 .25em"
               autosize placeholder=""/>
    </div>
    <div class="cell">
      <span class="label">ScriptSig</span>
      <n-input size="small" type="textarea" placeholder=""/>
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
