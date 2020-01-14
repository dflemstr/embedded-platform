use core::future;
use core::pin;
use core::task;

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Tick<'a, A>
where
    A: super::Timer + Unpin + ?Sized,
{
    timer: &'a mut A,
}

pub fn tick<A>(timer: &mut A) -> Tick<A>
where
    A: super::Timer + Unpin + ?Sized,
{
    Tick { timer }
}

impl<A> future::Future for Tick<'_, A>
where
    A: super::Timer + Unpin + ?Sized,
{
    type Output = Result<(), A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        pin::Pin::new(&mut *this.timer).poll_tick(cx)
    }
}
