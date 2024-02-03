use std::io;

use tokio::io::unix::AsyncFd;
use crate::platform::Tun;

pub struct AsyncTun {
    inner: AsyncFd<Tun>,
}

impl AsyncTun {
    pub fn new(tun: Tun) -> io::Result<AsyncTun> {
        tun.set_nonblocking()?;

        Ok(AsyncTun { 
            inner:  AsyncFd::new(tun)?,
        })
    }
}