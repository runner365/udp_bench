use tokio::net::UdpSocket;
use std::net::SocketAddr;

pub struct UdpServer {
    socket: UdpSocket,
}

impl UdpServer {
    pub async fn new(addr: SocketAddr) -> Result<Self, Box<dyn std::error::Error>>
    {
        let socket = UdpSocket::bind(addr).await?;

        println!("UDP server listening on {}", addr);
        Ok(UdpServer {
            socket,
        })
    }

    pub async fn read_data(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr), Box<dyn std::error::Error>> {
        let (n, src_addr) = self.socket.recv_from(buf).await?;
        Ok((n, src_addr))
    }

    pub async fn send_data(&self, buf: &[u8], target_addr: SocketAddr) -> Result<usize, Box<dyn std::error::Error>> {
        let n = self.socket.send_to(buf, target_addr).await?;
        Ok(n)
    }
}