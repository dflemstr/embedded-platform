use core::future;
use core::pin;
use core::task;

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Shutdown<'a, A: ?Sized> {
    writer: &'a mut A,
}

pub(crate) fn shutdown<A>(writer: &mut A) -> Shutdown<A>
where
    A: super::Write + Unpin + ?Sized,
{
    Shutdown { writer }
}

impl<A> future::Future for Shutdown<'_, A>
where
    A: super::Write + Unpin + ?Sized,
{
    type Output = Result<(), A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        pin::Pin::new(&mut *this.writer).poll_shutdown(cx)
    }
}
