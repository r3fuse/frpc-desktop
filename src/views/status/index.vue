<script setup lang="ts">
import Status from "../status.vue";
import { invoke } from "@tauri-apps/api/core";
import { useStatusStore } from "../../store/statusStore.ts";
import { ref, watch } from "vue";

const statusStore = useStatusStore();

function start() {
    invoke("start_frp");
    statusStore.changeWork();
}

function stop() {
    invoke("stop_frp");
    statusStore.changeWork();
}
const frpcStatus = ref(["noWork", "success", "warn", "error"]);
</script>

<template>
    <div class="statusHome">
        <div class="controller">
            <div class="status">
                <span :class="frpcStatus[statusStore.frpcStatus]"
                    >全局状态：CONNECTED</span
                >
            </div>
            <div>
                <button @click="statusStore.isWork ? stop() : start()">
                    {{ statusStore.isWork ? "停止" : "启动" }}
                </button>
            </div>
        </div>
        <div class="statusBox">
            <Status />
        </div>
    </div>
</template>

<style lang="scss" scoped>
@use "../../assets/css/main.scss" as *;
.statusHome {
    height: 98%;
    display: flex;
    flex-direction: column;
}
.controller {
    padding: 1rem 2rem;
    height: 26px;
    display: flex;
    justify-content: space-between;

    .status {
        color: white;
        font-weight: bold;
        display: flex;
        align-items: center;

        span {
            position: relative;
            display: flex;
            padding-left: 0.4rem;
            align-items: center;
        }

        & span::before {
            $size: 14px;
            content: "";
            width: $size;
            height: $size;
            border-radius: $size;
            background-color: #ccc;
            position: absolute;
            left: -1rem;
        }
        & .success::before {
            background-color: $success;
        }
        & .warn::before {
            background-color: $warn;
        }
        & .error::before {
            background-color: $error;
        }
    }
}
.statusBox {
    flex: 1;
}
.bottom {
    flex: 1;
    box-sizing: border-box;
    border-top: 2px solid #63687f;
    overflow: hidden;
}
</style>
