<script setup lang="ts">
import {useRouter} from "vue-router";
import {routes} from "../router.ts";

let router = useRouter();
</script>

<template>
  <n-ul>
    <n-li v-for="r in [...routes].splice(1)">
      <n-a @click="router.push(r.path)">{{ r.name }}</n-a>
    </n-li>
  </n-ul>
</template>

<style scoped>

</style>
