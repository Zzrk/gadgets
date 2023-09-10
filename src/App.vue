<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue';

const wereadRef = ref<HTMLIFrameElement>();

const handleClick = () => {
  invoke("mousemove")
}
</script>

<template>
  <div class="container">
    <div class="menu-bar">
      <button @click="handleClick">一键启动</button>
    </div>
    <div class="content">
      <iframe ref="wereadRef" src="https://weread.qq.com/" frameborder="0"></iframe>
    </div>
  </div>
</template>

<style scoped>
.container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.content {
  flex: 1;
}

iframe {
  width: 100%;
  height: 100%;
}
</style>
