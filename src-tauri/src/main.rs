// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use pipeJoint::{storage::JsonStorage, types::ProxyStruct};

// 获取转发列表
#[tauri::command]
fn get_proxy_list(state: tauri::State<JsonStorage>) -> Vec<ProxyStruct> {
    state.get_storage_info()
}

// buckCreate
#[tauri::command]
fn buck_proxy(state: tauri::State<JsonStorage>, proxy_config: ProxyStruct) -> Result<bool, String> {
    let proxy_list = state.get_storage_info();
    let mut is_upadate = false;

    let mut new_proxy_list = proxy_list
        .into_iter()
        .map(|item| {
            if item.key == proxy_config.key {
                is_upadate = true;
                proxy_config.clone()
            } else {
                item
            }
        })
        .collect::<Vec<ProxyStruct>>();

    if !is_upadate {
        new_proxy_list.push(proxy_config)
    }

    // save to config file
    let res = state.save_storage(new_proxy_list);
    if let Ok(_) = res {
        Ok(true)
    } else {
        Err(String::from("保存异常!"))
    }
}

// 编辑转发
#[tauri::command]
fn edit_proxy_item(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 删除转发
#[tauri::command]
fn del_proxy_item(state: tauri::State<JsonStorage>, key: String) -> Result<bool, String> {
    let proxy_list = state.get_storage_info();

    // new proxy list
    let new_proxy_list = proxy_list
        .into_iter()
        .filter(|item| item.key != key)
        .collect();

    // save to config file
    let res = state.save_storage(new_proxy_list);
    if let Ok(_) = res {
        Ok(true)
    } else {
        Err(String::from("保存异常!"))
    }
}

// 测试连接
#[tauri::command]
fn test_connection(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 设置 proxy 状态
#[tauri::command]
fn set_proxy_status(
    state: tauri::State<JsonStorage>,
    key: String,
    status: String,
) -> Result<bool, String> {
    let proxy_list = state.get_storage_info();

    // new proxy list
    let new_proxy_list = proxy_list
        .into_iter()
        .map(|item| {
            if item.key == key {
                ProxyStruct {
                    status: status.clone(),
                    ..item
                }
            } else {
                item
            }
        })
        .collect();

    // save to config file
    let res = state.save_storage(new_proxy_list);
    if let Ok(_) = res {
        Ok(true)
    } else {
        Err(String::from("保存异常!"))
    }
}

fn main() {
    // init config
    let proxy_conf = JsonStorage::new(String::from("../proxy_list.json"));

    tauri::Builder::default()
        .manage(proxy_conf)
        .invoke_handler(tauri::generate_handler![
            get_proxy_list,
            buck_proxy,
            del_proxy_item,
            set_proxy_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
