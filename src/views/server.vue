<script setup lang="ts">
import { InputTypeHTMLAttribute, onMounted, ref } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { type Config } from "../utils/type.ts";
import { changeServerConfigChain } from "../hooks/chains.ts";
import { useConfigStore } from "../store/configStore.ts";

// async function getConfig(): Promise<Config> {
//     return await invoke("get_config");
// }

const configStore = useConfigStore();

const config = ref<Config>({
    serverAddr: "",
    serverPort: 7000,
    auth: {
        method: null,
        token: "",
    },
    transport: {
        protocol: "tcp",
        dialServerTimeout: 10,
        poolCount: 0,
        tcpMux: true,
        tcpMuxKeepaliveInterval: 30,
        heartbeatInterval: 30,
        heartbeatTimeout: 90,
    },
});

const tokenInputType = ref<InputTypeHTMLAttribute>("password");

onMounted(async () => {
    const windowName = await getCurrentWindow();
    //从后端获取config并初始化
    config.value = configStore.get_config();
    // config.value = await getConfig();
    config.value.auth ??= { method: null, token: "" };
    config.value.transport ??= {
        protocol: "tcp",
        dialServerTimeout: 10,
        poolCount: 0,
        tcpMux: true,
        tcpMuxKeepaliveInterval: 30,
        heartbeatInterval: 30,
        heartbeatTimeout: 90,
    };
    console.log(config.value);
    console.log("appName", windowName.label);
});

//保存服务器设置
function save_server() {
    changeServerConfigChain(config.value);
}
</script>

