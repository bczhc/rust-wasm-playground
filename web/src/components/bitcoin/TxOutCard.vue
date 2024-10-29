<script setup lang="ts">
import Frame from "./Frame.vue";
import {safeParseInt, useWasm} from "../../lib.ts";
import {ref} from "vue";
import {CreateOutline as CreateIcon, InformationOutline as InfoIcon} from '@vicons/ionicons5';
import SelectableIcon from "./SelectableIcon.vue";
import ScriptAsmModal from "./ScriptAsmModal.vue";
import ScriptPubKeyFromAddressModal from "./ScriptPubKeyFromAddressModal.vue";

let valueModel = defineModel('value');
let emit = defineEmits(['close']);

let wasm = useWasm();
let showModal = ref({
  scriptPubKeyHelper: false,
  scriptPubKeyInfo: false,
});

let props = defineProps<{
  index: number,
}>();
</script>

<template>
  <ScriptPubKeyFromAddressModal v-model:show="showModal.scriptPubKeyHelper"
                                @result="x => valueModel.scriptPubKey = x"/>
  <ScriptAsmModal :script-hex="valueModel.scriptPubKey" v-model:show="showModal.scriptPubKeyInfo"/>

  <Frame :title="`TxOut #${index}`" title-adjust="left" title-size="normal" show-close-icon @close="emit('close')">
    <div>
      <span class="label">Amount:</span>
      <n-input size="small" style="min-width: 50%; margin: 0 .25em"
               autosize placeholder=""
               :value="valueModel.amount"
               @update:value="x => valueModel.amount = safeParseInt(x)"
      />
    </div>
    <div style="margin-top: .25em">
      <div class="label">
        ScriptPubKey
        <div id="icon-group">
          <SelectableIcon @click="showModal.scriptPubKeyHelper = true">
            <CreateIcon/>
          </SelectableIcon>
          <SelectableIcon @click="showModal.scriptPubKeyInfo = true">
            <InfoIcon/>
          </SelectableIcon>
        </div>
      </div>
      <n-input type="textarea" size="small" placeholder=""
               v-model:value="valueModel.scriptPubKey"
      />
    </div>
  </Frame>
</template>

<style scoped>
.label {
  display: inline-flex;
  align-items: center;

  #icon-group {
    margin: 0 .5em;
    display: inline-flex;
    align-items: center;
    gap: .25em;
  }
}
</style>
