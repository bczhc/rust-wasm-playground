<script setup lang="ts">
import Frame from "./Frame.vue";
import TxInCard from "./TxInCard.vue";
import {computed, Ref, ref} from "vue";
import {
  CHECK_DIGITS,
  defaultTxIn,
  defaultTxOut,
  GLOBAL_NETWORK,
  NetworkType,
  Transaction,
  TxIn,
  TxOut, updateNetwork
} from "../../bitcoin.ts";
import {ArrowForward as Arrow} from '@vicons/ionicons5';
import TxOutCard from "./TxOutCard.vue";
import {safeParseInt, useWasm} from "../../lib.ts";

let wasm = useWasm();

let networkOptions: { label: string, value: NetworkType }[] = [
  {value: 'bitcoin', label: 'Bitcoin'},
  {value: 'testnet', label: 'Testnet3'},
  {value: 'testnet4', label: 'Testnet4'},
  {value: 'sigtest', label: 'Sigtest'},
  {value: 'regtest', label: 'Regtest'},
];
let selectedNetwork = ref<NetworkType>('bitcoin');

let version = ref(1);
let lockTime = ref(0);

let txIns: Ref<TxIn[]> = ref([]);
let txOuts: Ref<TxOut[]> = ref([]);

let transaction = computed<Transaction>(() => {
  return {
    version: version.value,
    lockTime: lockTime.value,
    in: txIns.value,
    out: txOuts.value,
  };
});

let transactionHex = computed(() => {
  try {
    return wasm.TxBuilder.json_to_tx_hex(JSON.stringify(transaction.value));
  } catch (e: any) {
    return e.toString();
  }
});
</script>

<template>
  <div id="root-TxBuilder">
    <Frame title="Transaction" title-adjust="center" title-size="large">
      <n-form label-placement="top" inline>
        <n-form-item label="Version" style="margin: 0; padding: 0">
          <n-input placeholder="" size="small" style="min-width: 10em" autosize
                   :allow-input="CHECK_DIGITS"
                   :value="version"
                   @update:value="x => version = safeParseInt(x)"/>
        </n-form-item>
        <n-form-item label="LockTime" style="margin: 0; padding: 0">
          <n-input placeholder="" size="small" style="min-width: 10em" autosize
                   :allow-input="CHECK_DIGITS"
                   :value="lockTime"
                   @update:value="x => lockTime = safeParseInt(x)"/>
        </n-form-item>
        <n-form-item label="Network" style="margin: 0; padding: 0">
          <n-select :options="networkOptions" v-model:value="selectedNetwork"
                    size="small" style="min-width: 10em"
                    @update:value="x => updateNetwork(x)"
          />
        </n-form-item>
      </n-form>
      <div id="txs-div">
        <div>
          <TxInCard v-for="(_, index) in txIns" v-model:value="txIns[index]"
                    @close="txIns.splice(index, 1)"/>
          <n-button type="primary" @click="txIns.push(defaultTxIn())">Add
          </n-button>
        </div>
        <div>
          <Arrow v-if="txIns.length !== 0 && txOuts.length !== 0"/>
        </div>
        <div>
          <TxOutCard v-for="(_, index) in txOuts" v-model:value="txOuts[index]"
                     @close="txOuts.splice(index, 1)"
          />
          <n-button type="primary" @click="txOuts.push(defaultTxOut())">Add</n-button>
        </div>
      </div>
    </Frame>
    <div style="margin-top: 1em">Transaction:</div>
    <pre id="tx-output">{{ JSON.stringify(transaction, null, 2) }}</pre>
    <span>Consensus Encoded</span>
    <n-input type="textarea" rows="10" :value="transactionHex"/>
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

#tx-output {
  margin: 0;
}
</style>
