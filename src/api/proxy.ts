import { invoke } from "@tauri-apps/api/tauri";
import { ProxyStruct } from "./type";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
/**
 *
 * @returns
 */
export async function greet() {
    return await invoke("greet", { name });
}

/**
 * get proxy list
 * @returns
 */
export async function getProxyList(): Promise<Array<ProxyStruct>> {
    return await invoke("get_proxy_list");
}
