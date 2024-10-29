<script setup lang="ts">
import ScriptPubKeyHelper from "./ScriptPubKeyHelper.vue";

let model = defineModel('show');
let emit = defineEmits(['result']);
</script>

<template>
  <n-modal v-model:show="model"
  >
    <n-card
        style="width: 600px"
        title="From Address"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <ScriptPubKeyHelper
          @done="model = false"
          @result="x => emit('result', x)"/>
    </n-card>
  </n-modal>
</template>

<style scoped>

</style>
