//! Common IO primitives.
//!
//! These primitives are closely mirroring the definitions in
//! [`tokio-io`](https://docs.rs/tokio-io).  A big difference is that these definitions are not tied
//! to `std::io::Error`, but instead allow for custom error types, and also don't require
//! allocation.

use core::fmt;
use core::pin;
use core::task;

pub mod read;
pub mod read_exact;
pub mod shutdown;
pub mod write;
pub mod write_all;

pub trait Read: fmt::Debug {
    type Error: ReadError;

    fn poll_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buffer: &mut [u8],
    ) -> task::Poll<Result<usize, Self::Error>>;
}

pub trait ReadError: fmt::Debug {
    fn eof() -> Self;
}

pub trait Write: fmt::Debug {
    type Error: WriteError;

    fn poll_write(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        bytes: &[u8],
    ) -> task::Poll<Result<usize, Self::Error>>;

    fn poll_shutdown(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>>;
}

impl<A: ?Sized + Write + Unpin> Write for &mut A {
    type Error = A::Error;

    fn poll_write(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        bytes: &[u8],
    ) -> task::Poll<Result<usize, Self::Error>> {
        pin::Pin::new(&mut **self).poll_write(cx, bytes)
    }

    fn poll_shutdown(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>> {
        pin::Pin::new(&mut **self).poll_shutdown(cx)
    }
}

pub trait WriteError: fmt::Debug {
    fn write_zero() -> Self;
}

pub trait ReadExt: Read {
    fn read<'a>(&'a mut self, buffer: &'a mut [u8]) -> read::Read<'a, Self>
    where
        Self: Unpin,
    {
        read::read(self, buffer)
    }

    fn read_exact<'a>(&'a mut self, buffer: &'a mut [u8]) -> read_exact::ReadExact<'a, Self>
    where
        Self: Unpin,
    {
        read_exact::read_exact(self, buffer)
    }
}

impl<A> ReadExt for A where A: Read {}

pub trait WriteExt: Write {
    fn write<'a>(&'a mut self, bytes: &'a [u8]) -> write::Write<'a, Self>
    where
        Self: Unpin,
    {
        write::write(self, bytes)
    }

    fn write_all<'a>(&'a mut self, bytes: &'a [u8]) -> write_all::WriteAll<'a, Self>
    where
        Self: Unpin,
    {
        write_all::write_all(self, bytes)
    }

    fn shutdown(&mut self) -> shutdown::Shutdown<Self>
    where
        Self: Unpin,
    {
        shutdown::shutdown(self)
    }
}

impl<A> WriteExt for A where A: WriteExt {}
