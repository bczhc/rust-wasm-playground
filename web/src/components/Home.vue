<script setup lang="ts">
import {useRouter} from "vue-router";

let router = useRouter();
</script>

<template>
  <n-ul>
    <n-li><n-a @click="router.push('/argon2')">Argon2</n-a></n-li>
    <n-li><n-a @click="router.push('/exif')">Exif</n-a></n-li>
    <n-li><n-a @click="router.push('/ipv6-xor')">Ipv6Xor</n-a></n-li>
  </n-ul>
</template>

<style scoped>

</style>
