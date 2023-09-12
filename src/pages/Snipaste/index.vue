<script setup lang="ts">
import { WebviewWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';

const handlePickColor = async () => {
  invoke("screen_capture");
  // TODO: 有延迟
  new WebviewWindow('pick-color', {
    url: 'screenshot.html',
    fullscreen: true
  });
}
</script>

<template>
  <div class="snipaste-page">
    <button @click="handlePickColor">取色</button>
  </div>
</template>

<style scoped>
.snipaste-page {
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
