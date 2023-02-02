<script setup lang="ts">
import Sidebar from "./components/Sidebar.vue";
import Chat from "./components/Chat.vue";
</script>

<template>
  <div>
    <Sidebar/>
    <Chat/>
  </div>
</template>

<style scoped>
  div {
    display: grid;
  }
</style>
