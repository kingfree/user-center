<template lang="pug">
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
const message = useMessage()

const form = reactive({
  name: '',
  password: ''
})


async function doLogin() {
  try {
    const { data } = await request.post('/login', form)
    console.log(data)
    window.localStorage.setItem('token', data.token)
    message.success(`Welcome, ${data.name}!`)
    window.location.href = '/'
  } catch (e) {
    console.warn(e)
    message.error("Login failed!")
  }
}
</script>
<style>
</style>
