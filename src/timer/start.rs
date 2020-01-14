use core::future;
use core::pin;
use core::task;

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Start<'a, A>
where
    A: super::Timer + Unpin + ?Sized,
{
    timer: &'a mut A,
}

pub fn start<A>(timer: &mut A) -> Start<A>
where
    A: super::Timer + Unpin + ?Sized,
{
    Start { timer }
}

impl<A> future::Future for Start<'_, A>
where
    A: super::Timer + Unpin + ?Sized,
{
    type Output = Result<(), A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        pin::Pin::new(&mut *this.timer).poll_start(cx)
    }
}
