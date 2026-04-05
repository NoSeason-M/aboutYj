<template>
  <div class="yj-basic-page">
    <h1 class="main-title">yj--我想以此来记录我认识的喻婕</h1>

    <!-- 音乐控制组件 -->
    <MusicControl
      :minioIp="minioIp"
      :bucketName="bucketName"
      :rootDir="rootDir"
      :musicList="musicList"
    />

    <!-- 图片画廊组件 -->
    <ImageGallery
      :minioIp="minioIp"
      :bucketName="bucketName"
      :rootDir="rootDir"
      :config="imageConfig"
      @image-click="handleImageClick"
    />

    <!-- 图片查看器组件 -->
    <ImageViewer
      :visible="viewerVisible"
      :imageUrl="currentImageUrl"
      @close="handleViewerClose"
    />
  </div>
</template>

<script setup>
import { ref } from 'vue'
import MusicControl from '@/components/MusicControl.vue'
import ImageGallery from '@/components/ImageGallery.vue'
import ImageViewer from '@/components/ImageViewer.vue'

// 配置项（和原HTML一致）
const minioIp = ref("10.14.141.204")
const bucketName = ref("picture-yj")
const rootDir = ref("yj")

// 音乐列表
const musicList = ref([
  { name: "春娇与志明", file: "1.mp3" },
  { name: "the way i still love you", file: "2.mp3" },
])

// 图片配置
const imageConfig = ref({
  "25.国庆，是我们重相识的一天，我很感谢直播，和张宇辰，不然我觉得我不会再和你有所交集，这一刻我觉得我很幸运":1,
  "你很想去南京，是我对不起你，导致你没有考到理想的地方，这次转本要加油，一起加油":1,
  "25.10.3  第一次一起看的电影（猪猪侠），感觉有点小孩子的电影，可惜票找不到了": 1,
  "其实我很后悔，因为太想见你，当时的我还不是很优秀，很好看就去见你了，非常后悔":0,
  "如果我要是在我状态很好，优秀的时候去见你，你是不是就不会突然这样了": 0,
  "是我太心急了，好后悔":0,
  "25.11.7  你在抖音发了一张伤心的动图，我知道了每当天气不好 下雨的时候，你的心情会非常不好":1,
  "25.11.15---25.11.18  去重庆比赛，拍了些重庆的照片给你看，感觉重庆的夜景很美，希望有机会能一起去": 5,
  "25.11.16   和你打了视频，带你看了看重庆，很高兴，但是可惜你要乐跑":0,
  "25.11.18  在重庆买了些吃的，听说你最近睡眠不好，偶然听见店员说这个花茶对睡眠好，给你买了一些": 4,
  "25.11月------  为了能和你说上话，每天给你拍了些风景😂，哎，是有点傻傻的，但是确实想你":3,
  "26.1.2    爬上了武功山，然后分享给了你，其实山顶没有爬上去🫠🙃，太冷了  然后在抖音上找了个山顶的图片，忘记截图了，把抖音露出来了，后面才发现，在你面前真的太丢人了🙃 我服了":6,
  "图":30,
})

// 图片查看器状态
const viewerVisible = ref(false)
const currentImageUrl = ref('')

// 点击图片
const handleImageClick = (url) => {
  currentImageUrl.value = url
  viewerVisible.value = true
}

// 关闭图片查看器
const handleViewerClose = () => {
  viewerVisible.value = false
  currentImageUrl.value = ''
}
</script>

<style scoped>
.yj-basic-page {
  height: auto !important;
}

.main-title {
  text-align: center;
  margin: 30px 0;
  color: #333;
  font-size: 24px;
}
</style>