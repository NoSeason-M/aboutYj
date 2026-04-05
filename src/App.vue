<template>
  <router-view />
</template>

<script setup>
import { onMounted } from 'vue'
import { check } from '@tauri-apps/plugin-updater'
// 2.x 正确的 relaunch 导入路径
import { relaunch } from '@tauri-apps/plugin-process'

onMounted(async () => {
  try {
    const update = await check()
    if (update) {
      console.log(`发现新版本：${update.version}，正在安装...`)
      await update.downloadAndInstall()
      await relaunch()
    }
  } catch (error) {
    console.error('自动更新失败：', error)
  }
})
</script>

<style scoped>
/* 根组件无额外样式 */
</style>