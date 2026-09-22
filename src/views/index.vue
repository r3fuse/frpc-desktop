<script setup lang="ts">
import TitleBar from "../components/titleBar/index.vue";
import SideBar from "../components/sideBar/index.vue";
import { listen } from "@tauri-apps/api/event";
import { notification } from "../utils/notification.ts";

type MessageType = "Success" | "Warning" | "Error";

interface MsgType {
    msg_type: MessageType;
    title: string;
    message: string;
}

listen("notification", (event) => {
    console.log("Received notification:", event.payload);
    const msg: MsgType = event.payload as MsgType;
    notification({
        msg_type: msg.msg_type,
        title: "通知",
        message: msg.message,
    });
    // alert((event.payload as MsgType).message)
});
</script>

<template>
    <div class="index">
        <TitleBar />
        <div class="main">
            <div class="side">
                <SideBar />
            </div>
            <div class="view">
                <!-- <RouterView v-slot="{ Component }">
                    <KeepAlive include="alive">
                        <component :is="Component"></component>
                    </KeepAlive>
                </RouterView> -->
                <RouterView />
            </div>
        </div>
    </div>
</template>

<style scoped>
.index {
    max-height: 100vh;
    height: 100vh;
    display: flex;
    background-color: #3b4054;
    flex-direction: column;
    box-sizing: border-box;
}
.main {
    flex: 1;
    display: flex;
    flex-direction: row;
}
.view {
    flex: 1;
    height: 94vh;
}
.side {
    width: 20vw;
}
</style>
