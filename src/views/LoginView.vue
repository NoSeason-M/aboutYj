<template>
  <div class="login-page">
    <div class="login-container">
      <h1 class="login-title">专属记录登录</h1>
      <div class="input-group">
        <label class="input-label">账号</label>
        <input
          type="text"
          class="input-box"
          v-model="username"
          placeholder="请输入账号"
        >
      </div>
      <div class="input-group">
        <label class="input-label">密码</label>
        <input
          type="password"
          class="input-box"
          v-model="password"
          placeholder="请输入密码"
        >
      </div>
      <button class="btn login-btn" @click="handleLogin">登录</button>
      <div class="error-tip" v-show="showError">账号或密码错误，请重试</div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const username = ref('')
const password = ref('')
const showError = ref(false)
let errorTimer = null

// 登录验证逻辑
const handleLogin = () => {
  const trimUsername = username.value.trim()
  const trimPassword = password.value.trim()

  if (trimUsername === 'yj' && trimPassword === '20251001') {
    // 验证通过，跳转到目标页面
    router.push('/yj-basic')
  } else {
    // 验证失败，显示错误提示
    showError.value = true
    // 清除之前的定时器
    if (errorTimer) clearTimeout(errorTimer)
    // 3秒后隐藏错误提示
    errorTimer = setTimeout(() => {
      showError.value = false
    }, 3000)
  }
}

// 支持回车登录
onMounted(() => {
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') {
      handleLogin()
    }
  })
})
</script>

<style scoped>
.login-page {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
}

.login-container {
  background: #fff;
  padding: 40px;
  border-radius: 10px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  width: 100%;
  max-width: 420px;
}

.login-title {
  text-align: center;
  margin-bottom: 30px;
  color: #333;
  font-size: 24px;
  font-weight: bold;
}

.input-group {
  margin-bottom: 20px;
}

.input-label {
  display: block;
  margin-bottom: 8px;
  font-size: 14px;
  color: #666;
}

.login-btn {
  width: 100%;
  padding: 12px 0;
  font-size: 16px;
  margin-top: 10px;
}

.error-tip {
  color: #f56c6c;
  font-size: 12px;
  text-align: center;
  margin-top: 10px;
}
</style>