#![allow(clippy::missing_safety_doc)]

use crate::error;
use core::cell;
use core::fmt;
use core::pin;
use core::task;
use nrf52840_hal::target::interrupt;

pub struct Timer<T>(u32, T)
where
    T: Unpin + ?Sized;

#[derive(Debug)]
pub(crate) struct Timers {
    pub timer0: Option<Timer<nrf52840_hal::timer::Timer<nrf52840_hal::target::TIMER0>>>,
    pub timer1: Option<Timer<nrf52840_hal::timer::Timer<nrf52840_hal::target::TIMER1>>>,
    pub timer2: Option<Timer<nrf52840_hal::timer::Timer<nrf52840_hal::target::TIMER2>>>,
    pub timer3: Option<Timer<nrf52840_hal::timer::Timer<nrf52840_hal::target::TIMER3>>>,
    pub timer4: Option<Timer<nrf52840_hal::timer::Timer<nrf52840_hal::target::TIMER4>>>,
}

impl<T> Timer<T>
where
    T: Unpin,
{
    fn new(raw: T) -> Self {
        Self(1, raw)
    }
}

impl<T> fmt::Debug for Timer<T>
where
    T: Unpin + ?Sized,
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

#[derive(Debug)]
struct InterruptState {
    triggered: bool,
    waker: Option<task::Waker>,
}

type InterruptStateCell = bare_metal::Mutex<cell::RefCell<InterruptState>>;

macro_rules! timer {
    ($ty:ty, $interrupt:ident, $state:ident) => {
        static $state: InterruptStateCell =
            bare_metal::Mutex::new(cell::RefCell::new(InterruptState {
                triggered: false,
                waker: None,
            }));

        impl<M> embedded_platform::timer::Timer for Timer<nrf52840_hal::timer::Timer<$ty, M>>
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
                this.1.start(this.0);

                task::Poll::Ready(Ok(()))
            }

            fn poll_tick(
                self: pin::Pin<&mut Self>,
                cx: &mut task::Context<'_>,
            ) -> task::Poll<Result<(), Self::Error>> {
                impl_poll_tick(cx, &$state)
            }
        }

        impl<M> embedded_platform::timer::IntoPeriodicTimer
            for Timer<nrf52840_hal::timer::Timer<$ty, M>>
        where
            M: Unpin,
        {
            type PeriodicTimer =
                Timer<nrf52840_hal::timer::Timer<$ty, nrf52840_hal::timer::Periodic>>;

            fn into_periodic_timer(
                self,
                rate: embedded_platform::time::Rate,
            ) -> Result<Self::PeriodicTimer, Self::Error> {
                let result = self.1.into_periodic();
                Ok(Timer(
                    (nrf52840_hal::timer::Timer::<$ty, M>::TICKS_PER_SECOND as f32 / rate.as_hz())
                        as u32,
                    result,
                ))
            }
        }

        impl<M> embedded_platform::timer::IntoOneshotTimer
            for Timer<nrf52840_hal::timer::Timer<$ty, M>>
        where
            M: Unpin,
        {
            type OneshotTimer =
                Timer<nrf52840_hal::timer::Timer<$ty, nrf52840_hal::timer::OneShot>>;

            fn into_oneshot_timer(
                self,
                period: embedded_platform::time::Duration,
            ) -> Result<Self::OneshotTimer, Self::Error> {
                let result = self.1.into_oneshot();
                Ok(Timer(
                    period.as_nanos() * nrf52840_hal::timer::Timer::<$ty, M>::TICKS_PER_SECOND
                        / 1_000_000,
                    result,
                ))
            }
        }

        #[cfg(feature = "rt")]
        #[interrupt]
        fn $interrupt() {
            interrupt_impl(
                &$state,
                nrf52840_hal::target::Interrupt::$interrupt,
                || unsafe { (*<$ty>::ptr()).events_compare[0].write(|w| w) },
            );
        }
    };
}

fn impl_poll_tick<E>(
    cx: &mut task::Context,
    state: &InterruptStateCell,
) -> task::Poll<Result<(), E>> {
    cortex_m::interrupt::free(|cs| {
        let mut state = state.borrow(cs).borrow_mut();
        if state.triggered {
            state.triggered = false;
            task::Poll::Ready(Ok(()))
        } else {
            state.waker = Some(cx.waker().clone());
            task::Poll::Pending
        }
    })
}

fn interrupt_impl(
    state: &InterruptStateCell,
    interrupt: nrf52840_hal::target::Interrupt,
    clear: impl FnOnce(),
) {
    clear();
    cortex_m::peripheral::NVIC::unpend(interrupt);

    if let Some(waker) = cortex_m::interrupt::free(|cs| {
        let mut state = state.borrow(cs).borrow_mut();
        state.triggered = true;
        state.waker.take()
    }) {
        waker.wake()
    }
}

timer!(nrf52840_hal::target::TIMER0, TIMER0, TIMER0_STATE);
timer!(nrf52840_hal::target::TIMER1, TIMER1, TIMER1_STATE);
timer!(nrf52840_hal::target::TIMER2, TIMER2, TIMER2_STATE);
timer!(nrf52840_hal::target::TIMER3, TIMER3, TIMER3_STATE);
timer!(nrf52840_hal::target::TIMER4, TIMER4, TIMER4_STATE);
