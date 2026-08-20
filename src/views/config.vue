<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { invoke, } from '@tauri-apps/api/core'
import { useRoute } from 'vue-router';
import { type Proxies } from "../utils/type"
import TitleBar from "../components/titleBar/index.vue"
import { notification } from "../utils/notification.ts";
const route = useRoute();
const {operation} = route.params

const inputInfo = ref<Proxies>({
    name:"",
    type:"tcp",
    localIp:"localhost",
    localPort:0,
    remotePort:0
})


if(operation == "Edit" && localStorage.getItem("proxy") ){
    const data =JSON.parse(localStorage.getItem("proxy") as string)
    inputInfo.value = {...data} as Proxies
}
const connectType = ref(["tcp","udp","http","https"])

const data = JSON.parse(localStorage.getItem("proxy") as string)

async function addConfig(proxy:Proxies){
    try {
        await invoke("add_proxy",{config:proxy})
    } catch (error) {
        console.log(error);
        notification({msg_type:"Error",title:"error",message:(error as string).toString()})
    }
}

function changeConfig(name:string){
    console.log(inputInfo.value);
    invoke("change_config_by_name",{name:name,proxy:inputInfo.value})
}

onMounted(async()=>{
    console.log(route.params);
    console.log(inputInfo.value);
    console.log(data);
})



</script>

<template>
   <div class="proxy">
        <TitleBar/>
        <div class="innerBox" >
            <div class="name item">
                <label for="name">名称：</label><input type="text" id="name" v-model="inputInfo.name" autocomplete="off">
            </div>
            <div class="type item">
                <label for="type">类型：</label>
                <select name="" id="type" v-model="inputInfo.type">
                    <option v-for="value in connectType" :value="value">{{ value }}</option>
                </select>
            </div>
            <div class="localAddr item">
                <label for="localAddr">本地ip：</label><input type="text" id="localAddr" v-model="inputInfo.localIp"
                    autocomplete="off">
            </div>
            <div class="localPort item">
                <label for="localPort">本地端口：</label><input type="number" id="localPort" v-model="inputInfo.localPort"
                    autocomplete="off">
            </div>
            <div class="remotePort item">
                <label for="remotePort">远程端口：</label><input type="number" id="remotePort" v-model="inputInfo.remotePort"
                    autocomplete="off">
            </div>
            <div class="btn">
                <button @click="addConfig(inputInfo)" v-if="operation == 'Create'">保存</button>
                <button @click="changeConfig(data.name)" v-if="operation == 'Edit'" >修改</button>
            </div>
        </div>
    </div>
</template>


<style scoped>
.item{
    display: flex;
    margin: 0.6rem 0;
}
.item input{
    flex: 1;
}
.proxy{
    width: 100%;
    height: 100%;
    color: whitesmoke;
    background-color: rgb(70, 76, 100);
    font-size: small;
    font-weight: bold;
    box-sizing: border-box;
}
.innerBox{
    width: 80%;
    margin: 0 auto;
    padding: 1rem 0;
}
.btn{
    display: flex;
    justify-content: end;
}
</style>