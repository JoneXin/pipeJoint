#[derive(serde::Serialize)]
pub struct ProxyStruct<'a> {
    pub sourceIp: &'a str,
    pub sourcePort: u32,
    pub targetIp: &'a str,
    pub targetPort: u32,
    pub protocol: &'a str,
    pub status: &'a str,
    pub key: &'a str,
}
