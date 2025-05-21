//! Task and future related utilities.


use crate::time::Instant;
use core::pin::Pin;
use core::task::{ Context, Poll };
use core::time::Duration;
use pin_project::pin_project;


pub use wasm_rs_async_executor::single_threaded::{ spawn, TaskHandle, Task, JoinError };


/// Pauses the current task for some duration.
///
/// Execution might not be paused for the exact duration given,
///  but it will never unpause before the duration expires.
#[inline]
pub fn sleep(duration : Duration) -> Sleep { Sleep { timeout : Instant::now() + duration } }

/// Pauses the current task until some point in time.
///
/// Execution might not be paused until the exact point in time given,
///  but it will never unpause before the timeout expires.
#[inline]
pub fn sleep_until(timeout : Instant) -> Sleep { Sleep { timeout } }

/// A `Future` which waits until a specific point in time.
///
/// See [`sleep`] and [`sleep_until`].
pub struct Sleep {
    timeout : Instant
}
impl Future for Sleep {
    type Output = ();
    fn poll(self : Pin<&mut Self>, ctx : &mut Context<'_>) -> Poll<Self::Output> {
        if (Instant::now() >= self.timeout) {
            Poll::Ready(())
        } else {
            //ctx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}


/// Polls a `Future` for some duration.
///
/// If the `Future` completes, the result is returned. If it takes
///  longer than the given duration, it is cancelled.
#[inline]
pub fn timeout<F, T>(duration : Duration, fut : F) -> Timeout<F, T>
where
    F : Future<Output = T>
{ Timeout { timeout : Instant::now() + duration, fut } }

/// Polls a `Future` until some point in time.
///
/// If the `Future` completes, the result is returned. If it does not
///  finish before the given point in time, it is cancelled.
#[inline]
pub fn timeout_at<F, T>(timeout : Instant, fut : F) -> Timeout<F, T>
where
    F : Future<Output = T>
{ Timeout { timeout, fut } }

/// A `Future` which polls another `Future` until some specific point in time.
///
/// See [`timeout`] and [`timeout_at`].
#[pin_project]
pub struct Timeout<F, T>
where
    F : Future<Output = T>
{
    timeout : Instant,
    #[pin]
    fut     :  F
}

/// An error returned by [`Timeout`] if the `Future` exceeded the maximum execution time.
pub struct TimeoutError {
    /// The `Instant` when the `Future` expired.
    pub at : Instant
}

impl<F, T> Future for Timeout<F, T>
where
    F : Future<Output = T>
{
    type Output = Result<T, TimeoutError>;
    fn poll(mut self : Pin<&mut Self>, ctx : &mut Context<'_>) -> Poll<Self::Output> {
        match (self.as_mut().project().fut.poll(ctx)) {
            Poll::Ready(out) => Poll::Ready(Ok(out)),
            Poll::Pending => {
                if (Instant::now() >= self.timeout) {
                    Poll::Ready(Err(TimeoutError { at : self.timeout }))
                } else { Poll::Pending }
            }
        }
    }
}
