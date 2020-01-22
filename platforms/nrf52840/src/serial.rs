use crate::error;
use core::fmt;
use core::pin;
use core::task;

#[repr(transparent)]
pub struct Uarte<U>(U);

impl embedded_platform::io::Read
    for Uarte<nrf52840_hal::uarte::Uarte<nrf52840_hal::target::UARTE0>>
{
    type Error = error::Error;

    fn poll_read(
        mut self: pin::Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
        buffer: &mut [u8],
    ) -> task::Poll<Result<usize, Self::Error>> {
        // TODO: use non-blocking call
        let this = &mut *self;
        let len = buffer.len();
        this.0.read(buffer)?;
        task::Poll::Ready(Ok(len))
    }
}

impl embedded_platform::io::Write
    for Uarte<nrf52840_hal::uarte::Uarte<nrf52840_hal::target::UARTE0>>
{
    type Error = error::Error;

    fn poll_write(
        mut self: pin::Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
        bytes: &[u8],
    ) -> task::Poll<Result<usize, Self::Error>> {
        // TODO: use non-blocking call
        let this = &mut *self;
        let len = bytes.len().min(65536);
        this.0.write(&bytes[..len])?;
        task::Poll::Ready(Ok(len))
    }

    fn poll_flush(
        self: pin::Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>> {
        task::Poll::Ready(Ok(()))
    }

    fn poll_close(
        self: pin::Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>> {
        task::Poll::Ready(Ok(()))
    }
}

impl<U> fmt::Debug for Uarte<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Uarte").finish()
    }
}
