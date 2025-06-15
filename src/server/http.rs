use std::{
  net::{Ipv4Addr, SocketAddr},
  time::Duration,
};
use tokio::{net::TcpListener, time::sleep};

use crate::ResponseHtml;

pub struct HttpServer {
  ip: Ipv4Addr,
  port: u16,
}

impl HttpServer {
  pub fn new(ip: Ipv4Addr, port: u16) -> Self {
    Self { ip, port }
  }

  pub async fn run(
    &self,
  ) -> std::result::Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let addr: SocketAddr = (self.ip, self.port).into();
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    let response_html = ResponseHtml {};

    loop {
      let (stream, _) = listener.accept().await?;
      // let io =
      // unimplemented!()
      todo!()
    }

    Ok(())
  }
}
