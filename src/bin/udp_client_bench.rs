use std::net::SocketAddr;
use std::error::Error;
use udp_bench::common::*;
use std::env;
use std::sync::Arc;
use tokio::net::UdpSocket;
use std::sync::atomic::{AtomicI64, AtomicBool, Ordering};
use tokio::time::{sleep, Duration};
use std::sync::LazyLock;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Serialize)]
struct RttStats {
    ts: i64,
    rtt: f32,
    avg_rtt: f32,
    seq: u32,
}

#[derive(Serialize)]
struct TotalStats {
    sent: i64,
    received: i64,
    lost: i64,
    lost_percent: f32,
}

fn total_stats_to_json(
    sent: i64,
    received: i64,
    lost: i64,
    lost_percent: f32,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let stats = TotalStats {
        sent,
        received,
        lost,
        lost_percent,
    };

    let json_str = serde_json::to_string(&stats)?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;

    writeln!(file, "{}", json_str)?;

    Ok(())
}

fn append_rtt_stats_to_json(
    ts: i64,
    rtt: f32,
    avg_rtt: f32,
    seq: u32,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let stats = RttStats {
        ts,
        rtt,
        avg_rtt,
        seq,
    };

    let json_str = serde_json::to_string(&stats)?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;

    writeln!(file, "{}", json_str)?;

    Ok(())
}

static RUNNING: LazyLock<Arc<AtomicBool>> = LazyLock::new(|| {
    Arc::new(AtomicBool::new(true))
});

fn handle_data(data:&[u8], avg_rtt: &mut f32, rtt_dbg_ms: &mut i64) {
    let seq_data = &data[0..4];
    let ts_data = &data[4..8];
    let now_ms = get_millis_as_i64() as u32;
    let seq = match bytes_to_u32(seq_data) {
        Ok(n) => n,
        Err(e) => {
            println!("get seq item error:{e}");
            return;
        },
    };
    let ts = match bytes_to_u32(ts_data) {
        Ok(n) => n,
        Err(e) => {
            println!("get ts item error:{e}");
            return;
        },
    };
    
    let rtt = (now_ms - ts) as f32;
    if *avg_rtt == 0.0 {
        *avg_rtt = rtt;
    } else {
        *avg_rtt += (rtt - *avg_rtt) / 5.0;
    }
    
    if *rtt_dbg_ms == 0 {
        *rtt_dbg_ms = get_millis_as_i64();
    } else {
        let now_ms = get_millis_as_i64();
        if now_ms - *rtt_dbg_ms > 2*1000 {
            *rtt_dbg_ms = now_ms;
            println!("response rtt:{} seq:{}, ts:{}, avg_rtt:{}", rtt, seq, ts as u32, avg_rtt);
            append_rtt_stats_to_json(
                now_ms,
                rtt,
                *avg_rtt,
                seq,
                "rtt_stats.json"
            ).expect("Failed to append RTT stats to JSON");
        }
    }

}

