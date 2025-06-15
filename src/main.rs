use std::net::Ipv4Addr;

use command_lines::HttpServer;

#[tokio::main]
async fn main() -> ::std::result::Result<(), Box<dyn ::std::error::Error + Sync + Send>> {
  let http_server = HttpServer::new(Ipv4Addr::new(127, 0, 0, 1), 8081);

  let run_handle = http_server.run();

  println!("start");
  run_handle.await?;
  println!("end");

  Ok(())
}
