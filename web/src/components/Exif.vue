<script setup lang="ts">
import {useWasm} from "../lib.ts";
import {ref} from "vue";

const Exif = useWasm().Exif;

let gps = ref<Float64Array | null>();
let fileInput = ref<HTMLElement>();

function inputOnChange() {
  let reader = new FileReader();
  reader.onload = function () {
    gps.value = null;
    try {
      let arrayBuffer = this.result as ArrayBuffer;
      let array = new Uint8Array(arrayBuffer);
      let exif = Exif.read(array);
      onRead(exif);
    } catch (e) {
      console.error(e);
      alert(`${e}`);
    }
  }

  let first = ((fileInput.value!! as any)['files'] as Blob[])[0];
  reader.readAsArrayBuffer(first);
}

function onRead(addr: number) {
  gps.value = Exif.gps(addr);

  Exif.free(addr);
}

</script>

<template>
  <input type="file" @change="inputOnChange" ref="fileInput"/>

  <p>
    <span v-if="gps">GPS: </span>
    <span v-if="gps">{{ gps }}</span>
  </p>
</template>

<style scoped>

</style>
