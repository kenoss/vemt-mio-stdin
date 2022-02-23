use crate::io_source::IoSource;
use crate::{event, Interest, Registry, Token};

use std::fmt;
use std::io::{self, IoSlice, IoSliceMut, Read, Write};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

/// A non-blocking Unix stream socket.
pub struct StdinStream {
    inner: IoSource<std::fs::File>,
}

impl StdinStream {
    /// Creates a new `StdinStream` from a file descriptor of stdin.
    pub fn from_std(file: std::fs::File) -> StdinStream {
        StdinStream {
            inner: IoSource::new(file),
        }
    }
}

impl Read for StdinStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.do_io(|inner| (&*inner).read(buf))
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.inner.do_io(|inner| (&*inner).read_vectored(bufs))
    }
}

impl<'a> Read for &'a StdinStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.do_io(|inner| (&*inner).read(buf))
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.inner.do_io(|inner| (&*inner).read_vectored(bufs))
    }
}

impl Write for StdinStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.do_io(|inner| (&*inner).write(buf))
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.inner.do_io(|inner| (&*inner).write_vectored(bufs))
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.do_io(|inner| (&*inner).flush())
    }
}

impl<'a> Write for &'a StdinStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.do_io(|inner| (&*inner).write(buf))
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.inner.do_io(|inner| (&*inner).write_vectored(bufs))
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.do_io(|inner| (&*inner).flush())
    }
}

impl event::Source for StdinStream {
    fn register(
        &mut self,
        registry: &Registry,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        self.inner.register(registry, token, interests)
    }

    fn reregister(
        &mut self,
        registry: &Registry,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        self.inner.reregister(registry, token, interests)
    }

    fn deregister(&mut self, registry: &Registry) -> io::Result<()> {
        self.inner.deregister(registry)
    }
}

impl fmt::Debug for StdinStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl IntoRawFd for StdinStream {
    fn into_raw_fd(self) -> RawFd {
        self.inner.into_inner().into_raw_fd()
    }
}

impl AsRawFd for StdinStream {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}

impl FromRawFd for StdinStream {
    /// Converts a `RawFd` to a `StdinStream`.
    ///
    /// # Notes
    ///
    /// The caller is responsible for ensuring that the socket is in
    /// non-blocking mode.
    unsafe fn from_raw_fd(fd: RawFd) -> StdinStream {
        StdinStream::from_std(FromRawFd::from_raw_fd(fd))
    }
}
