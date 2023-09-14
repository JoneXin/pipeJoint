// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use pipeJoint::{storage::JsonStorage, types::ProxyStruct};

// 获取转发列表
#[tauri::command]
fn get_proxy_list(state: tauri::State<JsonStorage>) -> Vec<ProxyStruct> {
    state.get_storage_info()
}

// 新增转发
#[tauri::command]
fn add_proxy_item(
    state: tauri::State<JsonStorage>,
    proxy_config: ProxyStruct,
) -> Result<bool, String> {
    let res = state.add_proxy_item(proxy_config);
    if let Ok(_) = res {
        Ok(true)
    } else {
        Err(String::from("新增失败!"))
    }
}

// 编辑转发
#[tauri::command]
fn editProxyItem(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 删除转发
#[tauri::command]
fn deleteProxyItem(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 测试连接
#[tauri::command]
fn testConnection(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // init config
    let proxy_conf = JsonStorage::new(String::from("./proxy_list.json"));

    tauri::Builder::default()
        .manage(proxy_conf)
        .invoke_handler(tauri::generate_handler![get_proxy_list, add_proxy_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
