import { invoke } from "@tauri-apps/api/core";
import { Config, Proxies } from "../utils/type";
import { Event, listen, UnlistenFn } from "@tauri-apps/api/event";

/**获得后端日志*/
export async function frpLogsListener(
    cb: (e: Event<string>) => void,
): Promise<UnlistenFn> {
    return listen("frp-log", cb);
}

/**获取后端读取的配置文件*/
export async function getConfig(): Promise<Config> {
    return await invoke("get_config");
}

export async function deleteProxy(proxyName: string) {
    console.log(proxyName);
    await invoke("delete_config", { proxyName: proxyName });
}

/**启动frpc进程 */
export async function startFRP() {
    return await invoke("start_frp");
}

/**kill frpc进程 */
export async function stopFRP() {
    return await invoke("stop_frp");
}

/**切换单个代理的enable状态 */
export async function switchProxyStatus(proxyName: string) {
    return await invoke("change_activation_status", { name: proxyName });
}

export async function refreshConfig(): Promise<Config> {
    const cfg = (await invoke("refresh_config")) as Config;
    return cfg;
}

//更新单个代理数据
export async function updateProxy(name: string, newProxy: Proxies) {
    return await invoke("change_config_by_name", {
        name: name,
        proxy: newProxy,
    });
}

//添加代理
export async function addProxy(proxy: Proxies): Promise<void> {
    await invoke("add_proxy", { proxy: proxy });
}

//更改服务器配置
export async function changeServerConfig(config: Config) {
    return invoke("change_server", {
        newConfig: {
            serverAddr: config.serverAddr,
            serverPort: config.serverPort,
            auth: config.auth,
            transport: { ...config?.transport },
        },
    });
}
