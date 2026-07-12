<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
// import { invoke } from '@tauri-apps/api/core';
import { ref,onMounted } from 'vue';
import { useProxyStore } from '../store/proxy';
enum FRPConfigOperation{
    Create="Create",
    Edit="Edit",
    Delete="Delete"
}


const store = useProxyStore();

listen("get_config",(data)=>{
    console.log("获取的数据是：",data.payload);
    configData.value = (data.payload as Config).proxies
})

listen("config_updated",()=>{
    readConfig()
})



function readConfig(){
    invoke("get_config",{name:"test1"})
}

// function changeNodeStatus(name:string){
//     invoke()
// }

const configData  = ref<Proxies[]>([])

// const testProxy = ref<Proxies>({
//     name:"unknow",
//     type:"tcp",
//     localIp:"localhost",
//     localPort:9999,
//     remotePort:9999
// })

function createNewWindow(operation:FRPConfigOperation,proxy:Proxies){
    store.setProxy(proxy);
    console.log(store.getProxy());
    localStorage.setItem("proxy",JSON.stringify(proxy))
    invoke("create_window",{operation:operation,proxy:proxy})
}

onMounted(()=>{
    readConfig()
    const lists = document.querySelector("#data")
    console.log(lists);
})

</script>

<template>
    <div class="config">
        <!-- <h1>setting</h1> -->
        <div class="configTitle" id="title">
                <span class="name">名称</span>
                <span class="type">类型</span>
                <span class="localAddr">本地地址</span>
                <span class="localPort">本地端口</span>
                <span class="remote">远程端口</span>
                <span class="status">启用状态</span>
        </div>
        <div class="sever">
            <div class="configTitle" id="data" v-for="item in configData" @dblclick="createNewWindow(FRPConfigOperation.Edit,item)">
                <span class="name">{{ item.name }}</span>
                <span class="type">{{ item.type }}</span>
                <span class="localAddr">{{ item.localIp }}</span>
                <span class="localPort">{{ item.localPort }}</span>
                <span class="remote">{{ item.remotePort }}</span>
                <span class="status" @click="item.enable=!item.enable">{{ item.enable==false?"X":"√" }}</span>
            </div>
        </div>
    </div>
</template>

<style scoped>
.config{
    width: 100%;
    height: 100%;
    user-select: none;
}
.sever{
    width: 100%;
    height: 90%;
    border: 1px solid #666;
    overflow: hidden;
    overflow-y: scroll;
    scrollbar-width: thin;
    scrollbar-color: #ca9ee5 #3b4054 ;
}
#title{
    font-weight: bold;
    padding: 4px 0;
    color: #ca9ee5;
    font-size: small;
}
.configTitle{
    width: 98%;
    display: flex;
    justify-content: space-around;
    align-items: center;
    color: whitesmoke;
    padding: 0 2px;
}
.sever .configTitle span{
    border-right: 2px solid #ccc;
    padding-left: 6px;
    overflow: hidden;
    text-overflow: ellipsis;
}
#data:hover{
    background-color: #54586d;
}
.configTitle span{
    padding-left: 1rem;
    border-right: 0;
    overflow: hidden;
    text-overflow: ellipsis;
}
.name{
    flex: 3;
    overflow: hidden;
    text-overflow: ellipsis;
}
.type{
    flex: 1;
}
.localAddr{
    flex: 3;
}
.localPort{
    flex: 3;
}
.remote{
    flex: 3;
}
.configTitle .status{
    flex: 2;
    border-right: none;
    cursor: pointer;
}
.title{
    padding-right: 3rem;
    display: flex;
    justify-content: space-between;
}
</style>