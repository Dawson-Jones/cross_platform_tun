use crate::{error::*, syscall};
use std::{
    io::{self, Read, Write},
    os::fd::{AsRawFd, RawFd},
};

pub(crate) struct Fd(pub RawFd);

impl Fd {
    pub fn new(fd: RawFd) -> Result<Self> {
        if fd < 0 {
            return Err(Error::InvalidDescriptor);
        }

        Ok(Fd(fd))
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        let mut now = syscall!(fcntl(self.0, libc::F_GETFL))?;

        if nonblocking {
            now |= libc::O_NONBLOCK;
        } else {
            now &= !libc::O_NONBLOCK;
        }

        syscall!(fcntl(self.0, libc::F_SETFL, now)).and(Ok(()))
    }
}

impl AsRawFd for Fd {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

impl Read for Fd {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = syscall!(read(self.0, buf.as_mut_ptr() as *mut _, buf.len()))?;

        Ok(n as _)
    }

    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        let iov = bufs.as_ptr().cast();
        let iovcnt = bufs.len().min(libc::c_int::MAX as usize) as _;

        let n = syscall!(readv(self.0, iov, iovcnt))?;

        Ok(n as _)
    }
}

impl Write for Fd {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = syscall!(write(self.0, buf.as_ptr() as *const _, buf.len()))?;

        Ok(n as _)
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        let iov = bufs.as_ptr().cast();
        let iovcnt = bufs.len().min(libc::c_int::MAX as usize) as _;

        let n = syscall!(writev(self.0, iov, iovcnt))?;

        Ok(n as _)
    }

    fn flush(&mut self) -> io::Result<()> {
        // syscall!(fsync(self.0))?;

        Ok(())
    }
}

impl Drop for Fd {
    fn drop(&mut self) {
        if self.0 >= 0 {
            unsafe { libc::close(self.0) };
        }
    }
}
