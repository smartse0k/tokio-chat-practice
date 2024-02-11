use std::process::exit;
use tokio::net::TcpListener;
use crate::network::connection::Connection;

pub struct Server {
  address: String,
  port: u16,
}

impl Server {
  pub fn new(address: &String, port: u16) -> Self {
    Self {
      address: address.clone(),
      port,
    }
  }

  pub async fn start(&mut self) {
    let bind_address = format!("{}:{}", &self.address, self.port);

    let Ok(tcp_listener) = TcpListener::bind(&bind_address).await else {
      println!("[ERROR] can not bind address. bind_address={}", &bind_address);
      exit(1);
    };

    println!("[INFO] server started. bind_address={}", &bind_address);
    
    loop {
      let Ok((tcp_stream, socket_addr)) = tcp_listener.accept().await else {
        println!("[ERROR] failed to accept incoming socket.");
        exit(1);
      };

      println!("[INFO] client connected. address={}", socket_addr.to_string());

      let mut connection: Connection = Connection::new(socket_addr, tcp_stream);
      connection.bind_handler();

      tokio::spawn(async move {
        connection.process().await;
      });
    }
  }
}
