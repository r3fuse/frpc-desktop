import { invoke } from "@tauri-apps/api/core";
import { Config } from "../utils/type";
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
