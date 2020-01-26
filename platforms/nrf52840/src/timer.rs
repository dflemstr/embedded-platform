#![allow(clippy::missing_safety_doc)]

use crate::error;
use core::cell;
use core::fmt;
use core::pin;
use core::task;
use nrf52840_hal::target::interrupt;

pub struct Timer<T, M>
where
    T: Unpin,
{
    ticks: u32,
    raw: Option<nrf52840_hal::timer::Timer<T, M>>,
}

#[derive(Debug)]
pub(crate) struct Timers {
    pub timer0: Option<Timer<nrf52840_hal::target::TIMER0, nrf52840_hal::timer::OneShot>>,
    pub timer1: Option<Timer<nrf52840_hal::target::TIMER1, nrf52840_hal::timer::OneShot>>,
    pub timer2: Option<Timer<nrf52840_hal::target::TIMER2, nrf52840_hal::timer::OneShot>>,
    pub timer3: Option<Timer<nrf52840_hal::target::TIMER3, nrf52840_hal::timer::OneShot>>,
    pub timer4: Option<Timer<nrf52840_hal::target::TIMER4, nrf52840_hal::timer::OneShot>>,
}

impl<T, M> Timer<T, M>
where
    T: Unpin,
{
    fn new(raw: nrf52840_hal::timer::Timer<T, M>) -> Self {
        let ticks = 1;
        let raw = Some(raw);
        Self { ticks, raw }
    }
}

impl<T, M> fmt::Debug for Timer<T, M>
where
    T: Unpin,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Timer").finish()
    }
}

impl Timers {
    pub(crate) fn new(
        timer0: nrf52840_hal::target::TIMER0,
        timer1: nrf52840_hal::target::TIMER1,
        timer2: nrf52840_hal::target::TIMER2,
        timer3: nrf52840_hal::target::TIMER3,
        timer4: nrf52840_hal::target::TIMER4,
        nvic: &mut nrf52840_hal::target::NVIC,
    ) -> Self {
        let mut timer0 = nrf52840_hal::timer::Timer::new(timer0);
        let mut timer1 = nrf52840_hal::timer::Timer::new(timer1);
        let mut timer2 = nrf52840_hal::timer::Timer::new(timer2);
        let mut timer3 = nrf52840_hal::timer::Timer::new(timer3);
        let mut timer4 = nrf52840_hal::timer::Timer::new(timer4);

        timer0.enable_interrupt(Some(nvic));
        timer1.enable_interrupt(Some(nvic));
        timer2.enable_interrupt(Some(nvic));
        timer3.enable_interrupt(Some(nvic));
        timer4.enable_interrupt(Some(nvic));

        let timer0 = Some(Timer::new(timer0));
        let timer1 = Some(Timer::new(timer1));
        let timer2 = Some(Timer::new(timer2));
        let timer3 = Some(Timer::new(timer3));
        let timer4 = Some(Timer::new(timer4));

        Timers {
            timer0,
            timer1,
            timer2,
            timer3,
            timer4,
        }
    }
}

struct InterruptState<T> {
    inner: Option<InnerInterruptState<T>>,
}

struct InnerInterruptState<T> {
    waker: task::Waker,
    timer: InterruptTimer<T>,
}

enum InterruptTimer<T> {
    Oneshot(nrf52840_hal::timer::Timer<T, nrf52840_hal::timer::OneShot>),
    Periodic(nrf52840_hal::timer::Timer<T, nrf52840_hal::timer::Periodic>),
}

type InterruptStateCell<T> = bare_metal::Mutex<cell::RefCell<InterruptState<T>>>;

macro_rules! timer {
    ($ty:ty, $interrupt:ident, $state:ident) => {
        static $state: InterruptStateCell<$ty> =
            bare_metal::Mutex::new(cell::RefCell::new(InterruptState { inner: None }));

        impl<M> embedded_platform::timer::Timer for Timer<$ty, M>
        where
            M: Unpin,
        {
            type Error = error::Error;

            fn poll_start(
                mut self: pin::Pin<&mut Self>,
                _cx: &mut task::Context<'_>,
            ) -> task::Poll<Result<(), Self::Error>> {
                use embedded_hal::timer::CountDown;

                let this = &mut *self;
                this.raw.unwrap().start(this.ticks);

                task::Poll::Ready(Ok(()))
            }

            fn poll_tick(
                self: pin::Pin<&mut Self>,
                cx: &mut task::Context<'_>,
            ) -> task::Poll<Result<(), Self::Error>> {
                impl_poll_tick(&mut *self, cx, &$state)
            }
        }

        impl<M> embedded_platform::timer::IntoPeriodicTimer for Timer<$ty, M>
        where
            M: Unpin,
        {
            type PeriodicTimer = Timer<$ty, nrf52840_hal::timer::Periodic>;

            fn into_periodic_timer(
                self,
                rate: embedded_platform::time::Rate,
            ) -> Result<Self::PeriodicTimer, Self::Error> {
                let ticks = (nrf52840_hal::timer::Timer::<$ty, M>::TICKS_PER_SECOND as f32
                    / rate.as_hz()) as u32;
                let raw = Some(self.raw.unwrap().into_periodic());
                Ok(Timer { ticks, raw })
            }
        }

        impl<M> embedded_platform::timer::IntoOneshotTimer for Timer<$ty, M>
        where
            M: Unpin,
        {
            type OneshotTimer = Timer<$ty, nrf52840_hal::timer::OneShot>;

            fn into_oneshot_timer(
                self,
                period: embedded_platform::time::Duration,
            ) -> Result<Self::OneshotTimer, Self::Error> {
                let ticks = period.as_nanos()
                    * nrf52840_hal::timer::Timer::<$ty, M>::TICKS_PER_SECOND
                    / 1_000_000;
                let raw = Some(self.raw.unwrap().into_oneshot());
                Ok(Timer { ticks, raw })
            }
        }

        #[cfg(feature = "rt")]
        #[interrupt]
        fn $interrupt() {
            interrupt_impl(&$state, nrf52840_hal::target::Interrupt::$interrupt);
        }
    };
}

fn impl_poll_tick<T, M, E>(
    this: &mut Timer<T, M>,
    cx: &mut task::Context,
    state: &InterruptStateCell<T>,
) -> task::Poll<Result<(), E>>
where
    T: Unpin,
{
    cortex_m::interrupt::free(|cs| {
        let mut state = state.borrow(cs).borrow_mut();
        if let Some(state) = state.inner.take() {
            this.raw = Some(state.timer);
            task::Poll::Ready(Ok(()))
        } else {
            state.inner = Some((this.raw.take().unwrap(), cx.waker().clone()));
            task::Poll::Pending
        }
    })
}

fn interrupt_impl<T>(state: &InterruptStateCell<T>, interrupt: nrf52840_hal::target::Interrupt) {
    cortex_m::peripheral::NVIC::unpend(interrupt);
    cortex_m::interrupt::free(|cs| {
        if let Some((raw, waker)) = state.borrow(cs).borrow_mut().inner {
            raw.wait().unwrap();
            waker.wake();
        }
    });
}

timer!(nrf52840_hal::target::TIMER0, TIMER0, TIMER0_STATE);
timer!(nrf52840_hal::target::TIMER1, TIMER1, TIMER1_STATE);
timer!(nrf52840_hal::target::TIMER2, TIMER2, TIMER2_STATE);
timer!(nrf52840_hal::target::TIMER3, TIMER3, TIMER3_STATE);
timer!(nrf52840_hal::target::TIMER4, TIMER4, TIMER4_STATE);
