use std::io::Read;



fn main() -> Result<(), Box<dyn std::error::Error>>{
    // let mut conf = cross_platform_tun::Configuration::default();
    // conf
    //     .address("192.168.50.1")
    //     .netmask("255.255.255.0")
    //     .mtu(1400)
    //     .up();

    let mut conf= cross_platform_tun::Configuration::default();
    conf
        .address("10.0.0.9")
        .netmask("255.255.255.0")
        .destination("10.0.0.1")
        .up();

    let mut dev = cross_platform_tun::create(&conf).unwrap();
    let mut buf = [0u8; 4096];

    loop {
        let n = dev.read(&mut buf)?;
        println!("read {} bytes", n);
        println!("{:?}", &buf[..n]);
    }
}