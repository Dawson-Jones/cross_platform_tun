use super::sys::*;
use crate::{
    address::{Ipv4AddrExt, SockAddrExt},
    configuration::{Configuration, Layer},
    error::{Error, Result},
    interface::Interface,
    platform::posix::fd::Fd,
    syscall,
};
use libc::{c_int, c_short, IFNAMSIZ};
use std::{
    ffi::{CStr, CString}, io::{self, Read}, net::Ipv4Addr, os::fd::{AsRawFd, RawFd}
};

#[derive(Debug, Clone, Copy, Default)]
pub struct TunConf {
    pub(crate) packet_information: bool,
}

impl TunConf {
    pub fn packet_information(&mut self, value: bool) -> &mut Self {
        self.packet_information = value;
        self
    }
}

pub struct Queue {
    tun: Fd,
    pi_enabled: bool,
}

impl Queue {
    fn has_packet_information(&self) -> bool {
        self.pi_enabled
    }

    fn set_nonblocking(&self) -> io::Result<()> {
        self.tun.set_nonblocking(true)
    }

    fn cancel_nonblocking(&self) -> io::Result<()> {
        self.tun.set_nonblocking(false)
    }
}

impl AsRawFd for Queue {
    fn as_raw_fd(&self) -> RawFd {
        self.tun.as_raw_fd()
    }
}

pub struct Tun {
    name: String,
    queues: Vec<Queue>,
    ctl: Fd,
}

impl Tun {
    pub fn new(config: &Configuration) -> Result<Self> {
        let mut queues = Vec::new();
        let mut ifr: libc::ifreq = unsafe { std::mem::zeroed() };

        if let Some(name) = config.name.as_ref() {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    name.as_ptr() as *const _,
                    ifr.ifr_name.as_mut_ptr(),
                    name.len(),
                )
            }
        };


        let tun_type: c_short = config.layer.into();
        let queue_nums = config.queues.unwrap_or(1);
        if queue_nums < 1 {
            return Err(Error::InvalidQueuesNumber);
        }

        let pi = config.platform.packet_information;
        ifr.ifr_ifru.ifru_flags = tun_type
            | if pi { 0 } else { libc::IFF_NO_PI as c_short }
            | if queue_nums > 1 {
                libc::IFF_MULTI_QUEUE as c_short
            } else {
                0
            };

        for _ in 0..queue_nums {
            let tun_fd = syscall!(open(b"/dev/net/tun\0".as_ptr() as *const _, libc::O_RDWR))?;

            unsafe { tunsetiff(tun_fd, &mut ifr as *mut libc::ifreq as *mut c_int) }?;

            queues.push(Queue {
                tun: Fd::new(tun_fd)?,
                pi_enabled: pi,
            });
        }

        let ctl_fd = syscall!(socket(libc::AF_INET, libc::SOCK_DGRAM, 0))?;
        let ctl = Fd::new(ctl_fd)?;

        let name = unsafe {
            CStr::from_ptr(ifr.ifr_name.as_ptr())
                .to_string_lossy()
                .to_string()
        };
        let mut tun = Self { name, queues, ctl };
        tun.configure(config)?;

        Ok(tun)
    }

    fn ifreq(&self) -> libc::ifreq {
        let mut ifr: libc::ifreq = unsafe { std::mem::zeroed() };

        unsafe {
            std::ptr::copy_nonoverlapping(
                self.name.as_ptr() as *const _,
                ifr.ifr_name.as_mut_ptr(),
                self.name.len(),
            )
        };

        ifr
    }

    pub fn set_nonblocking(&self) -> io::Result<()> {
        // self.queues[0].set_nonblocking()
        for queue in &self.queues {
            queue.set_nonblocking()?;
        }
        Ok(())
    }

    pub fn has_packet_information(&self) -> bool {
        self.queues[0].has_packet_information()
    }
}

