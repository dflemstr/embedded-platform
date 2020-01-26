#![no_std]
#![deny(
    // missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    clippy::all
)]
#![allow(clippy::missing_safety_doc)]
// #![forbid(unsafe_code)]

use core::future;
use core::task;
use embedded_platform::platform;
use embedded_platform::specs;

pub mod error;
pub mod gpio;
pub mod i2c;
pub mod serial;
pub mod spi;
pub mod timer;

use nrf52840_hal::gpio as hal_gpio;
use nrf52840_hal::gpio::p0;
use nrf52840_hal::gpio::p1;

#[derive(Debug)]
pub struct ParticleArgon {
    p0: gpio::P0,
    p1: gpio::P1,
    timers: timer::Timers,
}

impl platform::Platform for ParticleArgon {
    type Error = error::Error;

    fn main<I, F>(run: I) -> !
    where
        I: FnOnce(Self) -> F,
        F: future::Future<Output = Result<(), Self::Error>>,
    {
        let future = async {
            use embedded_platform::platform::PlatformExt;
            let platform = ParticleArgon::initialize().await.unwrap();
            run(platform).await.unwrap();

            loop {
                cortex_m::asm::wfi();
            }
        };
        let wait = cortex_m::asm::wfi;
        direct_executor::run(future, wait)
    }

    fn poll_initialize(_cx: &mut task::Context<'_>) -> task::Poll<Result<Self, Self::Error>> {
        let mut core =
            cortex_m::Peripherals::take().expect("A cortex_m platform was already initialized");

        let peripherals = nrf52840_hal::nrf52840_pac::Peripherals::take()
            .expect("A nrf52840 platform was already initialized");

        let p0 = gpio::P0::new(peripherals.P0);
        let p1 = gpio::P1::new(peripherals.P1);

        let timers = timer::Timers::new(
            peripherals.TIMER0,
            peripherals.TIMER1,
            peripherals.TIMER2,
            peripherals.TIMER3,
            peripherals.TIMER4,
            &mut core.NVIC,
        );

        task::Poll::Ready(Ok(Self { p0, p1, timers }))
    }
}

impl ParticleArgon {
    pub fn take_timer0(
        &mut self,
    ) -> timer::Timer<nrf52840_hal::target::TIMER0, nrf52840_hal::timer::OneShot> {
        self.timers.timer0.take().expect("timer 0 is already taken")
    }

    pub fn take_timer1(
        &mut self,
    ) -> timer::Timer<nrf52840_hal::target::TIMER1, nrf52840_hal::timer::OneShot> {
        self.timers.timer1.take().expect("timer 1 is already taken")
    }

    pub fn take_timer2(
        &mut self,
    ) -> timer::Timer<nrf52840_hal::target::TIMER2, nrf52840_hal::timer::OneShot> {
        self.timers.timer2.take().expect("timer 2 is already taken")
    }

    pub fn take_timer3(
        &mut self,
    ) -> timer::Timer<nrf52840_hal::target::TIMER3, nrf52840_hal::timer::OneShot> {
        self.timers.timer3.take().expect("timer 3 is already taken")
    }

    pub fn take_timer4(
        &mut self,
    ) -> timer::Timer<nrf52840_hal::target::TIMER4, nrf52840_hal::timer::OneShot> {
        self.timers.timer4.take().expect("timer 4 is already taken")
    }
}

impl specs::feather::Feather for ParticleArgon {
    type MainLed = Self::D7;
    type MainI2cMapping = i2c::I2cMapping<Self::SDA, Self::SCL>;

