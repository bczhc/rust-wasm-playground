<script setup lang="ts">

import TxIn from "./TxIn.vue";
import Frame from "./Frame.vue";
import TxOut from "./TxOut.vue";
</script>

<template>
<div id="root-TxBuilder">
  <Frame title="Transaction" title-adjust="center" title-size="large">
    <n-form label-placement="top" inline>
      <n-form-item label="Version" style="margin: 0; padding: 0">
        <n-input placeholder="" size="small" style="min-width: 10em" autosize/>
      </n-form-item>
      <n-form-item label="LockTime" style="margin: 0; padding: 0">
        <n-input placeholder="" size="small" style="min-width: 10em" autosize/>
      </n-form-item>
    </n-form>
    <TxIn/>
    <TxOut/>
  </Frame>
</div>
</template>

<style scoped>
#root-TxBuilder {
  padding: 3px;
}
</style>
