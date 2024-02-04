use futures::StreamExt;
use packet::ip::Packet;

#[tokio::main]
async fn main() {
    let mut config = cross_platform_tun::Configuration::default();
    config
        .address("192.168.108.1")
        .netmask("255.255.255.0")
        .up();


    let dev = cross_platform_tun::create_as_async(&config).unwrap();
    let mut stream = dev.into_framed();
    while let Some(packet) = stream.next().await {
        match packet {
            Ok(pkt) => println!("pkt: {:#?}", Packet::unchecked(pkt.get_bytes())),
            Err(err) => panic!("Error: {:?}", err),
        }
    }
}