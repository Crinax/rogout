<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'

const window = getCurrentWindow()

const restartSystem = () => {
  invoke('system_restart')
}

const suspendSystem = async () => {
  await invoke('system_suspend');

  window.close()
}

const shutdownSystem = () => {
  invoke('system_shutdown')
}

const closeWindow = () => {
  window.close()
}
</script>

<template>
  <div class="rogout-container">
    <button class="rogout__button" @click="restartSystem()">
      <i class="pi pi-refresh" />
      <p>Restart</p>
    </button>

    <button class="rogout__button" @click="suspendSystem()">
      <i class="pi pi-eye-slash" />
      <p>Suspend</p>
    </button>

    <button class="rogout__button" @click="shutdownSystem()">
      <i class="pi pi-power-off" />
      <p>Shutdown</p>
    </button>

    <button class="rogout__button" @click="closeWindow()">
      <i class="pi pi-sign-out" />
      <p>Exit</p>
    </button>
  </div>
</template>
