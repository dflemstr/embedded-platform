use crate::io;
use core::fmt;
use core::pin;
use core::task;

pub trait SpiRead: fmt::Debug + Clone {
    type Error: io::ReadError;
    type Read: io::Read<Error = Self::Error> + Unpin;

    fn poll_begin_read_txn(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<Self::Read, Self::Error>>;
}

pub trait SpiWrite: fmt::Debug + Clone {
    type Error: io::WriteError;
    type Write: io::Write<Error = Self::Error> + Unpin;

    fn poll_begin_write_txn(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<Self::Write, Self::Error>>;
}

pub type SpiRWEnds<T> = (<T as SpiDuplex>::Read, <T as SpiDuplex>::Write);

pub trait SpiDuplex: fmt::Debug + Clone {
    type Error: io::ReadError + io::WriteError;
    type Read: io::Read<Error = Self::Error> + Unpin;
    type Write: io::Write<Error = Self::Error> + Unpin;

    fn poll_begin_duplex_txn(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<SpiRWEnds<Self>, Self::Error>>;
}

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