<template>
    <div class="server">
        <div class="inner">
            <h2>客户端配置</h2>
            <!-- <span>配置服务器连接与网络行为</span> -->
            <div class="box">
                <p>服务器配置</p>
                <div class="config">
                    <div class="line">
                        <div class="address item">
                            <label for="ip">服务器地址：</label>
                            <input
                                type="text"
                                id="ip"
                                v-model="config.serverAddr"
                            />
                        </div>
                        <div class="port item">
                            <label for="port">绑定端口：</label>
                            <input
                                type="number"
                                id="port"
                                max="65536"
                                min="1"
                                v-model="config.serverPort"
                            />
                        </div>
                    </div>
                    <div class="line">
                        <div class="method item">
                            <label for="method">验证方式：</label>
                            <select v-model="config.auth!.method">
                                <option :value="null">无</option>
                                <option value="token">token</option>
                                <!-- <option value="oidc">oidc</option> -->
                            </select>
                        </div>
                        <div class="token item">
                            <label for="token">token：</label>
                            <div class="tokenInput">
                                <input
                                    :type="tokenInputType"
                                    id="token"
                                    v-model="config.auth!.token"
                                    placeholder="若无则留空"
                                />
                                <span
                                    class="eye"
                                    @click="
                                        tokenInputType =
                                            tokenInputType == 'text'
                                                ? 'password'
                                                : 'text'
                                    "
                                >
                                    <svg
                                        v-if="tokenInputType == 'text'"
                                        t="1788852386194"
                                        class="icon"
                                        viewBox="0 0 1024 1024"
                                        version="1.1"
                                        xmlns="http://www.w3.org/2000/svg"
                                        p-id="1850"
                                        width="18"
                                        height="18"
                                    >
                                        <path
                                            d="M512 287.776c-156.256 0-300 73.408-433.216 224.224 133.216 150.784 276.992 224.224 433.216 224.224S812 662.816 945.216 512C812 361.216 668.224 287.776 512 287.776z m0 508.672c-188.512 0-359.168-94.816-512-284.448 152.832-189.632 323.488-284.448 512-284.448S871.168 322.368 1024 512c-152.832 189.632-323.488 284.448-512 284.448z m0-174.016a110.4 110.4 0 1 0 0-220.864 110.4 110.4 0 0 0 0 220.864z m0 60.224a170.656 170.656 0 1 1 0-341.312 170.656 170.656 0 0 1 0 341.312z"
                                            fill="#000000"
                                            p-id="1851"
                                        ></path>
                                    </svg>
                                    <svg
                                        v-else-if="tokenInputType != 'text'"
                                        t="1788852171337"
                                        class="icon"
                                        viewBox="0 0 1024 1024"
                                        version="1.1"
                                        xmlns="http://www.w3.org/2000/svg"
                                        p-id="1666"
                                        width="18"
                                        height="18"
                                    >
                                        <path
                                            d="M928.845558 421.757952l45.62944 70.99392c7.153664 11.13088 3.109888 25.468928-9.03168 32.0256-12.141568 6.556672-27.783168 2.849792-34.936832-8.27904l-44.27776-68.893696c-49.41824 28.005376-99.275776 49.5104-149.352448 65.000448l35.229696 77.840384c5.400576 11.931648-0.774144 25.616384-13.790208 30.5664-13.01504 4.950016-27.94496-0.709632-33.345536-12.640256l-36.436992-80.50688a22.441984 22.441984 0 0 1-0.78848-1.988608c-49.997824 11.56608-100.14208 17.321984-150.217728 17.747968v87.3216c0 12.918784-11.42272 23.390208-25.515008 23.390208-14.092288 0-25.516032-10.471424-25.516032-23.389184v-87.10144c0-0.54272 0.02048-1.08032 0.059392-1.6128-49.934336-3.134464-99.72736-11.423744-149.163008-24.389632 0.166912 3.328-0.442368 6.740992-1.927168 10.021888l-36.436992 80.50688c-5.400576 11.930624-20.329472 17.590272-33.345536 12.640256-13.016064-4.950016-19.18976-18.634752-13.790208-30.5664l36.435968-80.50688a23.037952 23.037952 0 0 1 3.433472-5.412864c-45.037568-14.46912-89.719808-32.734208-133.881856-54.429696a995.746816 995.746816 0 0 1-16.38912-8.240128l-47.9744 74.642432c-7.15264 11.128832-22.79424 14.835712-34.935808 8.27904-12.141568-6.556672-16.185344-20.89472-9.03168-32.024576l48.120832-74.870784c-17.281024-10.04544-33.57696-20.2496-48.82944-30.461952-11.23328-7.520256-21.10976-14.52544-29.564928-20.840448-5.169152-3.861504-8.773632-6.685696-10.745856-8.298496-10.514432-8.600576-11.431936-23.386112-2.050048-33.024 9.382912-9.638912 25.511936-10.479616 36.026368-1.87904 6.176768 5.052416 18.516992 14.270464 36.434944 26.267648 30.117888 20.164608 64.6656 40.364032 103.04512 59.218944 259.439616 127.455232 533.72416 128.717824 792.379392-79.681536 3.542016-2.863104 3.542016-2.863104 7.084032-5.766144 10.49088-8.624128 26.624-7.820288 36.031488 1.796096 9.409536 9.617408 8.531968 24.403968-1.958912 33.02912-3.73248 3.057664-3.73248 3.057664-7.465984 6.076416-26.23488 21.136384-52.64896 40.257536-79.210496 57.439232z"
                                            fill="#000000"
                                            p-id="1667"
                                        ></path>
                                    </svg>
                                </span>
                            </div>
                        </div>
                    </div>
                    <!-- <button @click="save_server">保存</button> -->
                </div>
            </div>
            <div class="box">
                <p>网络层配置</p>
                <div class="config">
                    <div class="line">
                        <div class="protocol item">
                            <label for="method">通信协议：</label>
                            <select v-model="config.transport!.protocol">
                                <option value="tcp">tcp</option>
                                <option value="kcp">kcp</option>
                                <option value="quic">quic</option>
                                <option value="websocket">websocket</option>
                                <option value="wss">wss</option>
                                <!-- <option value="oidc">oidc</option> -->
                            </select>
                        </div>
                        <div class="dialServerTimeout item">
                            <label for="dialServerTimeout">超时时间：</label>
                            <input
                                type="number"
                                id="dialServerTimeout"
                                max="65536"
                                min="1"
                                v-model="config.transport!.dialServerTimeout"
                            />
                        </div>
                    </div>
                    <div class="line">
                        <div class="poolCount item">
                            <label for="port">连接池大小：</label>
                            <input
                                type="number"
                                id="poolCount"
                                max="20"
                                min="1"
                                v-model="config.transport!.poolCount"
                            />
                        </div>
                    </div>
                    <div class="line">
                        <div class="tcpMux item">
                            <label for="tcpMux">多路复用</label>
                            <span>&nbsp;&nbsp;&nbsp;</span>
                            <button
                                class="switch"
                                :class="{ active: config.transport!.tcpMux }"
                                @click="
                                    config.transport!.tcpMux =
                                        !config.transport!.tcpMux
                                "
                            >
                                <span class="dot"></span>
                            </button>
                            <!-- <input
                            type="checkbox"
                            id="tcpMux"
                            v-model="config.transport!.tcpMux"
                        /> -->
                        </div>
                    </div>
                    <div class="line">
                        <div class="tcpMuxKeepaliveInterval item">
                            <label for="tcpMuxKeepaliveInterval"
                                >多路复用心跳间隔：</label
                            >
                            <input
                                type="number"
                                max="65536"
                                min="1"
                                id="tcpMuxKeepaliveInterval"
                                v-model="
                                    config.transport!.tcpMuxKeepaliveInterval
                                "
                                placeholder="s"
                            />
                        </div>
                        <div class="heartbeatInterval item">
                            <label for="heartbeatInterval"
                                >心跳包间隔时间：</label
                            >
                            <input
                                type="number"
                                max="65536"
                                min="1"
                                id="heartbeatInterval"
                                v-model="config.transport!.heartbeatInterval"
                            />
                        </div>
                        <div class="heartbeatTimeout item">
                            <label for="heartbeatTimeout">心跳超时时间：</label>
                            <input
                                type="number"
                                max="65536"
                                min="1"
                                id="heartbeatTimeout"
                                v-model="config.transport!.heartbeatTimeout"
                                width="120px"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <div class="operation">
            <button>恢复默认</button>
            <button @click="save_server()">保存配置</button>
        </div>
    </div>
