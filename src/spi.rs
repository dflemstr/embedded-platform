use core::fmt;
use core::pin;
use core::task;

pub mod complete;

pub trait Spi<'t, 'a>: fmt::Debug {
    type Error;
    type Transaction: SpiTransaction<'t, 'a, Error = Self::Error>;

    fn transaction(&'t mut self) -> Self::Transaction;
}

pub trait SpiTransaction<'t, 'a>: fmt::Debug {
    type Error;
    type Transfer: SpiTransfer<'a, Error = Self::Error>;
    type TransferSplit: SpiTransfer<'a, Error = Self::Error>;

    fn transfer(&'a mut self, buffer: &'a mut [u8]) -> Result<Self::Transfer, Self::Error>;

    fn transfer_split(
        &'a mut self,
        tx_buffer: &'a [u8],
        rx_buffer: &'a mut [u8],
    ) -> Result<Self::TransferSplit, Self::Error>;
}

pub trait SpiTransfer<'a> {
    type Error;
    fn poll_complete(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>>;
}

pub trait SpiTransferExt<'a>: SpiTransfer<'a> {
    fn complete(&'a mut self) -> complete::Complete<'a, Self>
    where
        Self: Unpin,
    {
        complete::complete(self)
    }
}

impl<'a, T> SpiTransferExt<'a> for T where T: SpiTransfer<'a> {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Polarity {
    IdleLow,
    IdleHigh,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Phase {
    CaptureOnFirstTransition,
    CaptureOnSecondTransition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Mode {
    pub polarity: Polarity,
    pub phase: Phase,
}

pub const MODE_0: Mode = Mode {
    polarity: Polarity::IdleLow,
    phase: Phase::CaptureOnFirstTransition,
};

pub const MODE_1: Mode = Mode {
    polarity: Polarity::IdleLow,
    phase: Phase::CaptureOnSecondTransition,
};

pub const MODE_2: Mode = Mode {
    polarity: Polarity::IdleHigh,
    phase: Phase::CaptureOnFirstTransition,
};

pub const MODE_3: Mode = Mode {
    polarity: Polarity::IdleHigh,
    phase: Phase::CaptureOnSecondTransition,
};
