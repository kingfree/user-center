import { createApp } from 'vue'
import App from './App.vue'
import naive from 'naive-ui'
import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
    history: createWebHistory(),
    routes: [
        //...
    ],
})

const app = createApp(App)
app.use(naive)
app.use(router)
app.mount('#app')
