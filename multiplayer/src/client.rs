use ciborium::value::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io;
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::net::UdpSocket;
use tracing::trace;

pub struct Client {
    socket: UdpSocket,
    server_address: SocketAddr,
}

impl Client {
    pub async fn connect(addr: &str) -> io::Result<Self> {
        let server_address = addr.to_socket_addrs()?.nth(0).unwrap();
        let socket = UdpSocket::bind("127.0.0.1:0").await?;
        Ok(Client {
            socket,
            server_address,
        })
    }

    pub async fn send_bytes(&mut self, data: &[u8]) -> io::Result<()> {
        let len = self.socket.send_to(data, &self.server_address).await?;
        trace!("[Client] {} bytes sent", len);
        Ok(())
    }

    pub async fn send_serialize<T: Serialize>(
        &mut self,
        data: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        ciborium::into_writer(data, &mut buf)?;
        self.send_bytes(&buf).await?;
        Ok(())
    }

    pub async fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.socket.recv(buf).await?;
        trace!("[Client] {} bytes received", len);
        Ok(len)
    }

    pub async fn read_serialize<T: DeserializeOwned>(
        &mut self,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let mut buf = [0; 1024];
        let n = self.read_bytes(&mut buf).await?;
        if n > 0 {
            let sub = &buf[..n];
            let data = ciborium::from_reader(sub)?;
            Ok(data)
        } else {
            Err(Box::new(Error::Custom("No bytes received".to_string())))
        }
    }
}
