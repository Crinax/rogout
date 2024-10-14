import 'primeicons/primeicons.css'
import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'

import { getCurrentWindow } from '@tauri-apps/api/window';

const window = getCurrentWindow()

document.onload = () => {
  window.setFocus();
}

document.addEventListener('keyup', event => {
  if (event.key === 'Escape') {
    window.close()
  }
})

createApp(App).mount('#rogout-root')
