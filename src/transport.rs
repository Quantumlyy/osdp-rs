//! Transport abstraction. The driver works against any half-duplex byte
//! stream that exposes [`Transport::write_all`] and [`Transport::read`].

use crate::error::Error;

/// Synchronous half-duplex byte transport.
pub trait Transport {
    /// Send `bytes` in their entirety. The implementation must not return
    /// until either every byte is on the wire or an error has occurred.
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), Error>;

    /// Read up to `buf.len()` bytes.
    ///
    /// Implementations should return `Ok(0)` when no bytes are available
    /// before the deadline configured at the transport level. Drivers track
    /// REPLY_DELAY independently via the [`crate::clock::Clock`].
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;

    /// Flush any internally-buffered bytes.
    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl<T: Transport + ?Sized> Transport for &mut T {
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), Error> {
        (**self).write_all(bytes)
    }
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        (**self).read(buf)
    }
    fn flush(&mut self) -> Result<(), Error> {
        (**self).flush()
    }
}

/// In-memory loopback transport used in tests and the chaos bus.
#[cfg(feature = "alloc")]
#[derive(Debug, Default)]
pub struct VecTransport {
    /// Bytes the caller has written and that are waiting to be drained by
    /// the peer (or by [`Self::take_outgoing`]).
    pub outgoing: alloc::collections::VecDeque<u8>,
    /// Bytes the peer has fed in and that the caller is yet to read.
    pub incoming: alloc::collections::VecDeque<u8>,
}

#[cfg(feature = "alloc")]
impl VecTransport {
    /// New empty transport.
    pub fn new() -> Self {
        Self::default()
    }

    /// Push `bytes` onto the read-side queue (simulates the peer).
    pub fn feed(&mut self, bytes: &[u8]) {
        self.incoming.extend(bytes.iter().copied());
    }

    /// Drain the write-side queue.
    pub fn take_outgoing(&mut self) -> alloc::vec::Vec<u8> {
        let v: alloc::vec::Vec<u8> = self.outgoing.drain(..).collect();
        v
    }
}

#[cfg(feature = "alloc")]
impl Transport for VecTransport {
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.outgoing.extend(bytes.iter().copied());
        Ok(())
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let n = buf.len().min(self.incoming.len());
        for slot in buf.iter_mut().take(n) {
            *slot = self.incoming.pop_front().unwrap();
        }
        Ok(n)
    }
}

/// Adapter wrapping any [`embedded_io::Read`] + [`embedded_io::Write`] into a
/// [`Transport`].
///
/// The embedded-io crate is the de-facto traits crate for synchronous byte
/// streams in `no_std` Rust (UARTs, sockets, in-memory pipes, …).
#[cfg(feature = "embedded-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-io")))]
pub struct EmbeddedIoTransport<T> {
    /// The wrapped reader/writer.
    pub inner: T,
}

#[cfg(feature = "embedded-io")]
impl<T> EmbeddedIoTransport<T> {
    /// Wrap `inner`.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Unwrap, returning the underlying reader/writer.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

#[cfg(feature = "embedded-io")]
impl<T> Transport for EmbeddedIoTransport<T>
where
    T: embedded_io::Read + embedded_io::Write,
{
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), Error> {
        embedded_io::Write::write_all(&mut self.inner, bytes)
            .map_err(|_| Error::Io("embedded-io write failed"))
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        embedded_io::Read::read(&mut self.inner, buf)
            .map_err(|_| Error::Io("embedded-io read failed"))
    }

    fn flush(&mut self) -> Result<(), Error> {
        embedded_io::Write::flush(&mut self.inner)
            .map_err(|_| Error::Io("embedded-io flush failed"))
    }
}
