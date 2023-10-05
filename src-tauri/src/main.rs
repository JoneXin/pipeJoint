#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use pipeJoint::cmds;
use pipeJoint::ProxyManager;

#[tokio::main]
async fn main() {
    let config_file = Path::new(&env::current_dir().unwrap()).join("proxy_list.json");
    let binding = config_file.into_os_string();
    let cwd = binding.to_str();
    if let Some(save_path) = cwd {
        println!("config save dir: {}", save_path);

        // init manager
        let mut proxy_manager = ProxyManager::new(save_path);

        // init proxy sedrver
        init_proxy_server(&mut proxy_manager).await;

        // init tauri
        tauri::Builder::default()
            .manage(Arc::new(Mutex::new(proxy_manager)))
            .invoke_handler(tauri::generate_handler![
                cmds::get_proxy_list,
                cmds::buck_proxy,
                cmds::del_proxy_item,
                cmds::set_proxy_status
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    } else {
        println!("save path is None");
    }
}

/**
 * 初始化 proxy list
 */
pub async fn init_proxy_server(proxy_manager: &mut ProxyManager) {
    let proxy_list = proxy_manager.get_porxy_list();

    for item in proxy_list {
        if item.status == "stoping" {
            break;
        };

        let status = proxy_manager.init_proxy(item.clone());
        if let Ok(_) = status {
            println!("{:#?} 端口启动代理", item);
        } else {
            println!("{:#?}", status);
        }
    }
}
