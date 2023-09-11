<script setup lang="ts">
import { WebviewWindow, PhysicalPosition } from '@tauri-apps/api/window';
import { onMounted, onUnmounted, ref } from 'vue';

// 当前是否处于运行状态
const isWorking = ref(false);

let webview: WebviewWindow;

// 停止任务快捷键的注册和删除
onMounted(async () => {
  const mainWindow = WebviewWindow.getByLabel('main');
  if (mainWindow) {
    const outerPosition = await mainWindow.outerPosition();
    const innerPosition = await mainWindow.innerPosition();
    const innerSize = await mainWindow.innerSize();
    webview = new WebviewWindow('weread', {
      url: 'https://weread.qq.com/',
      decorations: false,
      alwaysOnTop: true,
      x: innerPosition.x,
      y: innerPosition.y + 20,
      width: innerSize.width,
      height: innerSize.height - 20
    });
    mainWindow.onMoved(({ payload }) => {
      const { x, y } = payload;
      webview.setPosition(new PhysicalPosition(x + innerPosition.x - outerPosition.x, y + innerPosition.y - outerPosition.y + 20))
    });
  }
})

onUnmounted(() => {
  webview.close();
})

// 启动任务
const handleStart = () => {
  webview.setFocus();
}

// 停止任务
// const handleStop = () => {
//   isWorking.value = false;
// }
</script>

<template>
  <div class="container">
    <div class="menu-bar">
      <div>
        <button @click="handleStart">启动</button>
        <!-- <button @click="handleStop">停止</button> -->
      </div>
      <span>{{ isWorking ? '运行中' : '已停止' }}</span>
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
  height: 20px;
}

.content {
  flex: 1;
}

iframe {
  width: 100%;
  height: 100%;
}
</style>
