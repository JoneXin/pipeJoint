pub enum Protocol {
    Default,
    Http,
    Http2,
}
pub enum Transport {
    Default,
    Tcp,
    Tls,
}

// 监听或者转发的链接
pub struct Connection<'a> {
    pub protocol: Protocol,   // 通信协议
    pub transport: Transport, // 传输协议
    pub host: &'a str,        // 主机（ip或者域名）
    pub port: u32,            // 端口
}

impl<'a> Connection<'a> {
    pub fn new(host: &str, port: u32) -> Connection {
        Connection {
            protocol: Protocol::Default,
            transport: Transport::Tcp,
            host: host,
            port: port,
        }
    }
}
