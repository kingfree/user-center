import { createApp } from 'vue'
import App from './App.vue'
import naive from 'naive-ui'
import { createRouter, createWebHistory } from 'vue-router'

const whiteList = ['/login']
const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            name: 'home',
            component: () => import('./views/home.vue')
        },
        {
            path: '/login',
            name: 'login',
            meta: { layout: 'login' },
            component: () => import('./views/login.vue')
        }
    ],
})
router.beforeEach((to, from, next) => {
    const hasToken = window.localStorage.getItem('token')
    if (hasToken && to.path === '/login') {
        next({ path: (to.query || {}).redirect || '/' })
        return
    } else if (!hasToken && whiteList.indexOf(to.path) < 0) {
        next(`/login?redirect=${to.path}`)
        return
    }
    next()
})

const app = createApp(App)
app.use(naive)
app.use(router)
app.mount('#app')
