use std::{
    io::{self, Read, Write},
    task::{ready, Poll},
};

use crate::{error::Result, interface::Interface, platform::Tun};
use tokio::io::{unix::AsyncFd, AsyncRead, AsyncWrite};
use tokio_util::codec::Framed;

use super::codec::TunPacketCodec;

pub struct AsyncTun {
    inner: AsyncFd<Tun>,
}

impl AsyncTun {
    pub fn new(tun: Tun) -> Result<AsyncTun> {
        tun.set_nonblocking()?;

        Ok(AsyncTun {
            inner: AsyncFd::new(tun)?,
        })
    }

    pub fn new_multi_queue(tuns: Vec<Tun>) -> Result<Vec<AsyncTun>> {
        tuns.into_iter().map(AsyncTun::new).collect()
    }

    fn get_mut(&mut self) -> &mut Tun {
        self.inner.get_mut()
    }

    pub fn into_framed(mut self) -> Framed<Self, TunPacketCodec> {
        let pi = self.get_mut().has_packet_information();
        let codec = TunPacketCodec::new(pi, self.inner.get_ref().mtu().unwrap_or(1500 + 4));

        Framed::new(self, codec)
    }
}

impl AsyncRead for AsyncTun {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        loop {
            let mut guard = ready!(self
                // .get_mut()
                .inner
                .poll_read_ready_mut(cx))?;
            let rbuf = buf.initialize_unfilled();
            match guard.try_io(|inner| inner.get_mut().read(rbuf)) {
                Ok(res) => return Poll::Ready(res.map(|n| buf.advance(n))),
                Err(_wb) => continue,
            }
        }
    }
}

impl AsyncWrite for AsyncTun {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        loop {
            let mut guard = ready!(
                self
                // .get_mut()
                .inner
                .poll_write_ready_mut(cx)
            )?;

            match guard.try_io(|inner| inner.get_mut().write(buf)) {
                Ok(res) => return Poll::Ready(res),
                Err(_wb) => continue,
            }
        }
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        loop {
            let mut guard = ready!(
                self
                // .get_mut()
                .inner
                .poll_write_ready_mut(cx)
            )?;

            match guard.try_io(|inner| inner.get_mut().flush()) {
                Ok(res) => return Poll::Ready(res),
                Err(_wb) => continue,
            }
        }
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }
}
