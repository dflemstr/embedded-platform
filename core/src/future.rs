//! Asynchronous values.

use core::cell;
use core::ops;
use core::pin;
use core::ptr;
use core::task;

#[doc(inline)]
pub use core::future::*;

/// Wrap a generator in a future.
///
/// This function returns a `GenFuture` underneath, but hides it in `impl Trait` to give
/// better error messages (`impl Future` rather than `GenFuture<[closure.....]>`).
#[doc(hidden)]
pub fn from_generator<T: ops::Generator<Yield = ()>>(x: T) -> impl Future<Output = T::Return> {
    GenFuture(x)
}

/// A wrapper around generators used to implement `Future` for `async`/`await` code.
#[doc(hidden)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct GenFuture<T: ops::Generator<Yield = ()>>(T);

// We rely on the fact that async/await futures are immovable in order to create
// self-referential borrows in the underlying generator.
impl<T: ops::Generator<Yield = ()>> !Unpin for GenFuture<T> {}

#[doc(hidden)]
impl<T: ops::Generator<Yield = ()>> Future for GenFuture<T> {
    type Output = T::Return;
    fn poll(self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        // Safe because we're !Unpin + !Drop mapping to a ?Unpin value
        let gen = unsafe { pin::Pin::map_unchecked_mut(self, |s| &mut s.0) };
        set_task_context(cx, || match gen.resume() {
            ops::GeneratorState::Yielded(()) => task::Poll::Pending,
            ops::GeneratorState::Complete(x) => task::Poll::Ready(x),
        })
    }
}

static CONTEXT: bare_metal::Mutex<cell::Cell<ContextHolder>> =
    bare_metal::Mutex::new(cell::Cell::new(ContextHolder(None)));

#[derive(Clone, Copy)]
struct ContextHolder(Option<ptr::NonNull<task::Context<'static>>>);

unsafe impl Send for ContextHolder {}

struct SetOnDrop(ContextHolder);

impl Drop for SetOnDrop {
    fn drop(&mut self) {
        cortex_m::interrupt::free(|cs| {
            CONTEXT.borrow(cs).set(ContextHolder((self.0).0.take()));
        });
    }
}

#[doc(hidden)]
/// Sets the thread-local task context used by async/await futures.
pub fn set_task_context<F, R>(cx: &mut task::Context<'_>, f: F) -> R
where
    F: FnOnce() -> R,
{
    // transmute the context's lifetime to 'static so we can store it.
    let cx =
        unsafe { core::mem::transmute::<&mut task::Context<'_>, &mut task::Context<'static>>(cx) };
    let old_cx = cortex_m::interrupt::free(|cs| {
        CONTEXT
            .borrow(cs)
            .replace(ContextHolder(Some(ptr::NonNull::from(cx))))
    });
    let _reset = SetOnDrop(old_cx);
    f()
}

#[doc(hidden)]
/// Retrieves the thread-local task context used by async/await futures.
///
/// This function acquires exclusive access to the task context.
///
/// Panics if no context has been set or if the context has already been
/// retrieved by a surrounding call to get_task_context.
pub fn get_task_context<F, R>(f: F) -> R
where
    F: FnOnce(&mut task::Context<'_>) -> R,
{
    // Clear the entry so that nested `get_task_waker` calls
    // will fail or set their own value.
    let cx_ptr = cortex_m::interrupt::free(|cs| CONTEXT.borrow(cs).replace(ContextHolder(None)));
    let _reset = SetOnDrop(cx_ptr);

    let mut cx_ptr = cx_ptr.0.expect(
        "TLS Context not set. This is a rustc bug. \
        Please file an issue on https://github.com/rust-lang/rust.",
    );

    // Safety: we've ensured exclusive access to the context by
    // removing the pointer from TLS, only to be replaced once
    // we're done with it.
    //
    // The pointer that was inserted came from an `&mut Context<'_>`,
    // so it is safe to treat as mutable.
    unsafe { f(cx_ptr.as_mut()) }
}

#[doc(hidden)]
/// Polls a future in the current thread-local task waker.
pub fn poll_with_tls_context<F>(f: pin::Pin<&mut F>) -> task::Poll<F::Output>
where
    F: Future,
{
    get_task_context(|cx| F::poll(f, cx))
}
