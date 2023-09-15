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
    pub fn new(p: Protocol, t: Transport, host: &str, port: u32) -> Connection {
        Connection {
            protocol: p,
            transport: t,
            host: host,
            port: port,
        }
    }

    pub fn display(&self) {
        let p = match self.protocol {
            Protocol::Default => "http",
            Protocol::Http => "http",
            Protocol::Http2 => "http2",
        };

        let t = match self.transport {
            Transport::Default => "tcp",
            Transport::Tcp => "tcp",
            Transport::Tls => "tls",
        };

        println!(
            "protocol: {}, transport:{}, host: {}, port: {}",
            p, t, self.host, self.port
        );
    }
}
