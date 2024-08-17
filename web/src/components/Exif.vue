<script setup lang="ts">
import {useWasm} from "../lib.ts";
import {ref} from "vue";

const Exif = useWasm().Exif;

let gps = ref<Float64Array | null>();

window.onload = () => {
  document.querySelector('input')!!.addEventListener('change', function () {

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
    reader.readAsArrayBuffer(this.files!![0]);
  }, false);
};

function onRead(addr: number) {
  gps.value = Exif.gps(addr);

  Exif.free(addr);
}

</script>

<template>
  <input type="file"/>

  <p>
    <span v-if="gps">GPS: </span>
    <span v-if="gps">{{ gps }}</span>
  </p>
</template>

<style scoped>

</style>
