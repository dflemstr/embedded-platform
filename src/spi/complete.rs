use core::future;
use core::pin;
use core::task;

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Complete<'a, A: ?Sized> {
    transfer: &'a mut A,
}

pub fn complete<'a, A>(transfer: &'a mut A) -> Complete<'a, A>
where
    A: super::SpiTransfer<'a> + Unpin + ?Sized,
{
    Complete { transfer }
}

impl<'a, A> future::Future for Complete<'a, A>
where
    A: super::SpiTransfer<'a> + Unpin + ?Sized,
{
    type Output = Result<(), A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        pin::Pin::new(&mut *this.transfer).poll_complete(cx)
    }
}
