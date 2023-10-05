pub mod cmds;
pub mod conn_manager;
pub mod storage;
pub mod tray;
pub mod types;
pub mod utils;

use std::{collections::HashMap, error::Error};
use storage::JsonStorage;
use tokio::{spawn, task::JoinHandle};

use crate::{
    conn_manager::{
        connector::{Connection, Protocol},
        listener::HttpListener,
    },
    types::ProxyStruct,
};

pub struct ProxyManager {
    pub tcp_proxy_map: HashMap<String, JoinHandle<()>>,
    pub storage: JsonStorage,
}

impl ProxyManager {
    pub fn new(save_path: &str) -> ProxyManager {
        let storage = JsonStorage::new(save_path.to_string());

        ProxyManager {
            tcp_proxy_map: HashMap::new(),
            storage,
        }
    }

    pub fn init_proxy(&mut self, proxy_conf: ProxyStruct) -> Result<bool, Box<dyn Error>> {
        let ProxyStruct {
            source_ip: s_ip,
            source_port: s_port,
            target_ip: t_ip,
            target_port: t_port,
            key,
            ..
        } = proxy_conf.clone();

        // create proxy
        let handler = spawn(async move {
            let l_conn: Connection<'_> = Connection::new(&s_ip, s_port);

            let f_conn = Connection::new(&t_ip, t_port);

            let l = match l_conn.protocol {
                Protocol::Http => HttpListener::new(l_conn, f_conn),
                _ => HttpListener::new(l_conn, f_conn),
            };

            println!("{} init success", s_port);
            _ = l.listen().await;
        });

        self.tcp_proxy_map.insert(key, handler);

        Ok(true)
    }

    pub fn new_proxy(&mut self, proxy_conf: ProxyStruct) -> Result<bool, Box<dyn Error>> {
        let proxy_list: Vec<ProxyStruct> = self.get_porxy_list();

        let mut is_upadate = false;

        let mut new_proxy_list = proxy_list
            .into_iter()
            .map(|item| {
                if item.key == proxy_conf.key.clone() {
                    is_upadate = true;
                    proxy_conf.clone()
                } else {
                    item
                }
            })
            .collect::<Vec<ProxyStruct>>();

        if !is_upadate {
            new_proxy_list.push(proxy_conf.clone())
        }

        // save to file
        self.storage.save_storage(new_proxy_list)?;

        if is_upadate && proxy_conf.status == "starting" {
            // kill old proxy
            self.del_proxy(proxy_conf.key.clone());
            // create proxy
            self.init_proxy(proxy_conf.clone())?;
        }

        Ok(true)
    }

    pub fn del_proxy(&self, key: String) {
        let res = self.tcp_proxy_map.get(&key);

        if let Some(handler) = res {
            handler.abort();
            println!("已停止 key: {} 对应代理", key);
        } else {
            println!("{}", "无此端口对应的代理");
        }
    }

    pub fn get_porxy_list(&self) -> Vec<ProxyStruct> {
        self.storage.get_storage_info()
    }

    pub fn stop_all_proxy(&mut self) {
        self.tcp_proxy_map.iter().for_each(|(_, handler)| {
            handler.abort();
        });
        println!("全部停止");
    }

    pub fn get_proxy_status(&self, key: String) -> bool {
        let res = self.tcp_proxy_map.get(&key);

        if let Some(handler) = res {
            !handler.is_finished()
        } else {
            println!("{}", "无此端口对应的代理");
            false
        }
    }
}
