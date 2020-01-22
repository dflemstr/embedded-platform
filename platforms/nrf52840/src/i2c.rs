#![allow(unused_variables)]

use crate::error;
use crate::gpio;
use core::pin;
use core::task;
use nrf52840_hal::gpio as hal_gpio;
use nrf52840_hal::gpio::p0;

#[derive(Clone, Copy, Debug)]
pub struct I2c;

#[derive(Clone, Copy, Debug)]
pub struct I2cRead {}

#[derive(Clone, Copy, Debug)]
pub struct I2cWrite {}

#[derive(Clone, Copy, Debug)]
pub struct I2cMapping<SDA, SCL>(SDA, SCL);

impl
    embedded_platform::i2c::I2cBusMapping<
        gpio::Pin<p0::P0_26<hal_gpio::Input<hal_gpio::Floating>>>,
        gpio::Pin<p0::P0_27<hal_gpio::Input<hal_gpio::Floating>>>,
    >
    for I2cMapping<
        gpio::Pin<p0::P0_26<hal_gpio::Input<hal_gpio::Floating>>>,
        gpio::Pin<p0::P0_27<hal_gpio::Input<hal_gpio::Floating>>>,
    >
{
    type Error = error::Error;
    type Bus = I2c;

    fn poll_initialize(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        sda: &mut gpio::Pin<p0::P0_26<hal_gpio::Input<hal_gpio::Floating>>>,
        scl: &mut gpio::Pin<p0::P0_27<hal_gpio::Input<hal_gpio::Floating>>>,
    ) -> task::Poll<Result<Self::Bus, Self::Error>>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl embedded_platform::i2c::I2cRead for I2c {
    type Error = error::Error;
    type Read = I2cRead;

    fn poll_begin_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        addr: u8,
    ) -> task::Poll<Result<Self::Read, Self::Error>> {
        unimplemented!()
    }
}

impl embedded_platform::io::Read for I2cRead {
    type Error = error::Error;

    fn poll_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buffer: &mut [u8],
    ) -> task::Poll<Result<usize, Self::Error>> {
        unimplemented!()
    }
}

impl embedded_platform::i2c::I2cWrite for I2c {
    type Error = error::Error;
    type Write = I2cWrite;

    fn poll_begin_write(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        addr: u8,
    ) -> task::Poll<Result<Self::Write, Self::Error>> {
        unimplemented!()
    }
}

impl embedded_platform::io::Write for I2cWrite {
    type Error = error::Error;

    fn poll_write(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        bytes: &[u8],
    ) -> task::Poll<Result<usize, Self::Error>> {
        unimplemented!()
    }

    fn poll_flush(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>> {
        unimplemented!()
    }

    fn poll_close(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>> {
        unimplemented!()
    }
}
