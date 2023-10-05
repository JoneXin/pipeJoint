use std::{fs, io, path::Path};

use serde::de::Error;

use crate::types::ProxyStruct;

#[derive(Debug)]
pub struct JsonStorage {
    path: String,
}

impl JsonStorage {
    /**
     * 初始化所有代理
     */
    pub fn new(path: String) -> JsonStorage {
        // 是否存在文件
        let proxy_path = Path::new(path.as_str());
        let exist = proxy_path.exists();

        if !exist {
            fs::write(&path, "[]").unwrap();
        }

        JsonStorage { path }
    }

    /**
     * 获取存储信息
     */
    pub fn get_storage_info(&self) -> Vec<ProxyStruct> {
        let proxy_json = fs::read_to_string(self.path.as_str()).unwrap();
        println!("{}", proxy_json);

        let proxy_list: Vec<ProxyStruct> = serde_json::from_str(proxy_json.as_str()).unwrap();
        return proxy_list;
    }

    /**
     * 保存配置
     */
    pub fn save_storage(&self, config: Vec<ProxyStruct>) -> Result<(), io::Error> {
        let proxy_list = serde_json::to_string(&config)?;
        fs::write(self.path.as_str(), proxy_list)?;
        Ok(())
    }

    /**
     * 新增配置
     */
    pub fn add_proxy_item(&self, config: ProxyStruct) -> Result<(), io::Error> {
        let mut proxy_list = self.get_storage_info();
        proxy_list.push(config);
        self.save_storage(proxy_list)
    }
}
