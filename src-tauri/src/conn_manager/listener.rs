use crate::conn_manager::connector::Connection;
use std::error::Error;

use crate::conn_manager::handler::handle;
use tokio::net::TcpListener;
use tracing::info;

pub struct HttpListener<'a> {
    listen_conn: Connection<'a>,
    forward_conn: Connection<'a>,
}

impl<'a> HttpListener<'a> {
    pub fn new(listen_conn: Connection<'a>, forward_conn: Connection<'a>) -> HttpListener<'a> {
        HttpListener {
            listen_conn,
            forward_conn,
        }
    }

    pub async fn listen(&self) -> Result<(), Box<dyn Error>> {
        let addr = format!("{}:{}", self.listen_conn.host, self.listen_conn.port);
        let listener = TcpListener::bind(&addr).await.unwrap();

        while let Ok((inbound, _)) = listener.accept().await {
            let proxy_addr = format!("{}:{}", self.forward_conn.host, self.forward_conn.port);

            tokio::spawn(async move {
                match handle(inbound, &proxy_addr.clone()).await {
                    Ok(_) => info!("success"),
                    Err(e) => info!("error: {}", e),
                }
            });
        }

        Ok(())
    }
}
