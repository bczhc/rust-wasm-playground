<script setup lang="ts">

import Frame from "./Frame.vue";
import TxIn from "./TxIn.vue";
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
      <n-divider style="padding: 0; margin: 0"/>
      <div id="txs-div">
        <div>
          <TxIn/>
          <TxIn/>
          <n-button type="primary">Add</n-button>
        </div>
        <span>
          =>
        </span>
        <div>
          <TxOut/>
          <TxOut/>
          <TxOut/>
          <TxOut/>
          <TxOut/>
          <TxOut/>
          <TxOut/>
          <TxOut/>
          <TxOut/>
          <n-button type="primary">Add</n-button>
        </div>
      </div>
    </Frame>
  </div>
</template>

<style scoped>
#root-TxBuilder {
  padding: 3px;
}

#txs-div {
  display: flex;
  justify-content: space-evenly;

  span {
    font-size: 3em;
    //border-left: 1px solid black;
    //border-right: 1px solid black;
    display: inline-flex;
    align-items: center;
    text-align: center;
    padding: 0 .25em;
  }

  > *:nth-child(1), > *:nth-child(3) {
    width: 100%;
  }
}
</style>
