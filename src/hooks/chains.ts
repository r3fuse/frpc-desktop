import {
    switchProxyStatus,
    deleteProxy,
    getConfig,
    refreshConfig,
    updateProxy,
    addProxy,
    changeServerConfig,
} from "./useTauri";
import { useConfigStore } from "../store/configStore";
import { type Config, Proxies } from "../utils/type.ts";

async function updateConfigStore() {
    const configStore = useConfigStore();
    const cfg = await getConfig();
    console.log("config", cfg);
    configStore.set_config(cfg);
}

export async function switchProxyStatusChain(proxyName: string) {
    await switchProxyStatus(proxyName);
    await updateConfigStore();
}

export async function deleteProxyChain(proxyName: string) {
    await deleteProxy(proxyName);
    await updateConfigStore();
}

export async function refreshConfigData() {
    try {
        const cfg = await refreshConfig();
        const configStore = useConfigStore();
        configStore.config = cfg;
    } catch (error) {
        console.log("refreshConfigData failed:", error);
    }
}

export async function updateProxyChain(name: string, newProxy: Proxies) {
    await updateProxy(name, newProxy);
    await updateConfigStore();
}

export async function addProxyChain(proxy: Proxies): Promise<void> {
    await addProxy(proxy);
    await updateConfigStore();
}

export async function changeServerConfigChain(config: Config) {
    await changeServerConfig(config);
    await updateConfigStore();
}
