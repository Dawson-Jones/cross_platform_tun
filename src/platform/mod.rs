#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
// pub use self::linux::Configuration;
pub use linux::*;
// pub use self::linux::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{create, Tun, TunConf};

#[cfg(unix)]
pub mod posix;


#[cfg(test)]
mod test {
    use std::{net::Ipv4Addr, os::unix::thread, thread::sleep, time::Duration};

    use crate::{configuration::Configuration, interface::Interface};

    #[test]
    fn create() {
        let mut conf = Configuration::default();
        conf
            .address("192.168.50.1")
            .netmask("255.255.255.0")
            .mtu(1400)
            .up();

        let dev = super::create(&conf).unwrap();

        assert_eq!(
            "192.168.50.1".parse::<Ipv4Addr>().unwrap(),
            dev.address().unwrap()
        );

        assert_eq!(
            "255.255.255.0".parse::<Ipv4Addr>().unwrap(),
            dev.netmask().unwrap()
        );

        assert_eq!(1400, dev.mtu().unwrap());

        sleep(Duration::from_secs(10));
    }
}