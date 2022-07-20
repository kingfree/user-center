import axios from 'axios'

const service = axios.create({
    baseURL: '/api',
    withCredentials: true,
    timeout: 1000 * 60
})

service.defaults.headers.common['Content-Type'] = 'application/json';

service.interceptors.request.use(
    config => {
        let token = window.localStorage.getItem('token')
        if (token) {
            config.headers['Authorization'] = 'Bearer ' + token
        }
        return config
    },
    error => {
        return Promise.reject(error)
    }
)

export default service;
