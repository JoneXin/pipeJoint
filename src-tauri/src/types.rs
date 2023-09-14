use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProxyStruct {
    pub source_ip: String,
    pub source_port: u32,
    pub target_ip: String,
    pub target_port: u32,
    pub protocol: String,
    pub status: String,
    pub key: String,
}
