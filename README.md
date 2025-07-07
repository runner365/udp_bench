# udp_bench简介
udp_bench是基于rust tokio异步库开发的udp压测工具:
* 高性能: 基于tokio的异步高性能吞吐；
* 跨平台: rust tokio的跨平台

可用于测试服务器之间的udp连接情况：
* 连通性
* 探测带宽大小
* rtt延时
* 丢包率

# 编译
安装rust，执行
```
cargo build
```

# 运行

## udp服务端
```
./target/debug/udp_server_bench -l 192.168.20.245 -p 9898
```
* -l 设置监听地址
* -p 设置udp监听端口

## udp客户端
```
./target/debug/udp_client_bench -s 192.168.20.245 -p 9898 -c 192.168.10.245 -r 7878 -k 100
```
* -s 设置服务端的ip
* -p 设置服务端的udp端口
* -c 设置客户端的ip地址
* -r 设置客户端的udp端口
* -k 设置发送流量大小，单位kbps

## 测试结果：
```
send bytes:263120, count:253, kbps:101.811066
response rtt:1 seq:253, ts:3820570846, avg_rtt:0.80603707
send bytes:288080, count:277, kbps:97.882355
response rtt:1 seq:277, ts:3820572886, avg_rtt:0.9496931
send bytes:314080, count:302, kbps:101.216545
response rtt:2 seq:302, ts:3820574941, avg_rtt:0.9697274
send bytes:339040, count:326, kbps:97.64303
response rtt:1 seq:326, ts:3820576986, avg_rtt:0.9545333
send bytes:364000, count:350, kbps:97.54763
response rtt:1 seq:350, ts:3820579033, avg_rtt:0.9339393
send loop finished, total:368
send spawn finished successfully, total sent: 368
recv loop finished, total:368
recv spawn finished successfully, total received: 368
```
结果的日志包含信息:
* 发送kbps，fps和总数
* 实时rtt和平均rtt
* 丢包数，丢包率
