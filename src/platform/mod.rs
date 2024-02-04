#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{Tun, TunConf};

#[cfg(target_os = "macos")]
mod macos {
    mod sys;
    pub mod tun;
}
#[cfg(target_os = "macos")]
pub use macos::tun::{Tun, TunConf};

#[cfg(unix)]
pub mod posix {
    pub mod fd;
    pub mod sys;
}

#[cfg(test)]
mod test {
    use std::net::Ipv4Addr;

    use crate::{configuration::Configuration, interface::Interface};

    #[test]
    fn create() {
        let dev = Configuration::default()
            .address("192.168.50.1")
            .netmask("255.255.255.0")
            .mtu(1400)
            .up()
            .build()
            .unwrap();

        assert_eq!(
            "192.168.50.1".parse::<Ipv4Addr>().unwrap(),
            dev.address().unwrap()
        );

        assert_eq!(
            "255.255.255.0".parse::<Ipv4Addr>().unwrap(),
            dev.netmask().unwrap()
        );

        assert_eq!(1400, dev.mtu().unwrap());
    }
}
