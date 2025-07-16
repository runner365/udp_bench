use udp_bench::udp_server::UdpServer;
use udp_bench::stream_statics::StreamStatics;
use std::net::SocketAddr;

struct UdpEchoServer {
    s : UdpServer,
    statics: StreamStatics,
}

impl UdpEchoServer {
    pub async fn bind(addr : SocketAddr) -> Result<UdpEchoServer, Box<dyn std::error::Error>> {
        let server = UdpServer::new(addr).await?;
        
        let echo_server = UdpEchoServer {
            s: server,
            statics: StreamStatics::new(),
        };
        Ok(echo_server)
    }

    pub async fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut last_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("时间回溯")
            .as_millis() as i64;

        loop {
            let mut buf = vec![0u8; 1500];
            let (n, src_addr) = self.s.read_data(&mut buf).await?;
            
            if n == 0 {
                continue; // No data received, skip to next iteration
            }
            self.statics.add_recv_bytes(n as usize);
            let send_n = self.s.send_data(&buf[..n], src_addr).await?;
            if send_n != n {
                eprintln!("Failed to send all data back, sent {} bytes instead of {}", send_n, n);
            } else {
                // println!("Echoed {} bytes back to {}", n, src_addr);
                self.statics.add_send_bytes(n as usize);
            }
            let now_ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("时间回溯")
                .as_millis() as i64;

            if now_ms - last_ms >= 3*1000 {
                last_ms = now_ms;
                let (recv_kbps, recv_pps) = self.statics.get_recv_statics();
                let (send_kbps, send_pps) = self.statics.get_send_statics();
                println!("Recv: {} kbps, {} pps | Send: {} kbps, {} pps",
                    recv_kbps, recv_pps, send_kbps, send_pps);
            }
        }
    }
}

fn get_config(args : &Vec<String>) -> Result<(String, u16), String> {
    let mut iter = args.iter();
    iter.next(); // Skip the first argument (program name)

    let mut ip : String = String::new();
    let mut port : u16 = 0;

    loop {
        let ret = iter.next();
        match ret {
            Some(item) => {
                
                match item.as_str() {
                    "-l" => {
                        if let Some(ip_item) = iter.next() {
                            ip = ip_item.clone();
                        } else {
                            return Err("Missing IP address after -l".to_string());
                        }
                    }
                    "-p" => {
                        if let Some(port_item) = iter.next() {
                            match port_item.parse::<u16>() {
                                Ok(p) => {
                                    port = p;
                                }
                                Err(_) => {
                                    return Err("Invalid port number".to_string());
                                }
                            }
                        } else {
                            return Err("Missing port number after -p".to_string());
                        }
                    }
                    _ => {
                        return Err(format!("Usage: {} -l <ip> -p <port>", args[0]));
                    }
                }
            }
            None => {
                if ip.is_empty() || port == 0 {
                    return Err("Not enough arguments provided".to_string());
                } else {
                    return Ok((ip, port));
                }
            }
        }

    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let result = get_config(&args);
    let (ip, port) = match result {
        Ok((ip, port)) => (ip, port),
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            return;
        }
    };

    let addr = format!("{}:{}", ip, port);
    let socket_addr: SocketAddr = addr.parse().expect("Invalid address format");

    println!("Starting UdpServer on {}", socket_addr);

    let echo_ser = match UdpEchoServer::bind(socket_addr).await {
        Ok(server) => server,
        Err(e) => {
            eprintln!("Failed to bind UdpEchoServer: {}", e);
            return;
        }
        
    };
    if let Err(e) = echo_ser.run().await {
        eprintln!("Error running UdpEchoServer: {}", e);
    }
    println!("UdpEchoServer stopped.");

}
