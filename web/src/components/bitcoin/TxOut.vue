<script setup lang="ts">

import Frame from "./Frame.vue";
</script>

<template>
  <Frame title="TxOut" title-adjust="left" title-size="normal">
    <div>
      <span class="label">Amount:</span>
      <n-input size="small" style="min-width: 50%; margin: 0 .25em"
               autosize placeholder=""/>
    </div>
    <div style="margin-top: .25em">
      <span class="label">ScriptPubKey</span>
      <n-input type="textarea" size="small" placeholder=""/>
    </div>
  </Frame>
</template>

<style scoped>

</style>
