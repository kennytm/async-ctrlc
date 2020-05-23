// Copyright 2019 kennytm. Licensed under MIT OR Apache-2.0.

//! `async-ctrlc` is an async wrapper of the `ctrlc` crate.

use ctrlc::set_handler;
pub use ctrlc::Error;
#[cfg(feature = "stream")]
use futures_core::stream::Stream;
use std::{
    future::Future,
    pin::Pin,
    ptr::null_mut,
    sync::atomic::{AtomicBool, AtomicPtr, Ordering},
    task::{Context, Poll, Waker},
};

// TODO: Replace this with `AtomicOptionBox<Waker>`
// after https://github.com/jorendorff/atomicbox/pull/3 is merged.
static WAKER: AtomicPtr<Waker> = AtomicPtr::new(null_mut());
static ACTIVE: AtomicBool = AtomicBool::new(false);

/// A future which is fulfilled when the program receives the Ctrl+C signal.
#[derive(Debug)]
pub struct CtrlC {
    _private: (),
}

impl Future for CtrlC {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if ACTIVE.swap(false, Ordering::SeqCst) {
            Poll::Ready(())
        } else {
            let new_waker = Box::new(cx.waker().clone());
            let old_waker_ptr = WAKER.swap(Box::into_raw(new_waker), Ordering::SeqCst);
            if !old_waker_ptr.is_null() {
                unsafe { Box::from_raw(old_waker_ptr) };
            }
            Poll::Pending
        }
    }
}

#[cfg(feature = "stream")]
impl Stream for CtrlC {
    type Item = ();
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.poll(cx).map(Some)
    }
}

impl CtrlC {
    /// Creates a new `CtrlC` future.
    ///
    /// There should be at most one `CtrlC` instance in the whole program. The
    /// second call to `Ctrl::new()` would return an error.
    pub fn new() -> Result<Self, Error> {
        set_handler(|| {
            ACTIVE.store(true, Ordering::SeqCst);
            let waker_ptr = WAKER.swap(null_mut(), Ordering::SeqCst);
            if !waker_ptr.is_null() {
                unsafe { Box::from_raw(waker_ptr) }.wake();
            }
        })?;
        Ok(CtrlC { _private: () })
    }
}

#[cfg(unix)]
#[test]
fn test_unix() {
    use async_std::{future::timeout, task::block_on};
    use libc::{getpid, kill, SIGINT};
    use std::{
        thread::{sleep, spawn},
        time::Duration,
    };

    let thread = spawn(|| unsafe {
        sleep(Duration::from_millis(100));
        kill(getpid(), SIGINT);
    });

    let c = CtrlC::new().unwrap();
    let result = block_on(async move {
        let i = 1;
        timeout(Duration::from_millis(500), c).await.unwrap();
        i + 2
    });
    assert_eq!(result, 3);

    thread.join().unwrap();
}
