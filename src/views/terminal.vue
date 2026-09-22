<script setup lang="ts">
import { onMounted, ref, useTemplateRef } from "vue";
import { useStatusStore } from "../store/statusStore";
// defineOptions({
//     name: "alive",
// });
const statusStore = useStatusStore();
const terminal = ref<HTMLElement>();
const rightMenuShow = ref<boolean>(false);

function stripAnsi(str: string) {
    return str.replace(/\x1B\[[0-9;]*[mK]/g, "");
}

function goTop() {
    const logElement = document.querySelector(".log");
    if (logElement) {
        logElement.scrollTop = 0;
    }
}
const buttom = useTemplateRef("bottom");
function goBottom() {
    buttom.value?.scrollIntoView({ block: "end", behavior: "smooth" });
}

function InfoType(data: string) {
    if (data.includes("[I]")) return "info";
    if (data.includes("[W]")) return "warn";
    if (data.includes("[E]") || data.includes("failed")) return "error";
}
onMounted(() => {
    goBottom();
    const rightMenuDiv = document.querySelector<HTMLElement>(".rightMenu");
    const menuWidth = rightMenuDiv!.getBoundingClientRect().width;
    const menuHeight = rightMenuDiv!.getBoundingClientRect().height;
    terminal.value!.addEventListener("contextmenu", (e: MouseEvent) => {
        e.preventDefault();
        const rect = terminal.value!.getBoundingClientRect();
        let mouseX = Math.min(e.clientX, rect.width - menuWidth);
        let mouseY = Math.min(e.clientY, rect.height - menuHeight);
        console.log(rect.width, mouseX);
        if (mouseX >= rect.width) {
            rightMenuDiv!.style.top = `${mouseY}px`;
            rightMenuDiv!.style.left = `${mouseX}px`;
        } else {
            rightMenuDiv!.style.top = `${mouseY}px`;
            rightMenuDiv!.style.left = `${mouseX}px`;
        }
        rightMenuShow.value = !rightMenuShow.value;
    });
});
</script>

<template>
    <div class="terminal" ref="terminal">
        <div class="rightMenu" v-show="rightMenuShow">
            <ul>
                <li @click="(statusStore.clearLogs(), (rightMenuShow = false))">
                    Clear Log
                </li>
            </ul>
        </div>
        <div class="title">
            <span @click="goTop">执行日志📚</span>
            <span @click="goBottom">底部</span>
        </div>
        <div class="log">
            <ul>
                <li
                    v-for="line in statusStore.logs"
                    :key="line"
                    :class="InfoType(line)"
                >
                    <p>{{ line }}</p>
                </li>
                <span ref="bottom"></span>
            </ul>
            <!-- <p id="bottom"></p> -->
        </div>
    </div>
</template>

<style lang="scss" scoped>
@use "../assets/css/main.scss" as *;
a {
    color: white;
    text-decoration: none;
}

.terminal {
    height: 100%;
    max-height: 100%;
    background-color: rgb(48, 52, 70);
    padding: 0.8rem;
    box-sizing: border-box;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    position: relative;

    .rightMenu {
        position: fixed;
        background-color: white;
        font-weight: 400;
        letter-spacing: 1px;
        border-radius: 4px;
        overflow: hidden;
        padding: 4px 0;

        ul {
            margin: 0;
            padding: 0;
            list-style: none;

            li {
                padding: 0.2rem 2rem;
                cursor: pointer;
                font-family: "Courier New", Courier, monospace;

                &:hover {
                    background-color: $blue;
                }
            }
        }
    }

    .title {
        height: 1rem;
        width: 100%;
        user-select: none;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: space-between;
    }
    .log {
        // max-height: 80%;
        // height: 80%;
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
        letter-spacing: 1px;
        /* border-radius: 10px; */
        /* border: 2px solid rgb(123, 158, 240); */

        ul {
            margin: 0;
            padding-right: 0;
            padding-left: 1rem;

            li {
                p {
                    font-size: 1rem;
                }
            }

            .info {
                color: $success;
            }
            .error {
                color: $error;
            }
            .warn {
                color: $warn;
            }
        }
    }
    span {
        font-size: small;
        font-weight: bold;
        color: rgb(123, 158, 240);
        background-color: rgb(48, 52, 70);
    }
}
</style>
