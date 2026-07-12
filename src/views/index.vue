<script setup lang="ts">
import { ref } from "vue";
import terminal from "./terminal.vue";
import Nav from "./nav.vue";
import Status from "./status.vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
interface Message{
  msg_type:string,
  title:string,
  message:string
}

// enum FRPConfigOperation{
//     Create="Create",
//     Edit="Edit",
//     Delete="Delete"
// }

listen("notification", (event) => {
    console.log("Received notification:", event.payload);
    alert((event.payload as Message).message)
});

function start() {
    invoke("start_frp");
}

function stop() {
    invoke("stop_frp");
}

// function createWindow(){
//     invoke("open_config_window")
// }

// const testProxy = ref<Proxies>({
//     name:"unknow",
//     type:"tcp",
//     localIp:"localhost",
//     localPort:9999,
//     remotePort:9999
// })

// function createNewWindow(operation:FRPConfigOperation,proxy:Proxies){
//     invoke("create_window",{operation:operation,proxy:proxy})
// }
// const memory =ref(0)
// async function getMem(){
//     const [virual_mem,physical_mem]= await invoke("get_momery_usage")as number[];
//     console.log(virual_mem,physical_mem);
    
//     // console.log(mem/1024/1024);
//     // memory.value = mem/1024/1024
// }

const status = ref<boolean>(false)



</script>

<template>
    <div class="index">
        <div class="top">
            <!-- <h1>test</h1> -->
            <div class="btn">
                <span>FRP</span>
                <div class="control"  @click="(status=!status)?start():stop()">
                    <div class="outBox"></div>
                    <div class="circle green" v-show="status"></div>
                    <div class="circle red" v-show="!status"></div>
                </div>
            </div>
            <!-- <button @click="(status=!status)?start():stop()">start frp</button>
            <button @click="stop">stop frp</button> -->
            <!-- <button @click="createWindow">createWindow</button> -->
            <!-- <button @click="readConfig">readConfig</button> -->
            <!-- <button @click="createNewWindow(FRPConfigOperation.Edit)">Edit</button>
            <button @click="createNewWindow(FRPConfigOperation.Create,testProxy)">Create</button> -->
            <!-- <router-link to="/terminal">asd</router-link> -->
             <!-- <button @click="getMem">getMem</button><span>mem:{{ memory }}</span> -->
            <div>
                <div class="nav">
                    <Nav/>
                </div>
            </div>
            <div class="status">
                <Status/>
            </div>
        </div>
        <div class="bottom">
            <terminal/>
        </div>
    </div>
</template>

<style scoped>
.index{
    height: 100%;
    display: flex;
    background-color: #3b4054;
    flex-direction: column;
}
.top{
    padding: 1rem;
    flex: 5;
}
.nav{
    margin: 1rem 0;
}
.btn{
    /* position: relative; */
    display: flex;
    justify-content: end;
    align-items: center;
    user-select: none;
}
.btn span{
    padding: 0 1rem;
}
.btn .control{
    display: flex;
    cursor: pointer;
}
.outBox{
    width: 2.4rem;
    height: 12px;
    border: 2px solid #ccc;
    border-radius: 12px;
}
.circle{
    width: 1rem;
    height: 1rem;
    border-radius: 50%;
    /* position: absolute;
    left: 0;
    top: -8%; */
}
.green{
    background-color: #0fde16;
}
.red{
    background-color: rgb(220, 15, 15);
}
.bottom{
    flex: 3;
    overflow: hidden;
    border-top: 2px solid #63687f;
}

.status{
    height: 16rem;
}

</style>