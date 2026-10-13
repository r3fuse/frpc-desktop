<script setup lang="ts">
import { open } from "@tauri-apps/plugin-shell";

async function jumpGithub() {
    await open("https://github.com/r3fuse/frpc-desktop");
}

async function jumpIssues() {
    await open("https://github.com/r3fuse/frpc-desktop/issues");
}
</script>

<template>
    <div class="about">
        <div class="main">
            <div class="mid">
                <img src="../assets/line.png" alt="logo" width="100px" />
                <h2>Frpc Desktop</h2>
                <p>
                    轻量级Frp跨平台桌面客户端，可视化配置，轻松地将内网服务穿透到公网
                </p>
                <p>轻量/可视化配置/免费开源</p>
                <div class="link">
                    <button class="github btn" @click="jumpGithub()">
                        <img src="../assets/github.png" alt="" />
                        <span>Github</span>
                    </button>
                    <button class="issues btn" @click="jumpIssues()">
                        <span>?</span>
                        <span>问题反馈</span>
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
@use "../assets/css/main.scss" as *;
.about {
    display: flex;
    flex-direction: column;
    height: 100%;
    background-color: #eeeeee;
    overflow: hidden;
    box-sizing: border-box;
    padding: 1rem;

    h2 {
        margin-top: 10px;
    }
}
.main {
    flex: 1;
    color: var(--purple);
    font-weight: bold;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: white;

    .mid {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

        p {
            margin: 0;
        }

        .link {
            display: flex;
            gap: 1rem;
            margin-top: 1rem;

            .btn {
                display: flex;
                align-items: center;
                justify-content: space-around;
                font-size: 16px;
                border-radius: 4px;
                cursor: pointer;
                width: 7rem;
                font-weight: 100;
                font-weight: bold;

                img {
                    width: 16px;
                    height: 16px;
                }

                &.issues {
                    color: rgb(221, 52, 109);
                    background-color: #eb886071;
                    border: 1px solid #e4633c;
                }

                &.github {
                    background-color: rgb(160, 160, 160);
                    color: white;
                    border: 1px solid #1f1f1e;
                }
            }
        }
    }
}
#projectUrl {
    cursor: pointer;
}
</style>
