<script setup lang="ts">
import { onMounted } from "vue";
import { useConfigStore } from "./store/configStore";
import { useStatusStore } from "./store/statusStore";
import { getConfig } from "./hooks/useTauri";
import { frpLogsListener } from "./hooks/useTauri";
import { WorkStatus } from "./utils/statue";

const configStore = useConfigStore();
const statusStore = useStatusStore();
if (import.meta.env.PROD) {
    window.addEventListener("contextmenu", (e) => e.preventDefault(), true);
}

/**初始化configStore仓库 */
async function init_configstroe(): Promise<void> {
    const data = await getConfig();
    configStore.set_config(data);
}

function stripAnsi(str: string) {
    return str.replace(/\x1B\[[0-9;]*[mK]/g, "");
}

onMounted(async () => {
    await init_configstroe();
    frpLogsListener((event) => {
        console.log("原始负载" + event.payload);
        let tempStr = stripAnsi(event.payload as string);
        console.log("tempStr" + tempStr);
        if (tempStr.includes("[W]")) {
            statusStore.changeFRPStatus(WorkStatus.warn);
        }
        if (tempStr.includes("[E]") || tempStr.includes("server failed")) {
            statusStore.changeFRPStatus(WorkStatus.error);
        }
        statusStore.addLogs(tempStr);
    });
});
</script>

<template>
    <div class="app">
        <!-- <Bar/> -->
        <div class="view">
            <RouterView />
        </div>
    </div>
</template>

<style scoped>
.app {
    height: 100vh;
    display: flex;
    overflow: hidden;
}
.view {
    flex: 1;
}
</style>
<style>
body {
    margin: 0;
}
:root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: rgb(59, 64, 84);

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
}
</style>
