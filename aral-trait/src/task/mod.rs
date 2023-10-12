use std::{any::Any, future::Future, time::Duration};

pub trait JoinHandle<T>: Future<Output = Result<T, Box<dyn Any + Send + 'static>>> {}

pub trait Task {
    fn sleep(duration: Duration) -> impl Future<Output = ()> + Send;

    fn spawn<T: Send + 'static>(
        future: impl Future<Output = T> + Send + 'static,
    ) -> impl JoinHandle<T> + Send;

    fn spawn_blocking<T: Send + 'static>(
        f: impl FnOnce() -> T + Send + 'static,
    ) -> impl JoinHandle<T> + Send;
}
