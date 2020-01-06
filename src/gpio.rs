use core::pin;
use core::task;

pub mod get;
pub mod set;

pub trait Pin {
    type Error;
}

pub trait InputPin: Pin {
    fn poll_get(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<bool, Self::Error>>;
}

pub trait InputPinExt: InputPin {
    fn get(&mut self) -> get::Get<Self>
    where
        Self: Unpin,
    {
        get::get(self)
    }
}

impl<A> InputPinExt for A where A: InputPin {}

pub trait OutputPin: Pin {
    fn poll_set(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        high: bool,
    ) -> task::Poll<Result<(), Self::Error>>;
}

pub trait OutputPinExt: OutputPin {
    fn set(&mut self, high: bool) -> set::Set<Self>
    where
        Self: Unpin,
    {
        set::set(self, high)
    }
}

impl<A> OutputPinExt for A where A: OutputPin {}

pub trait IntoFloatingInputPin: Pin {
    type FloatingInputPin: InputPin<Error = Self::Error> + Unpin;

    fn into_floating_input_pin(self) -> Result<Self::FloatingInputPin, Self::Error>;
}

pub trait IntoPullUpInputPin: Pin {
    type PullUpInputPin: InputPin<Error = Self::Error> + Unpin;

    fn into_pull_up_input_pin(self) -> Result<Self::PullUpInputPin, Self::Error>;
}

pub trait IntoPullDownInputPin: Pin {
    type PullDownInputPin: InputPin<Error = Self::Error> + Unpin;

    fn into_pull_down_input_pin(self) -> Result<Self::PullDownInputPin, Self::Error>;
}

pub trait IntoOpenDrainOutputPin: Pin {
    type OpenDrainOutputPin: OutputPin<Error = Self::Error> + Unpin;

    fn into_open_drain_output_pin(
        self,
        initial_high: bool,
    ) -> Result<Self::OpenDrainOutputPin, Self::Error>;
}

pub trait IntoPushPullOutputPin: Pin {
    type PushPullOutputPin: OutputPin<Error = Self::Error> + Unpin;

    fn into_push_pull_output_pin(
        self,
        initial_high: bool,
    ) -> Result<Self::PushPullOutputPin, Self::Error>;
}

#[derive(Clone, Copy, Debug)]
pub struct NoConnect;

impl Pin for NoConnect {
    type Error = futures::never::Never;
}

impl InputPin for NoConnect {
    fn poll_get(
        self: pin::Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<bool, Self::Error>> {
        task::Poll::Ready(Ok(false))
    }
}

impl OutputPin for NoConnect {
    fn poll_set(
        self: pin::Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
        _high: bool,
    ) -> task::Poll<Result<(), Self::Error>> {
        task::Poll::Ready(Ok(()))
    }
}

impl IntoFloatingInputPin for NoConnect {
    type FloatingInputPin = Self;

    fn into_floating_input_pin(self) -> Result<Self::FloatingInputPin, Self::Error> {
        Ok(self)
    }
}

impl IntoPullUpInputPin for NoConnect {
    type PullUpInputPin = Self;

    fn into_pull_up_input_pin(self) -> Result<Self::PullUpInputPin, Self::Error> {
        Ok(self)
    }
}

impl IntoPullDownInputPin for NoConnect {
    type PullDownInputPin = Self;

    fn into_pull_down_input_pin(self) -> Result<Self::PullDownInputPin, Self::Error> {
        Ok(self)
    }
}

impl IntoOpenDrainOutputPin for NoConnect {
    type OpenDrainOutputPin = Self;

    fn into_open_drain_output_pin(
        self,
        _initial_high: bool,
    ) -> Result<Self::OpenDrainOutputPin, Self::Error> {
        Ok(self)
    }
}

impl IntoPushPullOutputPin for NoConnect {
    type PushPullOutputPin = Self;

    fn into_push_pull_output_pin(
        self,
        _initial_high: bool,
    ) -> Result<Self::PushPullOutputPin, Self::Error> {
        Ok(self)
    }
}
