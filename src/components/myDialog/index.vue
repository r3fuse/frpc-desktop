<script setup lang="ts">
import { ref, useTemplateRef, watch } from "vue";
import { Proxies, ProxyOperation } from "../../utils/type";
import { updateProxyChain, addProxyChain } from "../../hooks/chains";
import { notification } from "../../utils/notification";

const model = defineModel<{
    proxy: Proxies;
    operation: ProxyOperation;
    isShowDialog: boolean;
}>();

const defaultForm: Proxies = {
    name: "",
    type: "tcp",
    enabled: true,
};
const form = ref<Proxies>({ ...defaultForm });

const myDialog = useTemplateRef<HTMLDialogElement>("myDialog");

function updateProxy(name: string, newProxy: Proxies) {
    updateProxyChain(name, newProxy);
    model.value!.isShowDialog = false;
}

async function saveProxy(proxy: Proxies) {
    try {
        await addProxyChain(proxy);
        model.value!.isShowDialog = false;
    } catch (error) {
        console.log("catch inside");
        if (error instanceof Error) {
            console.log("属于 Error", error);
            notification({
                msg_type: "Error",
                message: error.message,
                title: "error",
            });
        }
        if (typeof error == "string") {
            console.log("属于 string", error);
            notification({
                msg_type: "Error",
                message: error,
                title: "error",
            });
        }
    }
}
watch(
    () => model.value?.isShowDialog,
    (show) => {
        if (show) {
            if (model.value?.operation == ProxyOperation.Edit) {
                form.value = { ...model.value.proxy };
            } else {
                form.value = { ...defaultForm };
            }
            myDialog.value?.showModal();
        } else {
            myDialog.value?.close();
        }
    },
);
</script>

<template>
    <Teleport to="body">
        <dialog ref="myDialog" class="myDialog">
            <div class="title">
                <h2>
                    {{
                        model?.operation == ProxyOperation.Edit
                            ? "编辑配置"
                            : "添加配置"
                    }}
                </h2>
                <span class="closeDialog" @click="model!.isShowDialog = false">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="1em"
                        height="1em"
                        viewBox="0 0 24 24"
                    >
                        <path
                            fill="currentColor"
                            d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12z"
                        />
                    </svg>
                </span>
            </div>
            <div class="main">
                <h3>基础配置</h3>
                <p>代理名称：</p>
                <input type="text" v-model="form.name" />
                <p>内网地址：</p>
                <input type="text" v-model="form.localIp" />
                <p>内网端口：</p>
                <input
                    type="number"
                    max="65536"
                    min="1"
                    v-model.number="form.localPort"
                />
                <p>代理类型：</p>
                <select v-model="form.type">
                    <option value="tcp">TCP</option>
                    <option value="udp">UDP</option>
                    <option value="http">HTTP</option>
                    <option value="https">HTTPS</option>
                </select>
                <div>
                    <h3>远程配置</h3>
                    <p>远程端口</p>
                    <input
                        type="number"
                        max="65536"
                        min="1"
                        v-model.number="form.remotePort"
                        :disabled="form.type.includes('http')"
                    />
                </div>
                <div class="high">
                    <h3>高级配置</h3>
                    <p>是否启用</p>
                    <button
                        class="switch"
                        :class="{ active: form.enabled }"
                        @click="form.enabled = !form.enabled"
                    >
                        <span class="dot"></span>
                    </button>
                </div>
            </div>
            <div class="bottom">
                <button
                    @click="
                        model?.operation == ProxyOperation.Add
                            ? saveProxy(form)
                            : updateProxy(model!.proxy.name, form)
                    "
                >
                    {{
                        model?.operation == ProxyOperation.Add ? "添加" : "保存"
                    }}
                </button>
            </div>
        </dialog>
    </Teleport>
</template>

<style lang="scss" scoped>
.myDialog {
    position: fixed;
    top: 0;
    z-index: 9;
    width: 36%;
    height: 72%;
    border: 0;
    border-radius: 10px;
    box-shadow: 0px 0px 4px 1px #ccc;
    -webkit-user-select: none;
    user-select: none;

    .title {
        display: flex;
        justify-content: space-between;
        border-bottom: 2px solid black;
        h2 {
            margin-bottom: 0.6rem;
        }
        span {
            cursor: pointer;
        }
        .closeDialog {
            display: flex;
            width: fit-content;
            height: fit-content;
            padding: 0.2rem;

            &:hover {
                color: rgb(206, 67, 67);
                background-color: #ebebeb;
            }
        }
    }
    .main {
        padding: 0 1rem;

        input {
            width: 100%;
        }
        h3 {
            margin-bottom: 0;
        }
        p {
            margin: 0;
            margin-top: 6px;
        }
        .high {
            .switch {
                width: 50px;
                height: 26px;
                padding: 2px;
                border: none;
                border-radius: 14px;
                background: #ccc;
                cursor: pointer;

                .dot {
                    display: block;
                    width: 22px;
                    height: 22px;
                    border-radius: 50%;
                    background: white;

                    transform: translateX(0);
                    transition: transform 200ms ease;
                }

                &.active {
                    background: #409eff;

                    .dot {
                        transform: translateX(24px);
                    }
                }
            }
        }
    }

    .bottom {
        display: flex;
        justify-content: right;
    }
}
</style>
