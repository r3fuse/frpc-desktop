<script setup lang="ts">
import { ref } from "vue";
import terminal from "./terminal.vue";
import Status from "./status.vue";
import Bar from "../components/bar/index.vue"
import TitleBar from "../components/titleBar/index.vue"
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { notification } from "../utils/notification.ts";

type MessageType = "Success" | "Warning" | "Error"

interface MsgType {
    msg_type:MessageType,
    title:string,
    message:string
}

listen("notification", (event) => {
    console.log("Received notification:", event.payload);
    const msg:MsgType = event.payload as MsgType
    notification({msg_type:msg.msg_type,title:"通知",message:msg.message})
    // alert((event.payload as MsgType).message)
});

function start() {  
    invoke("start_frp");
}

function stop() {
    invoke("stop_frp");
}

function move_circle(){
    const circle = document.querySelector(".circle");
    console.log(circle);
}

const status = ref<boolean>(false)


</script>

<template>
    <div class="index">
        <TitleBar/>
        <Bar/>
        <div class="controller">
            <div class="btn">
                <span>FRP</span>
                <div class="control"  @click="(status=!status)?start():stop(),move_circle()">
                    <div class="outBox" :class={green:status,red:!status}>
                        <!-- <div class="circle green" v-show="status"></div>
                        <div class="circle red" v-show="!status"></div> -->
                        <div class="circle" :class={left:status,right:!status}></div>
                    </div>
                </div>
            </div>
        </div>
        <div class="status">
            <Status/>
        </div>
        <div class="bottom">
            <terminal/>
        </div>
    </div>
</template>

<style scoped>
.index{
    max-height: 100vh;
    height: 100vh;
    display: flex;
    background-color: #3b4054;
    flex-direction: column;
    box-sizing: border-box;
}
.controller{
    padding:0 1rem .4rem 1rem;
    height: 26px;
    display: flex;
    flex-direction: column;
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
    color: #ca9ee5;
}
.btn .control{
    display: flex;
    cursor: pointer;
}
.outBox{
    width: 2.8rem;
    height: 1rem;
    border: 2px solid #ccc;
    border-radius: 12px;
    position: relative;
}
.circle{
    width: 1rem;
    height: 1rem;
    border-radius: 50%;
    position: absolute;
    background-color: #00b1fd;
    transition: all 300ms ease-in-out;
}
.circle.left{
    left: 1%;
    background-color: #0fde16;
}
.circle.right{
    left: 62%;
    background-color: #dc0f0f;
}
.status{
    height: 50%;
    padding: 0 1rem .4rem 1rem;
    box-sizing: border-box;
}
.bottom{
    flex: 1;
    box-sizing: border-box;
    border-top: 2px solid #63687f;
    overflow: hidden;
}
</style>