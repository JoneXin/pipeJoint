#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::alloc::System;
use std::env;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use pipeJoint::cmds;
use pipeJoint::tray;
use pipeJoint::ProxyManager;
use tauri::Manager;
use tauri::SystemTray;
#[tokio::main]
async fn main() {
    let config_file = Path::new(&env::current_dir().unwrap()).join("proxy_list.json");
    let binding = config_file.into_os_string();
    let cwd = binding.to_str();

    if let Some(save_path) = cwd {
        println!("config save dir: {}", save_path);

        // init manager
        let proxy_manager = ProxyManager::new(save_path);
        let pm = Arc::new(Mutex::new(proxy_manager));
        // init proxy sedrver
        init_proxy_server(Arc::clone(&pm)).await;

        // init tauri
        let builder = tauri::Builder::default()
            // .menu(tauri::Menu::os_default("pipeJoint"))
            .system_tray(tray::menu())
            .on_system_tray_event(tray::handler)
            .manage(Arc::clone(&pm))
            .invoke_handler(tauri::generate_handler![
                cmds::get_proxy_list,
                cmds::buck_proxy,
                cmds::del_proxy_item,
                cmds::set_proxy_status
            ]);

        let app = builder
            .build(tauri::generate_context!())
            .expect("error while running tauri application");

        app.run(move |app_handle, e| match e {
            tauri::RunEvent::WindowEvent {
                label,
                event: win_event,
                ..
            } => match win_event {
                // 捕获 x 不结束进程，隐藏
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    let win = app_handle.get_window(label.as_str()).unwrap();
                    win.hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            },
            tauri::RunEvent::Exit => {
                app_handle.exit(0);
            }
            _ => {}
        });
    } else {
        println!("save path is None");
    }
}

/**
 * 初始化 proxy list
 */
pub async fn init_proxy_server(pm: Arc<Mutex<ProxyManager>>) {
    let proxy_manager = &mut pm.lock().unwrap();
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
