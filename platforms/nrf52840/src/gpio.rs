use crate::error;
use core::fmt;
use core::pin;
use core::task;
use nrf52840_hal::gpio;
use nrf52840_hal::gpio::p0;
use nrf52840_hal::gpio::p1;

#[repr(transparent)]
pub struct Pin<P>(pub(crate) P)
where
    P: Unpin + ?Sized;

macro_rules! gpio {
    ($($m:ident: $mtyp:ident => [$($name:ident: $typ:ident,)*],)*) => {
    $(
        #[derive(Debug)]
        pub(crate) struct $mtyp {
            $(pub(crate) $name: Option<Pin<$m::$typ<gpio::Input<gpio::Floating>>>>,)*
        }

        impl $mtyp {
            pub(crate) fn new(port: nrf52840_hal::nrf52840_pac::$mtyp) -> Self {
                let port = $m::Parts::new(port);

                Self {
                    $($name: Some(Pin(port.$name)),)*
                }
            }
        }
    )*
    $($(
        impl<S> embedded_platform::gpio::Pin for Pin<$m::$typ<S>> where S: Unpin {
            type Error = error::Error;
        }

        impl<S> embedded_platform::gpio::InputPin for Pin<$m::$typ<gpio::Input<S>>> where S: Unpin {
            fn poll_get(
                self: pin::Pin<&mut Self>,
                _cx: &mut task::Context<'_>,
            ) -> task::Poll<Result<bool, Self::Error>> {
                use embedded_hal::digital::v2::InputPin;
                task::Poll::Ready(Ok(self.0.is_high().unwrap()))
            }
        }

        impl<S> embedded_platform::gpio::OutputPin for Pin<$m::$typ<gpio::Output<S>>> where S: Unpin {
            fn poll_set(
                mut self: pin::Pin<&mut Self>,
                _cx: &mut task::Context<'_>,
                high: bool,
            ) -> task::Poll<Result<(), Self::Error>> {
                use embedded_hal::digital::v2::OutputPin;

                let this = &mut *self;
                task::Poll::Ready(Ok(if high { this.0.set_high().unwrap() } else { this.0.set_low().unwrap() }))
            }
        }

        impl<S> embedded_platform::gpio::IntoFloatingInputPin for Pin<$m::$typ<S>> where S: Unpin {
            type FloatingInputPin = Pin<$m::$typ<gpio::Input<gpio::Floating>>>;

            fn into_floating_input_pin(self) -> Result<Self::FloatingInputPin, Self::Error> {
                Ok(Pin(self.0.into_floating_input()))
            }
        }

        impl<S> embedded_platform::gpio::IntoPullDownInputPin for Pin<$m::$typ<S>> where S: Unpin {
            type PullDownInputPin = Pin<$m::$typ<gpio::Input<gpio::PullDown>>>;

            fn into_pull_down_input_pin(self) -> Result<Self::PullDownInputPin, Self::Error> {
                Ok(Pin(self.0.into_pulldown_input()))
            }
        }

        impl<S> embedded_platform::gpio::IntoPullUpInputPin for Pin<$m::$typ<S>> where S: Unpin {
            type PullUpInputPin = Pin<$m::$typ<gpio::Input<gpio::PullUp>>>;

            fn into_pull_up_input_pin(self) -> Result<Self::PullUpInputPin, Self::Error> {
                Ok(Pin(self.0.into_pullup_input()))
            }
        }

        impl<S> embedded_platform::gpio::IntoPushPullOutputPin for Pin<$m::$typ<S>> where S: Unpin {
            type PushPullOutputPin = Pin<$m::$typ<gpio::Output<gpio::PushPull>>>;

            fn into_push_pull_output_pin(self, initial_high: bool) -> Result<Self::PushPullOutputPin, Self::Error> {
                Ok(Pin(self.0.into_push_pull_output(if initial_high { gpio::Level::High } else { gpio::Level::Low })))
            }
        }

        impl<S> embedded_platform::gpio::IntoOpenDrainOutputPin for Pin<$m::$typ<S>> where S: Unpin {
            type OpenDrainOutputPin = Pin<$m::$typ<gpio::Output<gpio::OpenDrain>>>;

            fn into_open_drain_output_pin(self, initial_high: bool) -> Result<Self::OpenDrainOutputPin, Self::Error> {
                Ok(Pin(self.0.into_open_drain_output(gpio::OpenDrainConfig::Disconnect0Standard1, if initial_high { gpio::Level::High } else { gpio::Level::Low })))
            }
        }

        impl<S> fmt::Debug for Pin<$m::$typ<gpio::Input<S>>> where S: Unpin {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // TODO: display current GPIO state
                f.debug_struct(stringify!($typ)).finish()
            }
        }

        impl<S> fmt::Debug for Pin<$m::$typ<gpio::Output<S>>> where S: Unpin {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // TODO: display current GPIO state
                f.debug_struct(stringify!($typ)).finish()
            }
        }
    )*)*
    };
}

gpio! {
    p0: P0 => [
        p0_00: P0_00,
        p0_01: P0_01,
        p0_02: P0_02,
        p0_03: P0_03,
        p0_04: P0_04,
        p0_05: P0_05,
        p0_06: P0_06,
        p0_07: P0_07,
        p0_08: P0_08,
        p0_09: P0_09,
        p0_10: P0_10,
        p0_11: P0_11,
        p0_12: P0_12,
        p0_13: P0_13,
        p0_14: P0_14,
        p0_15: P0_15,
        p0_16: P0_16,
        p0_17: P0_17,
        p0_18: P0_18,
        p0_19: P0_19,
        p0_20: P0_20,
        p0_21: P0_21,
        p0_22: P0_22,
        p0_23: P0_23,
        p0_24: P0_24,
        p0_25: P0_25,
        p0_26: P0_26,
        p0_27: P0_27,
        p0_28: P0_28,
        p0_29: P0_29,
        p0_30: P0_30,
        p0_31: P0_31,
    ],
    p1: P1 => [
        p1_00: P1_00,
        p1_01: P1_01,
        p1_02: P1_02,
        p1_03: P1_03,
        p1_04: P1_04,
        p1_05: P1_05,
        p1_06: P1_06,
        p1_07: P1_07,
        p1_08: P1_08,
        p1_09: P1_09,
        p1_10: P1_10,
        p1_11: P1_11,
        p1_12: P1_12,
        p1_13: P1_13,
        p1_14: P1_14,
        p1_15: P1_15,
    ],
}
