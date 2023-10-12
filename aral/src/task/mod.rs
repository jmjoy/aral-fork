use crate::get_runtime;
use aral_trait::task::{JoinHandle, Task};
use std::{
    any::Any,
    future::Future,
    pin::{pin, Pin},
    result,
    task::{Context, Poll},
    time::Duration,
};

#[inline]
pub async fn sleep(duration: Duration) {
    get_runtime().sleep(duration).await
}

#[inline]
pub fn spawn<T: Send + 'static>(
    future: impl Future<Output = T> + Send + 'static,
) -> impl JoinHandle<T> {
    get_runtime().spawn(future)
}

#[inline]
pub fn spawn_blocking<T: Send + 'static>(
    f: impl FnOnce() -> T + Send + 'static,
) -> impl JoinHandle<T> {
    get_runtime().spawn_blocking(f)
}
