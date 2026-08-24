<script setup lang="ts">
import Bar from "../components/bar/index.vue";
import TitleBar from "../components/titleBar/index.vue";
import { onMounted, ref, watch } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { type Config } from "../utils/type.ts";

onMounted(async () => {
    const windowName = await getCurrentWindow();
    console.log("appName", windowName.label);
});

const config = ref<Config>({
    serverAddr: "",
    serverPort: 0,
    auth: {
        method: "token",
        token: "",
    },
});

function save_server() {
    invoke("change_server", {
        newConfig: {
            serverAddr: config.value.serverAddr,
            serverPort: config.value.serverPort,
            auth: config.value.auth,
        },
    });
}

watch(config.value, (newVal) => {
    if (newVal?.auth?.token == "") {
        config.value!.auth!.token = null;
    }
});
</script>

<template>
    <TitleBar />
    <Bar />
    <div class="server">
        <div class="config">
            <div class="address">
                <label for="ip">服务器地址：</label
                ><input type="text" id="ip" v-model="config.serverAddr" />
            </div>
            <div class="port">
                <label for="port">绑定端口：</label
                ><input type="number" id="port" v-model="config.serverPort" />
            </div>
            <div class="token">
                <label for="token">token：</label
                ><input type="text" id="token" v-model="config.auth!.token" />
            </div>
            <button @click="save_server">保存</button>
        </div>
    </div>
</template>

<style scoped>
.server {
    padding: 0 1rem;
    margin-top: 1rem;
}
.config {
    color: var(--purple);
}
</style>
