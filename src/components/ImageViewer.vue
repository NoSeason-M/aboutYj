<template>
  <div
    class="viewer-modal"
    v-show="visible"
    @click="handleClose"
  >
    <span
      class="close-btn"
      @click.stop="handleClose"
    >×</span>
    <img class="view-image" :src="imageUrl" alt="查看图片">
  </div>
</template>

<script setup>
const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  },
  imageUrl: {
    type: String,
    default: ''
  }
})

const emit = defineEmits(['close'])

const handleClose = () => {
  emit('close')
}
</script>

<style scoped>
.viewer-modal {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 1);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 99999;
  cursor: pointer;
}

.view-image {
  max-width: 90vw;
  max-height: 90vh;
  object-fit: contain;
  outline: none;
}

.close-btn {
  position: absolute;
  top: 20px;
  right: 30px;
  color: #fff;
  font-size: 36px;
  z-index: 100000;
  display: block;
  cursor: pointer;
}
</style>