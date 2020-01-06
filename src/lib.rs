#![no_std]
#![deny(
    //missing_docs,
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

pub mod gpio;
pub mod i2c;
pub mod io;
pub mod platform;
pub mod specs;
pub mod spi;
pub mod time;

pub use platform::Platform;
