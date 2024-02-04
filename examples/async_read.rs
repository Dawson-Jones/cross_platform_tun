use std::io::Result;

use futures::StreamExt;
use packet::ip::Packet;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = cross_platform_tun::Configuration::default();
    config
        .address("192.168.108.1")
        .netmask("255.255.255.0")
        .up();

    let mut dev = cross_platform_tun::create_as_async(&config).unwrap();
    let mut buf = [0; 4096];
    loop {
        let n = dev.read(&mut buf[..]).await?;
        println!("packet received {n} size");
        for i in 0..n {
            print!("{:x} ", buf[i]);
        }
        println!();
    }
}