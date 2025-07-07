use tokio::net::UdpSocket;
use std::net::SocketAddr;
use std::error::Error;
use std::sync::Arc;

pub struct TokioUdpClient {
    socket: Arc<UdpSocket>,
}

impl TokioUdpClient {
    pub async fn new(addr: SocketAddr) -> Result<Self, Box<dyn Error>>
    {
        let tokio_socket = Arc::new(UdpSocket::bind(addr).await?);
        
        Ok(TokioUdpClient { socket: tokio_socket })
    }

    pub async fn send_to(&self, buf: &[u8], target_addr: SocketAddr) -> Result<usize, Box<dyn Error>> {
        let n = self.socket.send_to(buf, target_addr).await?;
        Ok(n)
    }

    pub async fn recv_data(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr), Box<dyn Error>> {
        let (n, src_addr) = self.socket.recv_from(buf).await?;
        if n == 0 {
            return Err("No data received".into());
        }
        Ok((n, src_addr))
    }
}