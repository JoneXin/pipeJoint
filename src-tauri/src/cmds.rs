use tauri::Error;
use tracing::{debug, error, info, instrument};

use crate::{types::ProxyStruct, ProxyManager};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};
pub type AppState = Arc<Mutex<ProxyManager>>;

// 获取转发列表
#[tauri::command]
pub fn get_proxy_list(state_mux: tauri::State<AppState>) -> Vec<ProxyStruct> {
    let proxy_manager = state_mux.lock().unwrap();
    proxy_manager.get_porxy_list()
}

// buckCreate
#[tauri::command]
pub fn buck_proxy(
    state_mux: tauri::State<AppState>,
    proxy_config: ProxyStruct,
) -> Result<bool, Error> {
    let state = state_mux.deref().clone();
    let proxy_manager = &mut state.lock().unwrap();

    // save to config file
    let res = proxy_manager.new_proxy(proxy_config);
    if let Ok(_) = res {
        Ok(true)
    } else {
        Err(Error::ApiNotAllowlisted("sad".to_string()))
    }
}

// 删除转发
#[tauri::command]
pub fn del_proxy_item(state_mux: tauri::State<AppState>, key: String) -> Result<bool, Error> {
    let proxy_manager = state_mux.lock().unwrap();
    let proxy_list = proxy_manager.get_porxy_list();

    let mut s_port = 0;
    // new proxy list
    let new_proxy_list = proxy_list
        .into_iter()
        .filter(|item| {
            if item.key == key {
                s_port = item.source_port;
                return false;
            }
            true
        })
        .collect();

    // save to config file
    let res = proxy_manager.storage.save_storage(new_proxy_list);

    if let Ok(_) = res {
        // kill 线程
        proxy_manager.del_proxy(key);
        Ok(true)
    } else {
        Err(Error::ApiNotAllowlisted("sad".to_string()))
    }
}

// 测试连接
#[instrument]
#[tauri::command]
pub fn test_connection(key: &str) -> String {
    info!("test logs");
    debug!("test logs");
    error!("test logs");
    println!("test logs");
    format!("Hello, {}! You've been greeted from Rust!", key)
}

// 设置 proxy 状态
#[tauri::command]
pub fn set_proxy_status(
    state_mux: tauri::State<AppState>,
    key: String,
    status: String,
) -> Result<bool, Error> {
    let proxy_manager = &mut state_mux.lock().unwrap();

    let proxy_list = proxy_manager.get_porxy_list();

    // new proxy list
    let new_proxy_list = proxy_list
        .into_iter()
        .map(|item| {
            if item.key == key {
                if status == "stoping" {
                    proxy_manager.del_proxy(key.clone());
                } else {
                    let _ = proxy_manager.init_proxy(item.clone());
                }

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
    let res = proxy_manager.storage.save_storage(new_proxy_list);
    if let Ok(_) = res {
        Ok(true)
    } else {
        Err(Error::ApiNotAllowlisted("sad".to_string()))
    }
}
