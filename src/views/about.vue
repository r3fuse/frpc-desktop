<script setup lang="ts">
import { notification } from "../utils/notification.ts";

async function copyUrl() {
    navigator.clipboard
        .writeText("https://github.com/r3fuse/frpc_Client")
        .then(() => {
            notification({
                msg_type: "Success",
                title: "通知",
                message: "复制成功",
            });
        })
        .catch(() => {
            notification({
                msg_type: "Error",
                title: "通知",
                message: "复制失败",
            });
        });
}
</script>

<template>
    <div class="about">
        <div class="main">
            about
            <p id="projectUrl" @click="copyUrl">
                项目地址：https://github.com/r3fuse/frpc_Client
            </p>
        </div>
    </div>
</template>

<style scoped>
.main {
    padding: 2rem;
    color: var(--purple);
    font-weight: bold;
}
#projectUrl {
    cursor: pointer;
}
</style>
