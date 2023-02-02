<script setup lang="ts">
</script>

<template>
  <div class="sidebar container">
    <h2>Chat History</h2>
    <button>+ New Chat</button>
  </div>
</template>
<style scoped>
.sidebar {
  padding: .5rem 0 2rem 0;
  float: left;
  width: 14%;
  background: #202123;
  height: 100vh;
}

button {
  font-family: "Inter", serif;
  width: 90%;
  text-align: left;
  justify-self: center;
  height: 10%;
  color: white;
  border: solid 1px rgba(255, 255, 255, 0.4);
  border-radius: .5rem;
  background: inherit;
}
</style>
