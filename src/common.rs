use serde::{Serialize, Deserialize};
use std::convert::TryInto;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub server_ip: String,
    pub server_port: u16,
    pub client_ip: String,
    pub client_port: u16,
    pub kbps: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub seq: u32,
    pub ts: u32,
    pub data: Vec<u8>,
}

pub fn get_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("时间回溯")
        .as_millis()
}

pub fn get_millis_as_i64() -> i64 {
    let ms_u128 = get_millis();
    ms_u128 as i64
}

pub fn bytes_to_u32(bytes: &[u8]) -> Result<u32, &'static str> {
    let data = &bytes[0..4];
    let data_ret = match data.try_into() {
        Ok(u) => u,
        Err(e) => {
            println!("data.try_into error:{e}");
            return Err("bytes is error");
        },
    };

    let num = u32::from_le_bytes(data_ret);

    Ok(num)
}

impl ClientConfig {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        //通过args.netxt()进行遍历
        let mut server_ip = String::new();
        let mut server_port = 0;
        let mut client_ip = String::new();
        let mut client_port = 0;
        let mut kbps : f32 = 64.0;
        let mut arg_iter = args.iter();

        arg_iter.next();
        loop {
            let arg = arg_iter.next();
            if arg.is_none() {
                break;
            }
            
            let arg = arg.ok_or("please input -l -p -c -r")?;

            match arg.as_str() {
                "-h" => {
                    println!("usage: -s server_ip -p server_port -c client_ip -r client_port -k kbps");
                    return Err("please input -s -p -c -r -k");
                },
                "-s" => {
                    server_ip = arg_iter.next().ok_or("please input server ip")?.clone();
                },
                "-p" => {
                    let server_port_str = arg_iter.next().ok_or("please input server port")?.clone();
                    server_port = match server_port_str.parse::<u16>() {
                        Ok(port) => port,
                        Err(e) => {
                            println!("server port is error: {e}");
                            return Err("server port is error");
                        },
                    };
                },
                "-c" => {
                    client_ip = arg_iter.next().ok_or("please input client ip")?.clone();
                },
                "-r" => {
                    let client_port_str = arg_iter.next().ok_or("please input client port")?.clone();
                    client_port = match client_port_str.parse::<u16>() {
                        Ok(port) => port,
                        Err(e) => {
                            println!("client port is error: {e}");
                            return Err("client port is error");
                        },
                    };
                },
                "-k" => {
                    let kbps_str = arg_iter.next().ok_or("please input kbps")?.clone();
                    kbps = match kbps_str.parse::<f32>() {
                        Ok(kbps) => kbps,
                        Err(e) => {
                            println!("kbps is error: {e}");
                            return Err("kbps is error");
                        },
                    };
                },
                _ => {
                    println!("please input -s -p -c -r -k");
                    return Err("please input -s -p -c -r -k");
                },
            }
        }

        Ok(Self {
            server_ip,
            server_port,
            client_ip,
            client_port,
            kbps,
        })
    }
}