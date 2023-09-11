<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { register, unregister } from '@tauri-apps/api/globalShortcut';
import { onMounted, onUnmounted, ref } from 'vue';

const wereadRef = ref<HTMLIFrameElement>();
// 当前是否处于运行状态
const isWorking = ref(false);

// 任务函数
const work = () => {
  invoke("mousemove");
  invoke("mousescroll");
  setTimeout(() => {
    if (!isWorking.value) return;
    work();
  }, 2000);
}

// 启动任务
const handleStart = () => {
  isWorking.value = true;
  work();
}

// 停止任务
const handleStop = () => {
  isWorking.value = false;
}

// 停止任务快捷键的注册和删除
onMounted(() => {
  register('CommandOrControl+Shift+C', handleStop)
})
onUnmounted(() => {
  unregister('CommandOrControl+Shift+C')
})
</script>

<template>
  <div class="container">
    <div class="menu-bar">
      <div>
        <button @click="handleStart">启动</button>
        <button @click="handleStop">停止</button>
      </div>
      <span>{{ isWorking ? '运行中' : '已停止' }}</span>
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

.menu-bar {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
}

.content {
  flex: 1;
}

iframe {
  width: 100%;
  height: 100%;
}
</style>
