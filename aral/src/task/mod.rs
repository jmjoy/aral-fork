use std::{any::Any, future::Future, time::Duration};
use crate::{current_runtime, Runtime};

pub trait JoinHandle<T>: Future<Output = Result<T, Box<dyn Any + Send + 'static>>> {}

pub trait Task {
    fn sleep(&self, duration: Duration) -> impl Future<Output = ()>;

    fn spawn<T: Send + 'static>(
        &self, future: impl Future<Output = T> + Send + 'static,
    ) -> impl JoinHandle<T>;

    fn spawn_blocking<T: Send + 'static>(
        &self, f: impl FnOnce() -> T + Send + 'static,
    ) -> impl JoinHandle<T>;
}


#[inline]
pub async fn sleep(duration: Duration) {
    current_runtime().task().sleep(duration).await
}

#[inline]
pub fn spawn<T: Send + 'static>(
    future: impl Future<Output = T> + Send + 'static,
) -> impl JoinHandle<T> {
    current_runtime().task().spawn(future)
}

#[inline]
pub fn spawn_blocking<T: Send + 'static>(
    f: impl FnOnce() -> T + Send + 'static,
) -> impl JoinHandle<T> {
    current_runtime().task().spawn_blocking(f)
}
