use libc::{ctl_info, ifreq};

pub const UTUN_CONTROL_NAME: &str = "com.apple.net.utun_control";

// ifaliasreq can not be found from libc
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ifaliasreq {
    pub ifra_name: [libc::c_char; libc::IFNAMSIZ],
    pub ifra_addr: libc::sockaddr,
    pub ifra_broadaddr: libc::sockaddr,
    pub ifra_mask: libc::sockaddr,
}

nix::ioctl_readwrite!(ctliocginfo, b'N', 3, ctl_info); // /* get id from name */
nix::ioctl_write_ptr!(siocsifflags, b'i', 16, ifreq);
nix::ioctl_readwrite!(siocgifflags, b'i', 17, ifreq);

nix::ioctl_write_ptr!(siocsifaddr, b'i', 12, ifreq);
nix::ioctl_readwrite!(siocgifaddr, b'i', 33, ifreq);

nix::ioctl_write_ptr!(siocsifdstaddr, b'i', 14, ifreq);
nix::ioctl_readwrite!(siocgifdstaddr, b'i', 34, ifreq);

nix::ioctl_write_ptr!(siocsifbrdaddr, b'i', 19, ifreq);
nix::ioctl_readwrite!(siocgifbrdaddr, b'i', 35, ifreq);

nix::ioctl_write_ptr!(siocsifnetmask, b'i', 22, ifreq);
nix::ioctl_readwrite!(siocgifnetmask, b'i', 37, ifreq);

nix::ioctl_write_ptr!(siocsifmtu, b'i', 52, ifreq);
nix::ioctl_readwrite!(siocgifmtu, b'i', 51, ifreq);

// SIOCAIFADDR
nix::ioctl_write_ptr!(siocaifaddr, b'i', 26, ifaliasreq);
// SIOCDIFADDR
nix::ioctl_write_ptr!(siocdifaddr, b'i', 25, ifreq);
