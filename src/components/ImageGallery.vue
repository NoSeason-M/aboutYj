<template>
  <div class="image-gallery">
    <div v-for="(count, dir) in config" :key="dir" class="folder-group">
      <div class="folder-title">📂 {{ dir }}</div>
      <div class="image-grid">
        <div
          class="image-card"
          v-for="i in count"
          :key="i"
          @click="handleImageClick(dir, i)"
        >
          <img
            class="image-cover"
            :src="getImageUrl(dir, i)"
            :alt="`${dir}${i}`"
          >
          <div class="image-info">{{ dir }}{{ i }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

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
  config: {
    type: Object,
    required: true
  },
  imageExt: {
    type: String,
    default: 'jpg'
  }
})

const emit = defineEmits(['image-click'])

// 生成图片URL
const getImageUrl = (dir, i) => {
  const baseUrl = `http://${props.minioIp}:9000/${props.bucketName}/${props.rootDir}/`
  return `${baseUrl}${dir}/${i}.${props.imageExt}`
}

// 点击图片
const handleImageClick = (dir, i) => {
  emit('image-click', getImageUrl(dir, i))
}
</script>

<style scoped>
.folder-group {
  margin-bottom: 40px;
  width: 100%;
  height: auto;
}

.folder-title {
  background: #fff;
  padding: 12px 15px;
  border-radius: 8px;
  margin-bottom: 15px;
  font-size: 16px;
  font-weight: bold;
  border-left: 4px solid #409eff;
  color: #333;
  width: 100%;
}

.image-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  gap: 18px;
  width: 100%;
  height: auto;
}

.image-card {
  background: #fff;
  border-radius: 10px;
  overflow: hidden;
  cursor: pointer;
  transition: 0.3s;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  height: auto;
}

.image-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.1);
}

.image-cover {
  width: 100%;
  height: 160px;
  object-fit: contain;
  background: #f5f5f5;
  display: block;
}

.image-info {
  padding: 12px;
}

.image-name {
  font-size: 14px;
  color: #333;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>