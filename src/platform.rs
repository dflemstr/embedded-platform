use core::fmt;
use core::task;

pub mod initialize;

pub trait Platform: fmt::Debug + Sized {
    type Error;
    fn poll_initialize(cx: &mut task::Context<'_>) -> task::Poll<Result<Self, Self::Error>>;
}

pub trait PlatformExt: Platform {
    fn initialize() -> initialize::Initialize<Self> {
        initialize::initialize()
    }
}

impl<A> PlatformExt for A where A: Platform {}
