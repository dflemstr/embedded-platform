use crate::error;
use core::fmt;
use core::pin;
use core::task;

#[derive(Debug)]
pub struct Spim<T> {
    raw: nrf52840_hal::spim::Spim<T>,
    cs: nrf52840_hal::gpio::Pin<nrf52840_hal::gpio::Output<nrf52840_hal::gpio::PushPull>>,
}

#[derive(Debug)]
pub struct Transaction<'t, T>
where
    T: nrf52840_hal::spim::Instance,
{
    raw: nrf52840_hal::spim::SpiTransaction<'t, T>,
}

#[derive(Debug)]
pub struct Transfer<'a, T> {
    raw: nrf52840_hal::spim::SpiTransfer<'a, T>,
}

#[derive(Debug)]
pub struct TransferSplit<'a, T> {
    raw: nrf52840_hal::spim::SpiUnevenTransfer<'a, T>,
}

#[derive(Debug)]
pub struct InterruptState {
    waker: Option<task::Waker>,
}

pub trait InterruptStorage {
    fn access_interrupt_storage<F>(critical_section: F)
    where
        F: FnOnce(&mut InterruptState);
}

impl<'t, 'a, T> embedded_platform::spi::Spi<'t, 'a> for Spim<T>
where
    T: nrf52840_hal::spim::Instance + InterruptStorage + fmt::Debug + 't + 'a,
{
    type Error = error::Error;
    type Transaction = Transaction<'t, T>;

    fn transaction(&'t mut self) -> Self::Transaction {
        let raw = self.raw.transaction(&mut self.cs);
        Transaction { raw }
    }
}

impl<'t, 'a, T> embedded_platform::spi::SpiTransaction<'t, 'a> for Transaction<'t, T>
where
    T: nrf52840_hal::spim::Instance + InterruptStorage + fmt::Debug + 't + 'a,
{
    type Error = error::Error;

    type Transfer = Transfer<'a, T>;
    type TransferSplit = TransferSplit<'a, T>;

    fn transfer(&'a mut self, buffer: &'a mut [u8]) -> Result<Self::Transfer, Self::Error> {
        let raw = self.raw.transfer_polling(buffer)?;
        Ok(Transfer { raw })
    }

    fn transfer_split(
        &'a mut self,
        tx_buffer: &'a [u8],
        rx_buffer: &'a mut [u8],
    ) -> Result<Self::TransferSplit, Self::Error> {
        let raw = self
            .raw
            .transfer_split_uneven_polling(tx_buffer, rx_buffer)?;
        Ok(TransferSplit { raw })
    }
}

impl<'a, T> embedded_platform::spi::SpiTransfer<'a> for Transfer<'a, T>
where
    T: nrf52840_hal::spim::Instance + InterruptStorage,
{
    type Error = error::Error;

    fn poll_complete(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>> {
        let this = &mut *self;
        if this.raw.poll_complete()? {
            task::Poll::Ready(Ok(()))
        } else {
            T::access_interrupt_storage(|mut storage| storage.waker = Some(cx.waker().clone()));
            task::Poll::Pending
        }
    }
}

impl<'a, T> embedded_platform::spi::SpiTransfer<'a> for TransferSplit<'a, T>
where
    T: nrf52840_hal::spim::Instance + InterruptStorage,
{
    type Error = error::Error;

    fn poll_complete(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>> {
        let this = &mut *self;
        if this.raw.poll_complete()? {
            task::Poll::Ready(Ok(()))
        } else {
            T::access_interrupt_storage(|mut storage| storage.waker = Some(cx.waker().clone()));
            task::Poll::Pending
        }
    }
}
