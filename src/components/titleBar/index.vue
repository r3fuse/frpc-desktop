<script setup lang="ts">
import { Window } from '@tauri-apps/api/window';
import { onMounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/core';

// const windowName = getCurrentWindow();
// const appWindow = new Window(windowName.label);

// function minimizeWindow() {
//     appWindow.minimize();
// }

// function maximizeWindow() {
//     appWindow.toggleMaximize();
// }

// function closeWindow() {
//     appWindow.close();
//     invoke("stop_frp");
// }
onMounted(() => {
    const windowName = getCurrentWindow();
    console.log("titleBar appName", windowName.label);
    const appWindow = new Window(windowName.label);

    document
        .getElementById("titlebar-minimize")
        ?.addEventListener("click", () => appWindow.minimize());
    document
        .getElementById("titlebar-maximize")
        ?.addEventListener("click", () => appWindow.toggleMaximize());
    document.getElementById("titlebar-close")?.addEventListener("click", () => {
        invoke("stop_frp");
        appWindow.close();
    });
});
</script>

<template>
    <div data-tauri-drag-region class="titleBar">
        <div class="title">
            frpc-desktop
        </div>
        <div class="controllWindow">
            <div class="titlebar-button resize" id="titlebar-minimize">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="1em"
                    height="1em"
                    viewBox="0 0 24 24"
                >
                    <path fill="currentColor" d="M20 14H4v-4h16" />
                </svg>
            </div>
            <div class="titlebar-button resize" id="titlebar-maximize">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="1em"
                    height="1em"
                    viewBox="0 0 24 24"
                >
                    <path fill="currentColor" d="M4 4h16v16H4zm2 4v10h12V8z" />
                </svg>
            </div>
            <div class="titlebar-button" id="titlebar-close">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="1em"
                    height="1em"
                    viewBox="0 0 24 24"
                >
                    <path
                        fill="currentColor"
                        d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12z"
                    />
                </svg>
            </div>
        </div>
    </div>
</template>

<style scoped>
.titleBar {
    position: sticky;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    box-sizing: border-box;
    padding: 0px 0.2rem;
    background-color: rgba(255, 255, 255, 0.8);
    /* background-color: #ccc; */
    user-select: none;
    -webkit-user-select: none;
}
.title{
    padding-left: 1rem;
    font-weight: bold;
}
.titleBar .titlebar-button {
    width: 24px;
    height: 24px;
    padding: 0 0.2rem;
    user-select: none;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
}
.controllWindow{
    display: flex;
}
.resize:hover {
    background-color: #aaa;
}

#titlebar-close:hover {
    background-color: rgb(222, 65, 65);
}
#titlebar-close:hover svg {
    color: white;
}
</style>
