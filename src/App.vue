<script setup lang="ts">
import Bar from "./components/bar/index.vue"

if (import.meta.env.PROD) {
  window.addEventListener("contextmenu", e => e.preventDefault(), true);
}

</script>

<template>
  <div class="app">
    <Bar/>
    <RouterView/>
  </div>
</template>

<style scoped>
.app{
  height: 100vh;
}

</style>
<style>
body{
  margin: 0;
}
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: rgb(59, 64, 84);


  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

</style>