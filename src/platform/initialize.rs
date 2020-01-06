use core::fmt;
use core::future;
use core::marker;
use core::pin;
use core::task;

pub struct Initialize<P> {
    phantom: marker::PhantomData<P>,
}

pub fn initialize<P>() -> Initialize<P> {
    let phantom = marker::PhantomData;
    Initialize { phantom }
}

impl<P> future::Future for Initialize<P>
where
    P: super::Platform,
{
    type Output = Result<P, P::Error>;

    fn poll(self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        P::poll_initialize(cx)
    }
}

impl<P> fmt::Debug for Initialize<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Initialize").finish()
    }
}
