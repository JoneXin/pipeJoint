// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use pipeJoint::types::ProxyStruct;

// 获取转发列表
#[tauri::command]
fn get_proxy_list() -> Vec<ProxyStruct<'static>> {
    vec![ProxyStruct {
        sourceIp: "127.0.0.1",
        sourcePort: 3398,
        targetIp: "127.0.0.1",
        targetPort: 3398,
        protocol: "TCP",
        status: "stoping",
        key: "1",
    }]
}

// 新增转发
#[tauri::command]
fn addProxyItem(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_proxy_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
