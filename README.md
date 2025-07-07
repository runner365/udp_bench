# UDP Bench - 基于Tokio的UDP服务器

这是一个基于Tokio异步运行时的高性能UDP服务器实现，提供了完整的UDP通信功能。

## 功能特性

- ✅ 异步UDP socket监听
- ✅ 可配置的数据接收回调函数
- ✅ 异步数据发送功能
- ✅ 优雅的服务器关闭机制
- ✅ 资源自动清理
- ✅ 完整的错误处理

## 快速开始

### 1. 运行UDP服务器示例

```bash
cargo run --bin udp_server_example
```

### 2. 运行UDP客户端示例

在另一个终端中运行：

```bash
cargo run --bin udp_client_example
```

## API 使用说明

### 创建UDP服务器

```rust
use udp_bench::udp_server::UdpServer;

let mut server = UdpServer::new();
```

### 设置数据接收回调

```rust
server.set_callback(|data, addr| {
    println!("收到来自 {} 的数据: {:?}", addr, data);
    // 在这里处理接收到的数据
});
```

### 绑定端口并启动

```rust
use std::net::SocketAddr;

// 绑定到指定地址
let addr: SocketAddr = "127.0.0.1:8080".parse()?;
server.bind(addr).await?;

// 启动接收循环
server.start_receiving().await?;
```

### 发送数据

```rust
let target_addr: SocketAddr = "127.0.0.1:8081".parse()?;
let data = b"Hello, UDP!";
server.send_to(data, target_addr).await?;
```

### 优雅关闭

```rust
// 完全关闭服务器（包括接收循环和socket）
server.shutdown().await;

// 或者分步关闭
server.stop_receiving().await;  // 停止接收循环
server.close_socket();          // 关闭socket
```

## 完整示例

```rust
use std::net::SocketAddr;
use tokio;
use udp_bench::udp_server::UdpServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建服务器
    let mut server = UdpServer::new();
    
    // 设置回调函数
    server.set_callback(|data, addr| {
        println!("收到来自 {} 的数据: {:?}", addr, data);
    });
    
    // 绑定并启动
    let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    server.bind(addr).await?;
    server.start_receiving().await?;
    
    // 运行一段时间
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    
    // 优雅关闭
    server.shutdown().await;
    
    Ok(())
}
```

## 架构设计

### 核心组件

1. **UdpServer**: 主要的服务器结构体
2. **UdpDataCallback**: 数据接收回调函数类型
3. **接收循环**: 异步处理UDP数据接收
4. **关闭机制**: 使用oneshot通道实现优雅关闭

### 线程安全

- 使用`Arc<UdpSocket>`实现socket的线程安全共享
- 回调函数使用`Send + Sync` trait确保线程安全
- 所有异步操作都在Tokio运行时中执行

### 错误处理

- 完整的错误类型定义
- 优雅的错误传播
- 资源清理保证

## 性能特性

- 64KB接收缓冲区
- 异步I/O操作
- 零拷贝数据传输
- 高效的内存管理

## 依赖

- `tokio = { version = "1", features = ["full"] }` - 异步运行时
- `serde = { version = "1.0", features = ["derive"] }` - 序列化支持
- `bincode = "1.3"` - 二进制序列化
- `serde_json = "1.0"` - JSON序列化

## 许可证

MIT License 