impl Interface for Tun {
    type Queue = Queue;

    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }

    fn set_name(&mut self, new_name: &str) -> Result<()> {
        // let new_name = CString::new(new_name)?;

        // if new_name.as_bytes_with_nul().len() > IFNAMSIZ {
        //     return Err(Error::NameTooLong);
        // }
        if new_name.len() > IFNAMSIZ {
            return Err(Error::NameTooLong);
        }

        let mut ifr = self.ifreq();
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_name.as_ptr() as *const _,
                ifr.ifr_ifru.ifru_newname.as_mut_ptr(),
                new_name.len(),
            )
        }

        unsafe { siocsifname(self.ctl.as_raw_fd(), &ifr) }?;

        self.name = new_name.into();

        Ok(())
    }

    fn enable(&mut self, value: bool) -> Result<()> {
        let mut flags = self.flags(None)?;

        if value {
            flags |= libc::IFF_UP as i16 | libc::IFF_RUNNING as i16;
        } else {
            flags &= !libc::IFF_UP as i16;
        }

        self.flags(Some(flags))?;

        Ok(())
    }

    fn flags(&self, flags: Option<i16>) -> Result<i16> {
        let mut ifr = self.ifreq();

        if let Some(flags) = flags {
            ifr.ifr_ifru.ifru_flags = flags;
            unsafe { siocsifflags(self.ctl.as_raw_fd(), &ifr) }?;
        } else {
            unsafe { siocgifflags(self.ctl.as_raw_fd(), &mut ifr) }?;
        }

        Ok(unsafe { ifr.ifr_ifru.ifru_flags })
    }

    fn address(&self) -> Result<Ipv4Addr> {
        let mut ifr = self.ifreq();

        unsafe { siocgifaddr(self.ctl.as_raw_fd(), &mut ifr) }?;

        // Ok(Ipv4Addr::from_sockaddr(unsafe { ifr.ifr_ifru.ifru_addr }))
        Ok(unsafe { ifr.ifr_ifru.ifru_addr }.into_ipv4addr())
    }

    fn set_address(&mut self, addr: Ipv4Addr) -> Result<()> {
        let mut ifr = self.ifreq();
        ifr.ifr_ifru.ifru_addr = addr.to_sockaddr();

        unsafe { siocsifaddr(self.ctl.as_raw_fd(), &ifr) }?;

        Ok(())
    }

    fn destination(&self) -> Result<std::net::Ipv4Addr> {
        let mut ifr = self.ifreq();

        unsafe { siocgifdstaddr(self.ctl.as_raw_fd(), &mut ifr) }?;

        Ok(unsafe { ifr.ifr_ifru.ifru_addr }.into_ipv4addr())
    }

    fn set_destination(&mut self, addr: std::net::Ipv4Addr) -> Result<()> {
        let mut ifr = self.ifreq();
        ifr.ifr_ifru.ifru_addr = addr.to_sockaddr();

        unsafe { siocsifdstaddr(self.ctl.as_raw_fd(), &ifr) }?;

        Ok(())
    }

    fn broadcast(&self) -> Result<std::net::Ipv4Addr> {
        let mut ifr = self.ifreq();

        unsafe { siocgifbrdaddr(self.ctl.as_raw_fd(), &mut ifr) }?;

        Ok(unsafe { ifr.ifr_ifru.ifru_addr }.into_ipv4addr())
    }

    fn set_broadcast(&mut self, addr: std::net::Ipv4Addr) -> Result<()> {
        let mut ifr = self.ifreq();
        ifr.ifr_ifru.ifru_addr = addr.to_sockaddr();

        unsafe { siocsifbrdaddr(self.ctl.as_raw_fd(), &ifr) }?;

        Ok(())
    }

    fn netmask(&self) -> Result<std::net::Ipv4Addr> {
        let mut ifr = self.ifreq();

        unsafe { siocgifnetmask(self.ctl.as_raw_fd(), &mut ifr) }?;

        Ok(unsafe { ifr.ifr_ifru.ifru_addr }.into_ipv4addr())
    }

    fn set_netmask(&mut self, addr: std::net::Ipv4Addr) -> Result<()> {
        let mut ifr = self.ifreq();
        ifr.ifr_ifru.ifru_addr = addr.to_sockaddr();

        unsafe { siocsifnetmask(self.ctl.as_raw_fd(), &ifr) }?;

        Ok(())
    }

    fn mtu(&self) -> Result<i32> {
        let mut ifr = self.ifreq();

        unsafe { siocgifmtu(self.ctl.as_raw_fd(), &mut ifr) }?;

        Ok(unsafe { ifr.ifr_ifru.ifru_mtu })
    }

    fn set_mtu(&mut self, mtu: i32) -> Result<()> {
        let mut ifr = self.ifreq();
        ifr.ifr_ifru.ifru_mtu = mtu;

        unsafe { siocsifmtu(self.ctl.as_raw_fd(), &ifr) }?;

        Ok(())
    }

    fn queue(&mut self, index: usize) -> Option<&mut Self::Queue> {
        self.queues.get_mut(index)
    }
}


impl AsRawFd for Tun {
    fn as_raw_fd(&self) -> RawFd {
        self.queues[0].as_raw_fd()
    }
}

impl Read for Tun {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.queues[0].tun.read(buf)
    }
}


impl From<Layer> for c_short {
    fn from(value: Layer) -> Self {
        match value {
            Layer::L2 => libc::IFF_TAP as _,
            Layer::L3 => libc::IFF_TUN as _,
        }
    }
}