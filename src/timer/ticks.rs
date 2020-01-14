use core::pin;
use core::task;

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Ticks<'a, A>
where
    A: super::Timer + Unpin + ?Sized,
{
    timer: &'a mut A,
}

pub fn ticks<A>(timer: &mut A) -> Ticks<A>
where
    A: super::Timer + Unpin + ?Sized,
{
    Ticks { timer }
}

impl<A> futures::stream::Stream for Ticks<'_, A>
where
    A: super::Timer + Unpin + ?Sized,
{
    type Item = Result<(), A::Error>;

    fn poll_next(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        let this = &mut *self;
        pin::Pin::new(&mut *this.timer).poll_tick(cx).map(Some)
    }
}
