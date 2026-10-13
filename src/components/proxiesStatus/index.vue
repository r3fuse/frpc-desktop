<script setup lang="ts">
import { ref } from "vue";
import { useConfigStore } from "../../store/configStore";
import MyDialog from "../myDialog/index.vue";
import { type Proxies } from "../../utils/type.ts";
import {
    switchProxyStatusChain,
    deleteProxyChain,
} from "../../hooks/chains.ts";
import { ProxyOperation } from "../../utils/type.ts";
import { refreshConfigData } from "../../hooks/chains.ts";

// enum FRPConfigOperation {
//     Create = "Create",
//     Edit = "Edit",
//     Delete = "Delete",
// }

const configStore = useConfigStore();

const dialogModel = ref({
    proxy: {} as Proxies,
    operation: ProxyOperation.Add,
    isShowDialog: false,
});

const rotating = ref<boolean>(false);

function rotate() {
    rotating.value = false;
    requestAnimationFrame(() => {
        rotating.value = true;
    });
}

async function refreshData() {
    await refreshConfigData();
}

document.addEventListener("contextmenu", (e) => {
    e.preventDefault();
});
</script>

<template>
    <div class="status">
        <MyDialog v-model="dialogModel"></MyDialog>
        <div class="title">
            <h3>代理列表</h3>
            <div class="btn">
                <div
                    class="refresh"
                    :class="{ rotating }"
                    @click="(refreshData(), rotate())"
                >
                    <svg
                        t="1791364350216"
                        class="icon"
                        viewBox="0 0 1024 1024"
                        version="1.1"
                        xmlns="http://www.w3.org/2000/svg"
                        p-id="4847"
                        width="20"
                        height="20"
                    >
                        <path
                            d="M934.4 206.933333c-17.066667-4.266667-34.133333 6.4-38.4 23.466667l-23.466667 87.466667C797.866667 183.466667 654.933333 96 497.066667 96 264.533333 96 74.666667 281.6 74.666667 512s189.866667 416 422.4 416c179.2 0 339.2-110.933333 398.933333-275.2 6.4-17.066667-2.133333-34.133333-19.2-40.533333-17.066667-6.4-34.133333 2.133333-40.533333 19.2-51.2 138.666667-187.733333 232.533333-339.2 232.533333C298.666667 864 138.666667 706.133333 138.666667 512S300.8 160 497.066667 160c145.066667 0 277.333333 87.466667 330.666666 217.6l-128-36.266667c-17.066667-4.266667-34.133333 6.4-38.4 23.466667-4.266667 17.066667 6.4 34.133333 23.466667 38.4l185.6 49.066667c2.133333 0 6.4 2.133333 8.533333 2.133333 6.4 0 10.666667-2.133333 17.066667-4.266667 6.4-4.266667 12.8-10.666667 14.933333-19.2l49.066667-185.6c0-17.066667-8.533333-34.133333-25.6-38.4z"
                            fill="#666666"
                            p-id="4848"
                        ></path>
                    </svg>
                </div>
                <button
                    @click="
                        ((dialogModel.isShowDialog = true),
                        (dialogModel.operation = ProxyOperation.Add))
                    "
                >
                    +添加代理
                </button>
            </div>
        </div>
        <div class="inside">
            <table width="100%" style="text-align: center">
                <thead>
                    <tr>
                        <th>名称</th>
                        <th>类型</th>
                        <th>本地地址</th>
                        <th>远程端口</th>
                        <th>状态</th>
                        <th>操作</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="item in configStore.config.proxies">
                        <td class="name">
                            <span>{{ item.name }}</span>
                        </td>
                        <td class="type">
                            <span :class="[item.type, 'typeItem']">{{
                                item.type
                            }}</span>
                        </td>
                        <td>
                            <span
                                >{{ item.localIp }}{{ item.localPort ? ":" : ""
                                }}{{ item.localPort }}</span
                            >
                        </td>
                        <td>
                            <span>{{ item.remotePort }}</span>
                        </td>
                        <td class="state">
                            <span
                                :class="item.enabled ? 'running' : 'nowork'"
                                >{{ item.enabled ? "已启用" : "未启用" }}</span
                            >
                        </td>
                        <td>
                            <div class="opration">
                                <span
                                    class="edit"
                                    @click="
                                        ((dialogModel.operation =
                                            ProxyOperation.Edit),
                                        (dialogModel.proxy = item),
                                        (dialogModel.isShowDialog = true))
                                    "
                                >
                                    <svg
                                        t="1790489577679"
                                        class="icon"
                                        viewBox="0 0 1024 1024"
                                        version="1.1"
                                        xmlns="http://www.w3.org/2000/svg"
                                        p-id="4806"
                                        width="16"
                                        height="16"
                                    >
                                        <path
                                            d="M492.8384 167.072l364.0896 364.096-491.84 491.84H0.992v-364.096l491.84-491.84z m0 185.2416l-364.0896 364.0896v178.848h178.848l364.096-364.0896L492.832 352.32zM665.2992 0.9984l357.7024 364.0896-108.5888 108.5888-364.0896-364.096L665.2992 1.0048zM640 896h384v128H512l128-128z"
                                            fill="#444444"
                                            p-id="4807"
                                        ></path>
                                    </svg>
                                </span>
                                <span
                                    class="start"
                                    v-if="!item.enabled"
                                    @click="switchProxyStatusChain(item.name)"
                                >
                                    <svg
                                        t="1790489684786"
                                        class="icon"
                                        viewBox="0 0 1024 1024"
                                        version="1.1"
                                        xmlns="http://www.w3.org/2000/svg"
                                        p-id="4818"
                                        width="25"
                                        height="25"
                                    >
                                        <path
                                            d="M383.23 271.53c-25.16 1.294-32.855 23.49-33.79 44.95l-0.9 0.73s2.192 408.877 3.43 412.14c10.237 27.506 40.219 27.511 61.65 15.53l0.56 0.22s345.098-199.46 347.91-201.71c18.281-15.019 35.094-36.958 8.6-60.47 0 0-372.092-210.038-375.58-210.6-4.338-0.731-8.286-0.975-11.88-0.79z"
                                            p-id="4819"
                                        ></path>
                                    </svg>
                                </span>
                                <span
                                    v-else
                                    class="stop"
                                    @click="switchProxyStatusChain(item.name)"
                                >
                                    <svg
                                        t="1790489772379"
                                        class="icon"
                                        viewBox="0 0 1024 1024"
                                        version="1.1"
                                        xmlns="http://www.w3.org/2000/svg"
                                        p-id="9067"
                                        width="25"
                                        height="25"
                                    >
                                        <path
                                            d="M191.397656 128.194684l191.080943 0 0 768.472256-191.080943 0 0-768.472256Z"
                                            fill="#282727"
                                            p-id="9068"
                                        ></path>
                                        <path
                                            d="M575.874261 128.194684l192.901405 0 0 768.472256-192.901405 0 0-768.472256Z"
                                            fill="#282727"
                                            p-id="9069"
                                        ></path>
                                    </svg>
                                </span>
                                <span
                                    class="del"
                                    @click="deleteProxyChain(item.name)"
                                >
                                    <svg
                                        t="1790489803086"
                                        class="icon"
                                        viewBox="0 0 1024 1024"
                                        version="1.1"
                                        xmlns="http://www.w3.org/2000/svg"
                                        p-id="4843"
                                        width="16"
                                        height="16"
                                    >
                                        <path
                                            d="M519.620465 0c-103.924093 0-188.511256 82.467721-192.083349 185.820279H85.015814A48.91386 48.91386 0 0 0 36.101953 234.686512a48.91386 48.91386 0 0 0 48.913861 48.866232h54.010046V831.345116c0 102.852465 69.822512 186.844279 155.909954 186.844279h439.200744c86.087442 0 155.909953-83.491721 155.909954-186.844279V284.100465h48.91386a48.91386 48.91386 0 0 0 48.913861-48.890046 48.91386 48.91386 0 0 0-48.913861-48.866233h-227.756651A191.559442 191.559442 0 0 0 519.620465 0z m-107.234232 177.080558c3.548279-49.771163 46.627721-88.540279 99.851907-88.540279 53.224186 0 96.327442 38.745302 99.351813 88.540279h-199.20372z m-111.997024 752.044651c-30.981953 0-65.083535-39.15014-65.083535-95.041488V287.744h575.488v546.839814c0 55.915163-34.077767 95.041488-65.059721 95.041488H300.389209v-0.500093z"
                                            fill="#2c2c2c"
                                            p-id="4844"
                                        ></path>
                                        <path
                                            d="M368.116093 796.814884c24.361674 0 44.27014-21.670698 44.27014-48.818605v-278.623256c0-27.147907-19.908465-48.818605-44.27014-48.818604-24.33786 0-44.27014 21.670698-44.27014 48.818604v278.623256c0 27.147907 19.360744 48.818605 44.293954 48.818605z m154.933581 0c24.361674 0 44.293953-21.670698 44.293954-48.818605v-278.623256c0-27.147907-19.932279-48.818605-44.293954-48.818604-24.33786 0-44.27014 21.670698-44.270139 48.818604v278.623256c0 27.147907 19.932279 48.818605 44.293953 48.818605z m132.810419 0c24.33786 0 44.27014-21.670698 44.27014-48.818605v-278.623256c0-27.147907-19.932279-48.818605-44.27014-48.818604s-44.27014 21.670698-44.27014 48.818604v278.623256c0 27.147907 19.360744 48.818605 44.27014 48.818605z"
                                            fill="#2c2c2c"
                                            p-id="4845"
                                        ></path>
                                    </svg>
                                </span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
