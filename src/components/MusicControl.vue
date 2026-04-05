<template>
  <div class="music-control">
    <select class="music-select" v-model="currentMusicIndex" @change="handleMusicChange">
      <option v-for="(item, index) in musicList" :key="index" :value="index">
        {{ item.name }}
      </option>
    </select>
    <button class="btn music-btn" @click="toggleMusic">
      {{ isPlaying ? '暂停' : '点击播放' }}
    </button>

    <div class="volume-control">
      <span class="volume-text">音量</span>
      <input
        type="range"
        class="volume-slider"
        v-model="volume"
        min="0"
        max="100"
        @input="handleVolumeChange"
      >
    </div>

    <!-- 播放模式选择 -->
    <select v-model="playMode" class="play-mode">
      <option value="single">单曲循环</option>
      <option value="list">列表循环</option>
    </select>

    <span class="music-tip">🎵 音乐：picture-yj/yj/ 目录 | 无损播放</span>

    <audio ref="audioRef" @ended="handleMusicEnd"></audio>
  </div>
</template>

<script setup>
import { ref, watch, onMounted } from 'vue'

// 接收父组件传递的配置
const props = defineProps({
  minioIp: {
    type: String,
    required: true
  },
  bucketName: {
    type: String,
    required: true
  },
  rootDir: {
    type: String,
    required: true
  },
  musicList: {
    type: Array,
    required: true
  }
})

// 响应式数据
const audioRef = ref(null)
const currentMusicIndex = ref(0)
const isPlaying = ref(false)
const volume = ref(50)
const playMode = ref('single')

// 计算基础URL
const baseUrl = ref('')
onMounted(() => {
  baseUrl.value = `http://${props.minioIp}:9000/${props.bucketName}/${props.rootDir}/`
  // 初始化加载第一首音乐
  loadMusic(0)
  // 设置初始音量
  if (audioRef.value) {
    audioRef.value.volume = volume.value / 100
  }
})

// 加载音乐
const loadMusic = (index) => {
  if (props.musicList.length === 0) return
  const file = props.musicList[index].file
  audioRef.value.src = baseUrl.value + file
}

// 播放/暂停切换
const toggleMusic = () => {
  if (audioRef.value.paused) {
    audioRef.value.play().then(() => {
      isPlaying.value = true
    }).catch(err => {
      alert("播放失败：检查音乐是否存在\n" + err.message)
    })
  } else {
    audioRef.value.pause()
    isPlaying.value = false
  }
}

// 音量改变
const handleVolumeChange = () => {
  audioRef.value.volume = volume.value / 100
}

// 切换音乐
const handleMusicChange = () => {
  loadMusic(currentMusicIndex.value)
  audioRef.value.play().catch(console.warn)
  isPlaying.value = true
}

// 播放结束处理
const handleMusicEnd = () => {
  if (playMode.value === 'list') {
    // 列表循环：播放下一首
    let nextIdx = Number(currentMusicIndex.value) + 1
    if (nextIdx >= props.musicList.length) nextIdx = 0
    currentMusicIndex.value = nextIdx
    loadMusic(nextIdx)
    audioRef.value.play().catch(console.warn)
    isPlaying.value = true
  } else {
    // 单曲循环：重新播放当前
    audioRef.value.play()
  }
}

// 监听播放状态（防止手动操作audio标签）
watch(() => audioRef.value?.paused, (newVal) => {
  isPlaying.value = !newVal
})
</script>

<style scoped>
.music-control {
  background: #fff;
  padding: 15px 20px;
  border-radius: 8px;
  margin: 0 auto 30px;
  max-width: 850px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 15px;
  flex-wrap: wrap;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.music-select {
  padding: 8px 12px;
  border: 1px solid #e6e6e6;
  border-radius: 6px;
  font-size: 14px;
  color: #333;
  outline: none;
  cursor: pointer;
  min-width: 180px;
}

.music-select:focus {
  border-color: #409eff;
}

.play-mode {
  padding: 8px 12px;
  border: 1px solid #e6e6e6;
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  outline: none;
}

.volume-control {
  display: flex;
  align-items: center;
  gap: 8px;
}

.volume-text {
  font-size: 14px;
  color: #666;
  white-space: nowrap;
}

.volume-slider {
  width: 120px;
  height: 6px;
  cursor: pointer;
  accent-color: #409eff;
}

.music-tip {
  font-size: 12px;
  color: #999;
  white-space: nowrap;
}
</style>