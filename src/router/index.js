import { createRouter, createWebHistory } from 'vue-router'
import LoginView from '@/views/LoginView.vue'
import YjBasicView from '@/views/YjBasicView.vue'

const routes = [
  {
    path: '/',
    name: 'login',
    component: LoginView
  },
  {
    path: '/yj-basic',
    name: 'yjBasic',
    component: YjBasicView
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router