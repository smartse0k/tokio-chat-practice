use std::net::SocketAddr;

use tokio::{io::AsyncReadExt, net::TcpStream};

pub struct Connection {
  socket_addr: SocketAddr,
  tcp_stream: TcpStream,
}

impl Connection {
  pub fn new(socket_addr: SocketAddr, tcp_stream: TcpStream) -> Self {
    Self {
      socket_addr,
      tcp_stream
    }
  }

  pub fn bind_handler(&mut self) {

  }

  pub async fn process(&mut self) {
    loop {
      let Ok(length) = self.tcp_stream.read_u16().await else {
        println!("[ERROR] failed to receive length.");
        break;
      };
      
      let mut data = vec![0; length as usize];

      let read_size = self.tcp_stream
        .read_exact(&mut data)
        .await
        .unwrap_or_else(|err| {
          println!("[ERROR] can not receive exactly. err={:?}", err);
          0
        });
      
      if read_size == 0 {
        continue;
      }

      println!("data={:?}", data);
    }
  }
}
