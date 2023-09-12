<script setup lang="ts">
import { useRouter } from "vue-router";
const router = useRouter();
</script>

<template>
  <div class="home-page">
    <button @click="router.push('/we-read-v1')">微信读书1.0</button>
    <button @click="router.push('/we-read-v2')">微信读书2.0</button>
    <button @click="router.push('/snipaste')">Snipaste</button>
  </div>
</template>

<style scoped>
.home-page {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 10px;
}

button {
  width: 100px;
  height: 20px;
}
</style>
