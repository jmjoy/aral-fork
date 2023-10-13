use crate::current_runtime;
use aral_trait::task::{JoinHandle, Task};
use std::{future::Future, time::Duration};

#[inline]
pub async fn sleep(duration: Duration) {
    current_runtime().sleep(duration).await
}

#[inline]
pub fn spawn<T: Send + 'static>(
    future: impl Future<Output = T> + Send + 'static,
) -> impl JoinHandle<T> {
    current_runtime().spawn(future)
}

#[inline]
pub fn spawn_blocking<T: Send + 'static>(
    f: impl FnOnce() -> T + Send + 'static,
) -> impl JoinHandle<T> {
    current_runtime().spawn_blocking(f)
}
