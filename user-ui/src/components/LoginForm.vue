<template lang="pug">
n-button(@click="fetchList") user list
pre {{code}}
n-card(title="Login")
  template(#footer)
    | root/root  user/password
  template(#action)
    n-button Login
</template>
<script setup>
import { ref, onMounted } from 'vue'
import request from '../utils/request'

const code = ref('')

async function fetchList() {
  const data = await request.get('/user')
  console.log(data)
}
</script>
<style>
</style>
