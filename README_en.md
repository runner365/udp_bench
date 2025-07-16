# udp_bench Introduction
udp_bench is a UDP benchmarking tool developed based on Rust's Tokio asynchronous library:
* High Performance: High-throughput performance based on Tokio's asynchronous capabilities
* Cross-platform: Cross-platform support thanks to Rust and Tokio

It can be used to test UDP connections between servers:
* Connectivity
* Bandwidth measurement
* RTT (Round-Trip Time) latency
* Packet loss rate

# Compilation
Install Rust, then run:
```
cargo build
```


# Usage

## UDP Server
```
/target/debug/udp_server_bench -l 192.168.20.245 -p 9898
```
* -l Set listening address
* -p Set UDP listening port

## UDP Client
```
./target/debug/udp_client_bench -s 192.168.20.245 -p 9898 -c 192.168.10.245 -r 7878 -k 100
```
* -s Set server IP address
* -p Set server UDP port
* -c Set client IP address
* -r Set client UDP port
* -k Set traffic sending rate in kbps

## Test Results:
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
The result logs contain the following information:
* Sent kbps, packets per second (pps), and total count
* Real-time RTT and average RTT
* Packet loss count and packet loss rate

The above markdown has the Chinese parts translated into English.