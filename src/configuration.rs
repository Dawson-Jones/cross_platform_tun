use std::{net::Ipv4Addr};

#[cfg(unix)]
use std::os::unix::io::RawFd;
#[cfg(windows)]
use std::os::windows::raw::HANDLE;

use crate::address::IntoIpv4Addr;
use crate::platform;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Layer {
    L2,
    #[default]
    L3,
}

#[derive(Debug, Default, Clone)]
pub struct Configuration {
    pub(crate) name: Option<String>,
    pub(crate) platform: platform::TunConf,

    pub(crate) address: Option<Ipv4Addr>,
    pub(crate) destnation: Option<Ipv4Addr>,
    pub(crate) broadcast: Option<Ipv4Addr>,
    pub(crate) netmask: Option<Ipv4Addr>,
    pub(crate) mtu: Option<i32>,
    // Set the interface to be enabled once crated
    pub(crate) enabled: bool,
    pub(crate) layer: Layer,
    pub(crate) queues: Option<usize>,
    #[cfg(unix)]
    pub(crate) raw_fd: Option<RawFd>,
    #[cfg(not(unix))]
    pub(crate) raw_fd: Option<i32>,
    #[cfg(windows)]
    pub(crate) raw_handle: Option<HANDLE>
}

impl Configuration {
    pub fn platform<F: FnOnce(&mut platform::TunConf)>(&mut self, f: F) -> &mut Self {
        f(&mut self.platform);
        self
    }

    pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
        self.name = Some(name.as_ref().into());
        self
    }

    pub fn address<A: IntoIpv4Addr>(&mut self, value: A) -> &mut Self {
        self.address = Some(value.into_ipv4().unwrap());
        self
    }

    pub fn destination<A: IntoIpv4Addr>(&mut self, value: A) -> &mut Self {
        self.destnation = Some(value.into_ipv4().unwrap());
        self
    }

    pub fn broadcast<A: IntoIpv4Addr>(&mut self, value: A) -> &mut Self {
        self.broadcast = Some(value.into_ipv4().unwrap());
        self
    }

    pub fn netmask<A: IntoIpv4Addr>(&mut self, value: A) -> &mut Self {
        self.netmask = Some(value.into_ipv4().unwrap());
        self
    }

    pub fn mtu(&mut self, value: i32) -> &mut Self {
        self.mtu = Some(value);
        self
    }

    pub fn up(&mut self) -> &mut Self {
        self.enabled = true;
        self
    }

    pub fn down(&mut self) -> &mut Self {
        self.enabled = false;
        self
    }

    pub fn layer(&mut self, layer: Layer) -> &mut Self {
        self.layer = layer;
        self
    }

    pub fn queues(&mut self, queues: usize) -> &mut Self {
        self.queues = Some(queues);
        self
    }

    #[cfg(unix)]
    pub fn raw_fd(&mut self, fd: RawFd) -> &mut Self {
        self.raw_fd = Some(fd);
        self
    }
    #[cfg(not(unix))]
    pub fn raw_fd(&mut self, fd: i32) -> &mut Self {
        self.raw_fd = Some(fd);
        self
    }
    #[cfg(windows)]
    pub fn raw_handle(&mut self, handle: HANDLE) -> &mut Self {
        self.raw_handle = Some(handle);
        self
    }
}