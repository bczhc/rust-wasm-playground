<script setup lang="ts">
import {useRouter} from "vue-router";
import {routes} from "../router.ts";
let router = useRouter();
</script>

<template>
  <n-ul>
    <n-li v-for="r in routes.splice(1)">
      <n-a @click="router.push(r.path)">{{
          (() => {
            let filePath = r.component['__file'];
            if (!filePath) return 'Unknown';
            let split = filePath.split('/');
            let basename = split[split.length - 1];
            let i = basename.lastIndexOf('.vue');
            return basename.substring(0, i);
          })()
        }}</n-a>
    </n-li>
  </n-ul>
</template>

<style scoped>

</style>
