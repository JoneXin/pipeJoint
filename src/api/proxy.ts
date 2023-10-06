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
export async function getProxyList(): Promise<Array<Required<ProxyStruct>>> {
    return await invoke("get_proxy_list");
}

/**
 * get proxy list
 * @returns
 */
export async function buckProxy(proxyConfig: ProxyStruct): Promise<boolean> {
    console.log(proxyConfig);
    return await invoke("buck_proxy", { proxyConfig });
}

/**
 * 删除
 * @param key
 * @returns
 */
export async function delProxy(key: string): Promise<boolean> {
    return await invoke("del_proxy_item", { key });
}

/**
 * 设置proxy 代理状态
 * @param key
 * @param status
 * @returns
 */
export async function proxyStatus(
    key: string,
    status: string
): Promise<boolean> {
    return await invoke("set_proxy_status", { key, status });
}

/**
 * 设置proxy 代理状态
 * @param key
 * @param status
 * @returns
 */
export async function testConnection(key: string): Promise<boolean> {
    return await invoke("test_connection", { key });
}
