<script setup lang="ts">
import Frame from "./Frame.vue";
import TxInCard from "./TxInCard.vue";
import TxOutCard from "./TxOutCard.vue";
import {computed, Ref, ref} from "vue";
import {CHECK_DIGITS, Transaction, TxIn, TxOut} from "../../bitcoin.ts";
import {ArrowForward as Arrow} from '@vicons/ionicons5';

let version = ref(1);
let lockTime = ref(0);

let transaction = computed<Transaction>(() => {
  return {
    version: version.value,
    lockTime: lockTime.value,
    in: [], /* TODO */
    out: [],
  }
});

let txIn1: Ref<TxIn> = ref({
  outpointTxId: "hhh",
  outpointIndex: 1,
  scriptSig: "aabb",
  sequence: 123,
});

let txOut1: Ref<TxOut> = ref({
  amount: 213123,
  scriptPubKey: "xx",
});
</script>

<template>
  <div id="root-TxBuilder">
    <Frame title="Transaction" title-adjust="center" title-size="large">
      <n-form label-placement="top" inline>
        <n-form-item label="Version" style="margin: 0; padding: 0">
          <n-input placeholder="" size="small" style="min-width: 10em" autosize
                   :allow-input="CHECK_DIGITS" v-model:value="version"/>
        </n-form-item>
        <n-form-item label="LockTime" style="margin: 0; padding: 0">
          <n-input placeholder="" size="small" style="min-width: 10em" autosize
                   :allow-input="CHECK_DIGITS" v-model:value="lockTime"/>
        </n-form-item>
      </n-form>
      <div id="txs-div">
        <div>
          <TxInCard v-model:value="txIn1"/>
          <n-button type="primary">Add</n-button>
        </div>
        <div>
          <Arrow/>
        </div>
        <div>
          <TxOutCard v-model:value="txOut1"/>
          <n-button type="primary">Add</n-button>
        </div>
      </div>
    </Frame>
    <div>Transaction:</div>
    {{ JSON.stringify(transaction) }}
    <br>
    {{ JSON.stringify(txIn1) }}
  </div>
</template>

<style scoped>
#root-TxBuilder {
  padding: 3px;
}

#txs-div {
  display: flex;
  justify-content: space-evenly;

  > *:nth-child(2) {
    width: 8em;
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
