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

use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    // TODO: actually start an executor and call run()
    loop {
        core::sync::atomic::spin_loop_hint();
    }
}

async fn run() -> Result<(), nrf52840_platform::error::Error> {
    use embedded_platform::platform::PlatformExt;

    let platform = nrf52840_platform::ParticleArgon::initialize().await?;

    feather_blink(platform).await?;

    Ok(())
}

async fn feather_blink<P>(mut feather: P) -> Result<(), P::Error>
where
    P: embedded_platform::specs::feather::Feather,
{
    use embedded_platform::gpio::IntoPushPullOutputPin;
    use embedded_platform::gpio::OutputPinExt;

    let mut main_led = feather.take_main_led().into_push_pull_output_pin(false)?;
    let mut on = false;

    loop {
        on = !on;
        main_led.set(on).await?;
    }
}
