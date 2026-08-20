<script setup lang="ts">
import Bar from "../components/bar/index.vue"
import TitleBar from '../components/titleBar/index.vue'
import { onMounted,ref } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/core';

onMounted(async()=>{
    const windowName = await getCurrentWindow();
    console.log("appName",windowName.label);
})


const server_ip = ref<string>("");
const server_port = ref<number>(0);

function save_server(){
    invoke("change_server",{serverAddr:server_ip.value,
                serverPort:server_port.value})
}

</script>

<template>
    <TitleBar/>
    <Bar/>
    <div class="server">
        <div class="config">
            <div class="address">
                <label for="ip">服务器地址：</label><input type="text" id="ip" v-model="server_ip" />
            </div>
            <div class="port">
                <label for="port">绑定端口：</label><input type="number" id="port" v-model="server_port" />
            </div>
            <button @click="save_server">保存</button>
        </div>
    </div>
</template>


<style scoped>
.server{
    padding: 0 1rem;
    margin-top: 1rem;
}
.config{
    color: var(--purple);
}
</style>