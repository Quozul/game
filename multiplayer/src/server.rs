use std::io;
use tokio::net::UdpSocket;
use tracing::trace;

pub struct Server {
    socket: UdpSocket,
}

impl Server {
    pub async fn new(addr: &str) -> io::Result<Self> {
        let listener = UdpSocket::bind(addr).await?;
        Ok(Server { socket: listener })
    }

    pub async fn listen(&self) -> io::Result<()> {
        let mut buf = [0; 1024];
        loop {
            let (len, addr) = self.socket.recv_from(&mut buf).await?;
            trace!("[Server] {} bytes received from {:?}", len, addr);

            let len = self.socket.send_to(&buf[..len], addr).await?;
            trace!("[Server] {} bytes sent", len);
        }
    }
}
