<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { ref,onMounted, watch } from 'vue';
enum FRPConfigOperation{
    Create="Create",
    Edit="Edit",
    Delete="Delete"
}

listen("get_config",(data)=>{
    console.log("获取的数据是：",data.payload);
    configData.value = (data.payload as Config).proxies
})

listen("config_updated",()=>{
    readConfig()
})

//修改启用状态
try {
    listen("change_activation_status",()=>{
        readConfig()
    })
} catch (error) {
    console.log(error);
}

function deleteConfig(proxy:Proxies){
    invoke("delete_config",{proxy})
}

const selectedItem = ref<string>("");
const selectedProxy = ref<Proxies>({
    name:"unknow",
    type:"http",
    localIp:"127.0.0.1",
    localPort:65535,
    remotePort:65535
})
const isShowIp = ref(false)

function readConfig(){
    invoke("get_config",{name:"test1"})
}

async function change_activation_status(name:string){
    invoke("change_activation_status",{name}).then(()=>{
        readConfig()
    })
}


const configData  = ref<Proxies[]>([])

function createNewWindow(operation:FRPConfigOperation,proxy:Proxies){
    localStorage.setItem("proxy",JSON.stringify(proxy))
    invoke("create_window",{operation:operation,proxy:proxy})
}

document.addEventListener("contextmenu",(e)=>{
    e.preventDefault()
})

watch(selectedItem,(newValue,oldValue)=>{
    console.log("change","newValue"+newValue,"oldValue:"+oldValue);
})

onMounted(()=>{
    readConfig()
    const lists = document.querySelector("#data")
    console.log(lists);
})

</script>

<template>
    <div class="config">
        <div class="ip" @click="isShowIp=!isShowIp">
            <span style="color: #ca9ee5;font-size: small;font-weight: bold;">服务器地址：</span>
            <span style="color: white;" v-show="isShowIp">8.1.2.200</span>
            <span style="color: white;" v-show="!isShowIp">*******</span>
        </div>
        <!-- <h1>setting</h1> -->
        <div class="tools">
                <div class="item" @click="createNewWindow(FRPConfigOperation.Create,selectedProxy as Proxies)">
                    <img src="../assets/add.svg" alt="" ><span>添加</span>
                </div>
                <div class="item" @click="createNewWindow(FRPConfigOperation.Edit,selectedProxy as Proxies)">
                    <img src="../assets/edit.svg" alt=""><span>编辑</span>
                </div>
                <div class="item" @click="deleteConfig(selectedProxy)">
                    <img src="../assets/remove.svg" alt=""><span>删除</span>
                </div>
        </div>
        <div class="configTitle" id="title">
                <span class="name">名称</span>
                <span class="type">类型</span>
                <span class="localAddr">本地地址</span>
                <span class="localPort">本地端口</span>
                <span class="remote">远程端口</span>
                <span class="status">启用状态</span>
        </div>
        <div class="cfg"  >
            <div class="configTitle" id="data" :class="{selected:selectedItem==item.name}" v-for="item in configData" @click="selectedItem =item.name,selectedProxy=item"  @dblclick="createNewWindow(FRPConfigOperation.Edit,item)">
                <span class="name">{{ item.name }}</span>
                <span class="type">{{ item.type }}</span>
                <span class="localAddr">{{ item.localIp }}</span>
                <span class="localPort">{{ item.localPort }}</span>
                <span class="remote">{{ item.remotePort }}</span>
                <!-- <span class="status" @click="item.enable=!item.enable">{{ item.enable==false?"X":"√" }}</span> -->
                <span class="status" @click="change_activation_status(item.name)">{{ item.enabled==false?"X":"√" }}</span>
            </div>
            <div class="empty" @click="selectedItem = ' ' "></div>
        </div>
    </div>
</template>

<style scoped>
.config{
    width: 100%;
    height: 100%;
    user-select: none;
    border: 2px solid #ccc;
    position: relative;
    box-sizing: border-box;
    padding: 0 0.4rem;
}
.ip{
    position: absolute;
    top: -0.8rem;
    left: 0.4rem;
    background-color: #3b4054;
    padding: 0 0.6rem;
    cursor: pointer;
}
.tools{
    margin-top: 1rem;
    width: 100%;
    color: white;
    display: flex;
    gap: 1rem;
    cursor: pointer;
    padding-left: 0.4rem;
}
.tools span:hover{
    color: #ccc;
}
.tools .item{
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: small;
}
.tools .item img{
    width: 1rem;
    height: 1rem;
}
.cfg{
    width: 100%;
    height: 74%;
    border: 1px solid #666;
    overflow: hidden;
    overflow-y: scroll;
    scrollbar-width: thin;
    scrollbar-color: #ca9ee5 #3b4054 ;
    display: flex;
    flex-direction: column;
}
.empty{
    flex: 1;
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
    position: relative;
}
.cfg .configTitle span{
    border-right: 2px solid #ccc;
    padding-left: 6px;
    overflow: hidden;
    text-overflow: ellipsis;
}
#data:hover{
    background-color: #54586d;
}
.selected{
    background-color: #54586d;
}
.configTitle span{
    padding-left: 1rem;
    border-right: 0;
    overflow: hidden;
    text-overflow: ellipsis;
}
.menu{
    width: 6rem;
    position: absolute;
    left: 8rem;
    top: 0.4rem;
    background-color: #53596f;
    z-index: 999;
    font-size: small;
    border: 1px solid #686e83;
    box-shadow: 1px 2px 1px 1px #8e8e8e8e,
                1px 1px 1px 1px #f2f2f2;
}
.menu>ul{
    list-style: none;
    padding: 0;
}
.menu>ul>li{
    padding: 0 1rem;
}
.menu>ul>li:hover{
    cursor: pointer;
    background-color: #6a6e81;
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