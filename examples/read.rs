use std::io::Read;



fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut dev = cross_platform_tun::Configuration::default()
        .address("10.0.0.9")
        .netmask("255.255.255.0")
        .destination("10.0.0.1")
        .up()
        .build();

    let mut buf = [0u8; 4096];

    loop {
        let n = dev.read(&mut buf)?;
        println!("read {} bytes", n);
        println!("{:?}", &buf[..n]);
    }
}