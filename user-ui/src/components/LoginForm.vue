<template lang="pug">
n-button(@click="fetchList") user list
n-card(title="Login")
  n-form(:model="form")
    n-form-item(label="Name" path="name")
      n-input(v-model:value="form.name")
    n-form-item(label="Password" path="password")
      n-input(v-model:value="form.password" type="password")
  template(#footer)
    n-text root/root  user/password
  template(#action)
    n-button(@click="doLogin" type="primary") Login
</template>
<script setup>
import { reactive, onMounted } from 'vue'
import request from '../utils/request'
import { useMessage } from 'naive-ui'

const form = reactive({
  name: '',
  password: ''
})

async function fetchList() {
  const { data } = await request.get('/user')
  console.log(data)
}

const message = useMessage()

async function doLogin() {
  const { data } = await request.post('/login', form)
  console.log(data)
  window.localStorage.setItem('token', data.token)
  message.success(`Welcome, ${data.name}`)
}
</script>
<style>
</style>
