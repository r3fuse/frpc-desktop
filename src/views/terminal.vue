<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { ref,useTemplateRef } from "vue";

const log = ref<string[]>([]);
listen("frp-log", (event) => {
    console.log(event.payload);
    let tempStr = stripAnsi(event.payload as string);
    log.value.push(tempStr);
    goBottom();
});

function stripAnsi(str: string) {
    return str.replace(/\x1B\[[0-9;]*[mK]/g, "");
}

function goTop() {
    const logElement = document.querySelector(".log");
    if (logElement) {
        logElement.scrollTop = 0;
    }
}
const buttom = useTemplateRef("bottom")
function goBottom() {
    buttom.value?.scrollIntoView({block:"end",behavior:"smooth"});
}

</script>

<template>
    <div class="terminal">
        <div class="title">
            <span @click="goTop">执行日志📚</span>
            <span @click="goBottom">底部</span>
        </div>
        <div class="log">
            <ul>
                <li v-for="line in log" :key="line">{{ line }}</li>
                <span ref="bottom"></span>
            </ul>
            <!-- <p id="bottom"></p> -->
        </div>
    </div>
</template>

<style scoped>
a{
    color: white;
    text-decoration: none;
}

.terminal{
    height: 100%;
    max-height: 100%;
    background-color: rgb(48, 52, 70);
    padding: 0.8rem;
    box-sizing: border-box;
    overflow: hidden;
    position: relative;
    display: flex;
    flex-direction: column;
}
.log {
    /* max-height: 80%;
    height: 80%; */
    flex: 1;
    overflow: hidden;
    overflow-y: scroll;
    scrollbar-width: thin;
    scrollbar-color: rgb(176, 192, 231) transparent;
    background-color: rgb(48, 52, 70);
    color: rgb(123, 158, 240);
    font-weight: bold;
    font-family: consolas;
    padding: 0.5rem;
    letter-spacing: 0.6px;
    /* border-radius: 10px; */
    /* border: 2px solid rgb(123, 158, 240); */
}
.title{
    height: 1rem;
    width: 100%;
    user-select: none;
    cursor: pointer;
    z-index: 99;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

span{
    font-size: small;
    font-weight: bold;
    /* position: absolute;
    top: 0.1rem;
    left: 2%; */
    color: rgb(123, 158, 240);
    background-color: rgb(48, 52, 70);
}

.log ul{
    margin: 0;
    padding-right: 0;
    padding-left: 1rem;
}
</style>