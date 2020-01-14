#![no_std]
#![deny(
    // missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    clippy::all
)]
#![feature(thread_local, generator_trait, optin_builtin_traits)]

pub mod future;

pub use core::*;
