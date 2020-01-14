use crate::time;
use core::fmt;
use core::pin;
use core::task;

pub mod start;
pub mod tick;
pub mod ticks;

pub trait Timer: fmt::Debug {
    type Error;

    fn poll_start(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>>;

    fn poll_tick(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>>;
}

pub trait TimerExt: Timer {
    fn start(&mut self) -> start::Start<Self>
    where
        Self: Unpin,
    {
        start::start(self)
    }

    fn tick(&mut self) -> tick::Tick<Self>
    where
        Self: Unpin,
    {
        tick::tick(self)
    }

    fn ticks(&mut self) -> ticks::Ticks<Self>
    where
        Self: Unpin,
    {
        ticks::ticks(self)
    }
}

impl<T> TimerExt for T where T: Timer {}

pub trait IntoPeriodicTimer: Timer {
    type PeriodicTimer: Timer<Error = Self::Error> + Unpin;

    fn into_periodic_timer(self, period: time::Rate) -> Result<Self::PeriodicTimer, Self::Error>;
}

pub trait IntoOneshotTimer: Timer {
    type OneshotTimer: Timer<Error = Self::Error> + Unpin;

    fn into_oneshot_timer(self, delay: time::Duration) -> Result<Self::OneshotTimer, Self::Error>;
}
