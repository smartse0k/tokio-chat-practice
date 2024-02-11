use std::process::exit;

use chat::network::server::Server;
use tokio::runtime::Runtime;

fn main() {
  let mut server = Server::new(
    &"127.0.0.1".to_string(),
    10000
  );

  let Ok(tokio_runtime) = Runtime::new() else {
    println!("[ERROR] can not create tokio runtime.");
    exit(1);
  };

  tokio_runtime.block_on(async {
    server.start().await;
  });
}