async fn recv_udp_data_select(recv_client: Arc<UdpSocket>, running_clone : Arc<AtomicBool>) -> Result<i64, Box<dyn Error>> {
    let mut avg_rtt: f32 = 0.0;
    let mut rtt_dbg_ms: i64 = 2000;
    let mut total : i64 = 0;

    loop {
        let mut buf = vec![0u8; 1500];
        tokio::select! {
            result = recv_client.recv_from(buf.as_mut_slice()) => {
                match result {
                    Ok((n, _)) => {
                        if n > 0 {
                            handle_data(&buf[..n], &mut avg_rtt, &mut rtt_dbg_ms);
                            total += 1;
                        } else {
                            println!("No data received");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving data: {}", e);
                        return Err(Box::new(e));
                    }
                }
            },
            _ = sleep(Duration::from_millis(100)) => {
                // Timeout after 100ms, continue the loop
                if !running_clone.load(Ordering::SeqCst) {
                    return Ok(total);
                }
                continue;
            }
        }
    }
}

async fn send_loop(send_client: Arc<UdpSocket>, remote_addr: SocketAddr, kbps: f32, duration: i64) -> Result<i64, Box<dyn Error>> {
    let start_ms = get_millis_as_i64();
    let mut seq: u32 = 0;
    
    let mut last_dbg_ms = start_ms;
    
    const SEND_INTERVAL : i64 = 30;
    let mut total_bytes: f32 = 0.0;
    let mut last_total_bytes: f32 = 0.0;
    let mut bytes_buffer_len: f32 = 10.0*1024.0;
    let mut total : i64 = 0;
    println!("send_loop start, start_ms:{}", start_ms);
    loop {
        if get_millis_as_i64() - start_ms > duration * 1000 {
            break;
        }
        let ts = get_millis_as_i64() as u32;
        let data = Data {
            seq,
            ts,
            data: vec![0; 1024],
        };
        seq += 1;

        let send_bytes: &[u8] = &bincode::serialize(&data)?;
        let sent_len = send_client.send_to(send_bytes, remote_addr).await?;
        total += 1;

        if bytes_buffer_len >= sent_len as f32 {
                bytes_buffer_len -= sent_len as f32;
        } else {
            let last_ms = get_millis_as_i64();
            loop {
                let now_ms = get_millis_as_i64();
                let diff_ms = now_ms - last_ms;
                if diff_ms > SEND_INTERVAL {
                    bytes_buffer_len += kbps * diff_ms as f32 / 8.0;
                    // println!("the left buffer(+):{}, add:{}, diff ms:{}, kbps:{}",
                    //     bytes_buffer_len, kbps * diff_ms as f32 / 8.0, diff_ms, kbps);
                    if bytes_buffer_len >= sent_len as f32 {
                        bytes_buffer_len -= sent_len as f32;
                        break;
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        }
        total_bytes += sent_len as f32;
        
        let diff_dbg_ms = (get_millis_as_i64() - last_dbg_ms) as f32;
        if diff_dbg_ms > 2.0*1000.0 {
            let diff_bytes = total_bytes - last_total_bytes;
            let kbps = diff_bytes * 8.0 /diff_dbg_ms;

            println!("send bytes:{total_bytes}, count:{total}, kbps:{kbps}");

            last_total_bytes = total_bytes;
            last_dbg_ms = get_millis_as_i64();
        }
    }
    Ok(total)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = ClientConfig::new(&env::args().collect::<Vec<String>>())?;
    println!("UDP Client Benchmark is starting...");
    println!("config: {:?}", config);

    // 绑定本地地址，例如 127.0.0.1:8080
    let local_addr_str = format!("{}:{}", config.client_ip, config.client_port);
    let local_addr: SocketAddr = local_addr_str.parse()?;

    // 创建 UdpClient 实例，并传入回调函数

    let target_addr_str = format!("{}:{}", config.server_ip, config.server_port);
    let target_addr: SocketAddr = target_addr_str.parse()?;

    let tokio_client = Arc::new(UdpSocket::bind(local_addr).await?);
    let recv_client = tokio_client.clone();
    let send_client: Arc<UdpSocket> = tokio_client.clone();
    let sent_total: Arc<AtomicI64> = Arc::new(AtomicI64::new(0));
    let recv_total: Arc<AtomicI64> = Arc::new(AtomicI64::new(0));

    let recv_total_clone = recv_total.clone();
    let running_clone = RUNNING.clone();

    let recv_spawn = tokio::spawn(async move {
        let _result = recv_udp_data_select(recv_client, running_clone).await;
        if let Err(e) = _result {
            eprintln!("Error in recv loop: {}", e);
        } else {
            let ret = _result.unwrap();
            println!("recv loop finished, total:{}", ret);
            recv_total_clone.fetch_add(ret, Ordering::Relaxed);
        }
    });
    
    let kbps = config.kbps;
    let duration = config.duration;
    
    let sent_total_clone = sent_total.clone();
    let send_spawn = tokio::spawn(async move {
        let _result = send_loop(send_client, target_addr, kbps, duration).await;
        if let Err(e) = _result {
            eprintln!("Error in send loop: {}", e);
        } else {
            let ret = _result.unwrap();
            
            println!("send loop finished, total:{}", ret);
            sent_total_clone.fetch_add(ret, Ordering::Relaxed);
        }
    });
    let mut sent_count : i64 = 0;
    let send_result = tokio::join!(send_spawn);
    if let Err(e) = send_result.0 {
        eprintln!("Error in send spawn: {}", e);
    } else {
        sent_count = sent_total.load(Ordering::Relaxed);
        println!("send spawn finished successfully, total sent: {}", sent_count);
    }
    
    //tokio sleep 1 second to ensure the recv loop is ready
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    RUNNING.store(false, Ordering::SeqCst);

    let mut recv_count : i64 = 0;
    let recv_result = tokio::join!(recv_spawn);
    if let Err(e) = recv_result.0 {
        eprintln!("Error in recv spawn: {}", e);
    } else {
        recv_count = recv_total.load(Ordering::Relaxed);
        println!("recv spawn finished successfully, total received: {}", recv_count);
    }
    println!("");
    println!("Total sent: {}, Total received: {}", sent_count, recv_count);
    let lost_count = sent_count - recv_count;
    let lost_percent = if sent_count > 0 {
        lost_count as f32 * 100.0 / sent_count as f32
    } else {
        0.0
    };
    println!("Total lost: {}, Lost percent: {:.2}%", lost_count, lost_percent);

    total_stats_to_json(
        sent_count,
        recv_count,
        lost_count,
        lost_percent,
        "total_stats.json"
    ).expect("Failed to write total stats to JSON");
    Ok(())
}