</template>

<style lang="scss" scoped>
@use "../../assets/css/main.scss" as *;
@use "../../assets/css/proxyType.scss";

.status {
    height: 100%;
    max-height: 100%;
    overflow: hidden;
    user-select: none;
    border: 1px solid #ebebeb;
    border-radius: 6px;
    position: relative;
    box-sizing: border-box;
    padding: 1rem 0.8rem;
    display: flex;
    flex-direction: column;
    background-color: white;
    -webkit-user-select: none;
    user-select: none;

    .title {
        display: flex;
        justify-content: space-between;
        padding-bottom: 6px;

        .refresh {
            display: flex;
            align-items: center;

            &.rotating {
                animation: rotate360 0.5s ease;
            }
        }

        @keyframes rotate360 {
            from {
                transform: rotate(0deg);
            }
            to {
                transform: rotate(360deg);
            }
        }

        .btn {
            display: flex;
            gap: 1rem;

            .refresh,
            button {
                cursor: pointer;
            }
        }
    }

    h3 {
        padding: 0;
        margin: 0.2rem 0;
    }

    .inside {
        flex: 1;
        border: 1px solid #ccc;
        border-radius: 6px;
        display: flex;
        overflow: hidden;
        overflow-y: scroll;

        .typeItem {
            padding: 0.4rem 0.6rem;
            border-radius: 2px;
        }

        .type,
        .state {
            font-weight: bold;
            font-size: 14px;
        }

        table {
            // flex: 1;
            width: 100%;
            height: fit-content;
            border-spacing: 0;
            overflow: hidden;
            overflow-y: scroll;

            thead {
                th {
                    position: sticky;
                    top: 0;
                    background-color: rgb(246, 249, 252);
                    letter-spacing: 3px;
                }
                tr {
                    height: 2.4rem;
                }
            }

            tbody {
                tr:hover {
                    background-color: #f1f1f1;
                }
                .opration {
                    display: flex;
                    padding: 0 1rem;
                    align-items: center;
                    justify-content: space-around;

                    span {
                        cursor: pointer;
                    }
                }
            }

            td {
                border-top: 1px solid #e9e9e9;
                height: 3rem;
            }
            .name {
                width: 10rem;
                max-width: 10rem;
                overflow-x: scroll;
                scrollbar-width: thin;
                scrollbar-color: #00000000 #00000000;
            }
            .running {
                background-color: rgba(12, 208, 12, 0.222);
                color: rgb(14, 145, 77);
                padding: 0.4rem 0.6rem;
                border-radius: 2px;
            }
            .nowork {
                background-color: rgba(128, 129, 128, 0.222);
                color: rgb(135, 135, 135);
                padding: 0.4rem 0.6rem;
                border-radius: 2px;
            }
        }
    }
}
</style>
