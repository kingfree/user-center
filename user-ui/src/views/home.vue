<template lang="pug">
n-page-header(title="Users" subtitle="Which have less permissions")
  template(#extra)
    n-space(align="center")
      n-gradient-text(type="info") Welcome, {{ current_user.name }} !
      n-button(@click="doLogout" size="small" secondary) Logout
div(style="margin: 10px 0")
  n-button(@click="createUser" type="primary") Create
n-data-table(:columns="columns" :data="user.list" @update:sorter="handleSorterChange")
n-modal(v-model:show="showModal")
  n-card(:title="title" style="max-width: 500px;")
    n-form(:model="form" :rules="rules")
      n-form-item(label="Name" path="name")
        n-input(v-model:value="form.name")
      n-form-item(label="Password" path="password")
        n-input(v-model:value="form.password" type="password" placeholder="Leave blank to keep the same")
      n-form-item(label="Description" path="description")
        n-input(v-model:value="form.description" type="textarea")
      n-form-item(label="Group" path="group_id")
        n-select(v-model:value="form.group_id" :options="groups")
    template(#action)
      n-space(align="center" justify="end")
        n-button(@click="doCancel") Cancel
        n-button(@click="doSave" type="primary") Save
</template>
<script setup>
import { ref, reactive, onBeforeMount, onMounted, h, computed } from 'vue'
import request from '../utils/request'
import jwt_decode from 'jwt-decode'
import { NButton, NPopconfirm, NSpace } from 'naive-ui'
import { useMessage } from 'naive-ui'
const message = useMessage()

const s = {
  sortOrder: false,
  sorter: true
}
const createColumns = () => {
  return [
    { title: "ID", key: 'id', ...s },
    { title: "Name", key: 'name', ...s },
    { title: "Description", key: 'description' },
    { title: "Group", key: 'group_name' },
    { title: "Level", key: 'level' },
    {
      title: "Action", key: 'actions', render(row) {
        return h(NSpace, {}, {
          default: () => [
            h(NButton, {
              size: 'small',
              onClick: () => editUser(row),
            }, 'Edit'),
            h(
              NPopconfirm,
              {
                onPositiveClick: () => deleteUser(row),
              },
              {
                default: () => 'Are you sure?',
                trigger: () => h(NButton, {
                  size: 'small',
                  type: 'error',
                }, 'Delete'),
              }
            )
          ]
        })
      }
    }
  ]
}
const columns = ref(createColumns())
const params = reactive({ sort: 'name', asc: true })
const user = reactive({ list: [] });

onMounted(() => fetchList())
async function fetchList() {
  const { data } = await request.get('/user', { params })
  user.list = data;
}
async function deleteUser(row) {
  console.log(row);
  await request.delete('/user/' + row.id)
  message.success('User deleted')
  await fetchList()
}

const showModal = ref(false);
const EMPTY = {
  id: null,
  name: '',
  password: '',
  description: '',
  group_id: null,
};
const form = reactive({ ...EMPTY });
const title = computed(() => {
  return form.id ? 'Edit User' : 'Create User'
})
async function editUser(row) {
  const { data } = await request.get('/user/' + row.id);
  Object.assign(form, data);
  showModal.value = true;
}
function createUser() {
  Object.assign(form, { ...EMPTY });
  showModal.value = true;
}
function doCancel() {
  showModal.value = false;
}
const rules = {
  name: { required: true, },
  group_id: { required: true, },
}
async function doSave() {
  try {
    if (form.id) {
      await request.put('/user/' + form.id, form)
    } else {
      let data = { ...form }
      delete data.id
      await request.post('/user', data)
    }
    await fetchList()
  } catch (e) {
    console.warn(e)
    message.error('Save failed')
  }
  showModal.value = false;
}

const groups = ref([]);
async function fetchGroups() {
  const { data } = await request.get('/group')
  groups.value = data.map(item => ({
    label: item.name + ' (' + item.id + ')',
    value: item.id,
  }));
}
onBeforeMount(() => fetchGroups())

async function handleSorterChange(sorter) {
  console.log(sorter)
  params.sort = sorter.columnKey
  params.asc = sorter.order !== 'descend'
  await fetchList()
  columns.value.forEach(column => {
    if (column.key === sorter.columnKey) {
      column.sortOrder = sorter.order
    } else {
      column.sortOrder = false
    }
  })
}

const current_user = reactive({ id: 0, name: '', level: -1 })
onBeforeMount(() => checkLogin())
function checkLogin() {
  try {
    let decoded = jwt_decode(window.localStorage.getItem('token'))
    console.log(decoded)
    current_user.id = decoded.id
    current_user.name = decoded.name
    current_user.level = decoded.level
    return true
  } catch (e) {
    console.warn(e)
    window.localStorage.removeItem('token')
    window.location.reload()
    return false
  }
}

function doLogout() {
  window.localStorage.removeItem('token')
  window.location.reload()
}
</script>
<style>
</style>