</template>

<style lang="scss" scoped>
@use "../assets/css/main.scss" as *;

input {
    font-size: 16px;
}
.server {
    // min-height: 100vh;
    height: 100%;
    max-height: 100%;
    // margin-top: 2rem;
    user-select: none;
    -webkit-user-select: none;
    display: flex;
    flex-direction: column;
    overflow: hidden;

    .inner {
        overflow: hidden;
        overflow-y: scroll;
        padding: 2rem 0rem;
    }

    h2 {
        margin-left: 2rem;
        margin-bottom: 0;
    }

    .box {
        display: flex;
        align-content: center;
        justify-content: center;
        flex-direction: column;
        padding: 2rem 2rem 1rem 2rem;
        border-radius: 6px;
        margin: 1rem 2rem;
        box-sizing: border-box;
        // border: 0.1rem solid #cccccc;
        transition: all 300ms ease-in-out;
        box-shadow: 0 0 3px 1px #ccc;

        &:focus-within p {
            background: linear-gradient(
                to bottom,
                transparent 50%,
                rgba(101, 138, 241, 0.5) 50%
            );
            background-size: 100% 100%;
            background-repeat: no-repeat;
        }

        p {
            width: fit-content;
            font-size: 1.2rem;
            font-weight: bold;
            margin: 0;
            color: rgb(14, 13, 13);
            position: relative;
            transition: all 300ms ease-in-out;
            margin-left: 1.4rem;
            background: linear-gradient(
                to bottom,
                transparent 50%,
                rgba(101, 138, 241, 0.5) 50%
            );
            background-size: 0 100%;
            background-repeat: no-repeat;

            &::before {
                content: "";
                width: 4px;
                height: 2rem;
                background-color: rgb(205, 57, 57);
                position: absolute;
                left: -1.4rem;
                top: -6px;
                border-radius: 2px;
            }
        }

        .config {
            display: flex;
            flex-direction: column;
            // align-items: center;
            font-size: 16px;
            padding: 0.4rem 0rem;

            // input[type="number"] {
            //     width: 8rem;
            //     text-align: center;
            // }

            .line {
                display: flex;
                align-items: center;
                justify-content: left;

                .item {
                    width: 50%;
                }
                .tcpMux.item {
                    display: flex;
                    flex-direction: row;
                    align-items: center;

                    .switch {
                        width: 40px;
                        height: 20px;
                        padding: 2px;
                        border: none;
                        border-radius: 14px;
                        background: #ccc;
                        cursor: pointer;

                        .dot {
                            display: block;
                            width: 16px;
                            height: 16px;
                            border-radius: 50%;
                            background: white;

                            transform: translateX(0);
                            transition: transform 200ms ease;
                        }

                        &.active {
                            background: #1e7cda;

                            .dot {
                                transform: translateX(20px);
                            }
                        }
                    }
                }
            }

            .item.item {
                margin: 0.4rem;
                display: flex;
                flex-direction: column;
                width: 50%;
            }
            .token {
                user-select: none;
                display: flex;

                .tokenInput {
                    position: relative;

                    #token {
                        width: 100%;
                    }

                    .eye {
                        position: absolute;
                        right: 1rem;
                        top: 6px;
                        cursor: pointer;
                    }
                }
            }
        }
    }

    .operation {
        position: sticky;
        bottom: 0;
        height: 4rem;
        display: flex;
        justify-content: end;
        align-items: center;
        border-top: 1px solid #ccc;
        padding-right: 2rem;

        button {
            margin-left: 1rem;
            height: fit-content;
            border: 0;
            background-color: white;
            box-shadow: 0 0 1px 1px #ccc;
            border-radius: 4px;
            padding: 4px 1rem;
            cursor: pointer;
        }

        button:nth-child(2) {
            background-color: #1e7cda;
            color: white;
        }
    }
}
</style>
