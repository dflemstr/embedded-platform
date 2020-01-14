//! A simple example app that blinks the main LED of any feather board.
//!
//! It is wired up to use a ParticleArgon device, but it demonstrates some generic code for the
//! actual blinking.
#![no_std]
#![no_main]
#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    clippy::all
)]
#![forbid(unsafe_code)]
#![allow(clippy::missing_safety_doc)]

use embedded_platform::prelude::*;
use futures::prelude::*;

use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    nrf52840_platform::ParticleArgon::main(|mut platform| async {
        let timer = platform.take_timer0().into_periodic_timer(1.0.hz())?;
        feather_blink(platform, timer).await?;
        Ok(())
    })
}

async fn feather_blink<P>(
    mut feather: P,
    mut timer: impl embedded_platform::timer::Timer<Error = P::Error> + Unpin,
) -> Result<(), P::Error>
where
    P: embedded_platform::specs::feather::Feather,
{
    let mut main_led = feather.take_main_led().into_push_pull_output_pin(false)?;
    let mut on = false;

    timer.start().await?;
    let mut ticks = timer.ticks();

    loop {
        on = !on;
        main_led.set(on).await?;
        ticks.try_next().await?;
    }
}
