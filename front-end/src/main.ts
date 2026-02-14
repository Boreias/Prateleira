import './presentation/assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'
import router from './presentation/router'

const app = createApp(App)

app.use(router)

app.mount('#app')
