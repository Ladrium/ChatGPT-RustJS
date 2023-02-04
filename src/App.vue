<script setup lang="ts">
import Titlebar from "./components/Titlebar.vue";
import Sidebar from "./components/Sidebar.vue";
import Chat from "./components/Chat.vue";
</script>

<template>
  <div>
    <Titlebar/>
    <Sidebar/>
    <Chat/>
  </div>
</template>

<style scoped>
  div {
    display: grid;
  }
</style>
