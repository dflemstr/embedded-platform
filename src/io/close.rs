use core::future;
use core::pin;
use core::task;

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Close<'a, A: ?Sized> {
    writer: &'a mut A,
}

pub fn close<A>(writer: &mut A) -> Close<A>
where
    A: super::Write + Unpin + ?Sized,
{
    Close { writer }
}

impl<A> future::Future for Close<'_, A>
where
    A: super::Write + Unpin + ?Sized,
{
    type Output = Result<(), A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        pin::Pin::new(&mut *this.writer).poll_close(cx)
    }
}
