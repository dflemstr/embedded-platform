use crate::io;
use core::fmt;
use core::pin;
use core::task;

pub mod begin_read;
pub mod begin_write;
pub mod initialize;

// TODO: this should capture the lifetime of self and let it flow into Self::Read
pub trait I2cRead: fmt::Debug {
    type Error;
    type Read: io::Read;

    fn poll_begin_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        addr: u8,
    ) -> task::Poll<Result<Self::Read, Self::Error>>;
}

// TODO: this should capture the lifetime of self and let it flow into Self::Read
pub trait I2cReadExt: I2cRead {
    fn begin_read(&mut self, address: u8) -> begin_read::BeginRead<Self>
    where
        Self: Unpin,
    {
        begin_read::begin_read(self, address)
    }
}

impl<'r, A> I2cReadExt for A where A: I2cRead {}

pub trait I2cWrite: fmt::Debug {
    type Error;
    type Write: io::Write;

    fn poll_begin_write(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        addr: u8,
    ) -> task::Poll<Result<Self::Write, Self::Error>>;
}

pub trait I2cWriteExt: I2cWrite {
    fn begin_write(&mut self, address: u8) -> begin_write::BeginWrite<Self>
    where
        Self: Unpin,
    {
        begin_write::begin_write(self, address)
    }
}

impl<A> I2cWriteExt for A where A: I2cWrite {}

pub trait I2cBusMapping<SDA, SCL> {
    type Error;
    type Bus: I2cRead<Error = Self::Error> + I2cWrite<Error = Self::Error>;

    fn poll_initialize(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        sda: &mut SDA,
        scl: &mut SCL,
    ) -> task::Poll<Result<Self::Bus, Self::Error>>
    where
        Self: Sized;
}

pub trait I2cBusMappingExt<SDA, SCL>: I2cBusMapping<SDA, SCL>
where
    SDA: Unpin,
    SCL: Unpin,
{
    fn initialize(self, sda: SDA, scl: SCL) -> initialize::Initialize<Self, SDA, SCL>
    where
        Self: Sized + Unpin,
    {
        initialize::initialize(self, sda, scl)
    }
}

impl<A, SDA, SCL> I2cBusMappingExt<SDA, SCL> for A
where
    A: I2cBusMapping<SDA, SCL>,
    SDA: Unpin,
    SCL: Unpin,
{
}
