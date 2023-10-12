use std::{any::Any, future::Future, time::Duration};

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
