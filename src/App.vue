<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted } from "vue";
import { useConfigStore } from "./store/configStore";
import { Config } from "./utils/type";

const configStore = useConfigStore();

if (import.meta.env.PROD) {
    window.addEventListener("contextmenu", (e) => e.preventDefault(), true);
}

async function init_configstroe() {
    const data = (await invoke("get_config")) as Config;
    configStore.set_config(data);
}

onMounted(async () => {
    await init_configstroe();
});
</script>

<template>
    <div class="app">
        <!-- <Bar/> -->
        <div class="view">
            <RouterView />
        </div>
    </div>
</template>

<style scoped>
.app {
    height: 100vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
}
.view {
    flex: 1;
}
</style>
<style>
body {
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
