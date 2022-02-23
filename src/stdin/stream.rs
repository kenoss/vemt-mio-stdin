use crate::io_source::IoSource;
use crate::{event, sys, Interest, Registry, Token};

use std::fmt;
use std::io::{self, IoSlice, IoSliceMut, Read, Write};
use std::net::Shutdown;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::os::unix::net;
use std::path::Path;

/// A non-blocking Unix stream socket.
pub struct StdinStream {
    inner: IoSource<net::StdinStream>,
}

impl StdinStream {
    /// Connects to the socket named by `path`.
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<StdinStream> {
        sys::uds::stream::connect(path.as_ref()).map(StdinStream::from_std)
    }

    /// Creates a new `StdinStream` from a standard `net::StdinStream`.
    ///
    /// This function is intended to be used to wrap a Unix stream from the
    /// standard library in the Mio equivalent. The conversion assumes nothing
    /// about the underlying stream; it is left up to the user to set it in
    /// non-blocking mode.
    ///
    /// # Note
    ///
    /// The Unix stream here will not have `connect` called on it, so it
    /// should already be connected via some other means (be it manually, or
    /// the standard library).
    pub fn from_std(stream: net::StdinStream) -> StdinStream {
        StdinStream {
            inner: IoSource::new(stream),
        }
    }

    /// Creates an unnamed pair of connected sockets.
    ///
    /// Returns two `StdinStream`s which are connected to each other.
    pub fn pair() -> io::Result<(StdinStream, StdinStream)> {
        sys::uds::stream::pair().map(|(stream1, stream2)| {
            (StdinStream::from_std(stream1), StdinStream::from_std(stream2))
        })
    }

    /// Returns the socket address of the local half of this connection.
    pub fn local_addr(&self) -> io::Result<sys::SocketAddr> {
        sys::uds::stream::local_addr(&self.inner)
    }

    /// Returns the socket address of the remote half of this connection.
    pub fn peer_addr(&self) -> io::Result<sys::SocketAddr> {
        sys::uds::stream::peer_addr(&self.inner)
    }

    /// Returns the value of the `SO_ERROR` option.
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.inner.take_error()
    }

    /// Shuts down the read, write, or both halves of this connection.
    ///
    /// This function will cause all pending and future I/O calls on the
    /// specified portions to immediately return with an appropriate value
    /// (see the documentation of `Shutdown`).
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.inner.shutdown(how)
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