    type SDA = gpio::Pin<p0::P0_26<hal_gpio::Input<hal_gpio::Floating>>>;
    type SCL = gpio::Pin<p0::P0_27<hal_gpio::Input<hal_gpio::Floating>>>;
    type D2 = gpio::Pin<p1::P1_01<hal_gpio::Input<hal_gpio::Floating>>>;
    type D3 = gpio::Pin<p1::P1_02<hal_gpio::Input<hal_gpio::Floating>>>;
    type D4 = gpio::Pin<p1::P1_08<hal_gpio::Input<hal_gpio::Floating>>>;
    type D5 = gpio::Pin<p1::P1_10<hal_gpio::Input<hal_gpio::Floating>>>;
    type D6 = gpio::Pin<p1::P1_11<hal_gpio::Input<hal_gpio::Floating>>>;
    type D7 = gpio::Pin<p1::P1_12<hal_gpio::Input<hal_gpio::Floating>>>;
    type D8 = gpio::Pin<p1::P1_03<hal_gpio::Input<hal_gpio::Floating>>>;
    type P0 = gpio::Pin<p0::P0_11<hal_gpio::Input<hal_gpio::Floating>>>;
    type TX = gpio::Pin<p0::P0_06<hal_gpio::Input<hal_gpio::Floating>>>;
    type RX = gpio::Pin<p0::P0_08<hal_gpio::Input<hal_gpio::Floating>>>;
    type MISO = gpio::Pin<p1::P1_14<hal_gpio::Input<hal_gpio::Floating>>>;
    type MOSI = gpio::Pin<p1::P1_13<hal_gpio::Input<hal_gpio::Floating>>>;
    type SCK = gpio::Pin<p1::P1_15<hal_gpio::Input<hal_gpio::Floating>>>;
    type A5 = gpio::Pin<p0::P0_31<hal_gpio::Input<hal_gpio::Floating>>>;
    type A4 = gpio::Pin<p0::P0_30<hal_gpio::Input<hal_gpio::Floating>>>;
    type A3 = gpio::Pin<p0::P0_29<hal_gpio::Input<hal_gpio::Floating>>>;
    type A2 = gpio::Pin<p0::P0_28<hal_gpio::Input<hal_gpio::Floating>>>;
    type A1 = gpio::Pin<p0::P0_04<hal_gpio::Input<hal_gpio::Floating>>>;
    type A0 = gpio::Pin<p0::P0_03<hal_gpio::Input<hal_gpio::Floating>>>;

    fn take_main_led(&mut self) -> Self::MainLed {
        self.p1.p1_12.take().expect("pin 1.12 is already taken")
    }

    fn take_main_i2c(
        &mut self,
    ) -> <Self::MainI2cMapping as embedded_platform::i2c::I2cBusMapping<Self::SDA, Self::SCL>>::Bus
    {
        unimplemented!()
    }

    fn take_sda(&mut self) -> Self::SDA {
        self.p0.p0_26.take().expect("pin 0.26 is already taken")
    }

    fn take_scl(&mut self) -> Self::SCL {
        self.p0.p0_27.take().expect("pin 0.27 is already taken")
    }

    fn take_d2(&mut self) -> Self::D2 {
        self.p1.p1_01.take().expect("pin 1.01 is already taken")
    }

    fn take_d3(&mut self) -> Self::D3 {
        self.p1.p1_02.take().expect("pin 1.02 is already taken")
    }

    fn take_d4(&mut self) -> Self::D4 {
        self.p1.p1_08.take().expect("pin 1.08 is already taken")
    }

    fn take_d5(&mut self) -> Self::D5 {
        self.p1.p1_10.take().expect("pin 1.10 is already taken")
    }

    fn take_d6(&mut self) -> Self::D6 {
        self.p1.p1_11.take().expect("pin 1.11 is already taken")
    }

    fn take_d7(&mut self) -> Self::D7 {
        self.p1.p1_12.take().expect("pin 1.12 is already taken")
    }

    fn take_d8(&mut self) -> Self::D8 {
        self.p1.p1_03.take().expect("pin 1.03 is already taken")
    }

    fn take_p0(&mut self) -> Self::P0 {
        self.p0.p0_11.take().expect("pin 0.11 is already taken")
    }

    fn take_tx(&mut self) -> Self::TX {
        self.p0.p0_06.take().expect("pin 0.06 is already taken")
    }

    fn take_rx(&mut self) -> Self::RX {
        self.p0.p0_08.take().expect("pin 0.08 is already taken")
    }

    fn take_miso(&mut self) -> Self::MISO {
        self.p1.p1_14.take().expect("pin 1.14 is already taken")
    }

    fn take_mosi(&mut self) -> Self::MOSI {
        self.p1.p1_13.take().expect("pin 1.13 is already taken")
    }

    fn take_sck(&mut self) -> Self::SCK {
        self.p1.p1_15.take().expect("pin 1.15 is already taken")
    }

    fn take_a5(&mut self) -> Self::A5 {
        self.p0.p0_31.take().expect("pin 0.31 is already taken")
    }

    fn take_a4(&mut self) -> Self::A4 {
        self.p0.p0_30.take().expect("pin 0.30 is already taken")
    }

    fn take_a3(&mut self) -> Self::A3 {
        self.p0.p0_29.take().expect("pin 0.29 is already taken")
    }

    fn take_a2(&mut self) -> Self::A2 {
        self.p0.p0_28.take().expect("pin 0.28 is already taken")
    }

    fn take_a1(&mut self) -> Self::A1 {
        self.p0.p0_04.take().expect("pin 0.04 is already taken")
    }

    fn take_a0(&mut self) -> Self::A0 {
        self.p0.p0_03.take().expect("pin 0.03 is already taken")
    }
}
