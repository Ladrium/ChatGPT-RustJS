<template>
  <div class="title-bar">
    <img src="../assets/ChatGPTIcon.png" alt="ChatGPT" class="title-bar-icon" data-tauri-drag-region>
    <div class="title-bar-text">
      <p data-tauri-drag-region>ChatGPT</p>
    </div>
    <div class="title-bar-button" @click="ontop()">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32">
        <title>Always On Top</title>
        <path
            d="M30 13.66a2.12 2.12 0 0 1-2 1.3h-2.47L19 21.45V24a2.12 2.12 0 0 1-2.12 2.12 2.09 2.09 0 0 1-1.5-.62l-3.75-3.75-7 7a1 1 0 0 1-1.42 0 1 1 0 0 1 0-1.42l7-7-3.7-3.75A2.12 2.12 0 0 1 8 13h2.54L17 6.47V3.94a2.11 2.11 0 0 1 3.61-1.5l8.91 8.91a2.11 2.11 0 0 1 .48 2.31Z"/>
      </svg>
    </div>
    <div class="title-bar-button" id="title-bar-minimize" @click="minimize()">
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
        <title>Minimize</title>
        <path fill="none" d="M0 0h24v24H0V0z"/>
        <path d="M7 19h10c.55 0 1 .45 1 1s-.45 1-1 1H7c-.55 0-1-.45-1-1s.45-1 1-1z"/>
      </svg>
    </div>
    <div class="title-bar-button" @click="closeWindow()">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
        <title>Close</title>
        <path
            d="M19.8534546,19.1465454L12.7069092,12l7.1465454-7.1465454c0.1871948-0.1937256,0.1871948-0.5009155,0-0.6947021c-0.1918335-0.1986084-0.5083618-0.2041016-0.7069702-0.0122681l-7.1465454,7.1465454L4.8534546,4.1465454c-0.1937256-0.1871338-0.5009155-0.1871338-0.6947021,0C3.960144,4.3383789,3.9546509,4.6549072,4.1464844,4.8535156L11.2929688,12l-7.1464844,7.1464844c-0.09375,0.09375-0.1464233,0.2208862-0.1464233,0.3534546C4,19.776062,4.223877,19.999939,4.5,20c0.1326294,0.0001221,0.2598267-0.0526123,0.3534546-0.1465454l7.1464844-7.1464844l7.1465454,7.1465454C19.2401123,19.9474487,19.3673706,20.0001831,19.5,20c0.1325073-0.000061,0.2595825-0.0526733,0.3533325-0.1463623C20.048645,19.6583862,20.0487061,19.3417969,19.8534546,19.1465454z"/>
      </svg>
    </div>
  </div>
</template>

<script>
import {appWindow} from "@tauri-apps/api/window";

export default {
  name: "Titlebar",
  data() {
    return {
      isOntop: false
    }
  },
  methods: {
    closeWindow() {
      appWindow.close();
    },
    minimize() {
      appWindow.minimize();
    },
    ontop() {
      appWindow.setAlwaysOnTop(!this.isOntop);
      this.isOntop = !this.isOntop;
    }
  }
}
</script>

<style scoped>
.title-bar {
  height: 25px;
  background: white;
  align-items: center;
  user-select: none;
  display: flex;
  width: 100%;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}

.title-bar-text {
  flex-grow: 1;
  color: black;
  text-align: center;
}

.title-bar-icon {
  margin-left: 2px;
  max-width: 20px;
  max-height: 20px;
}

.title-bar-button {
  display: inline-flex;
  justify-content: center;
  padding-left: .1rem;
  align-items: center;
  width: 25px;
  height: 25px;
}

.title-bar-button:hover {
  background: black;
  fill: white;
}
</style>
