use aral_trait::task::{JoinHandle, Task};
use std::{
    any::Any,
    future::Future,
    pin::{pin, Pin},
    task::{Context, Poll},
};

pub struct TokioJoinHandle<T>(tokio::task::JoinHandle<T>);

impl<T> Future for TokioJoinHandle<T> {
    type Output = Result<T, Box<dyn Any + Send + 'static>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        pin!(&mut self.0)
            .poll(cx)
            .map(|r| r.map_err(|err| err.into_panic()))
    }
}

impl<T> JoinHandle<T> for TokioJoinHandle<T> {}

pub struct TokioTask;

impl Task for TokioTask {
    #[inline]
    fn sleep(&self, duration: std::time::Duration) -> impl std::future::Future<Output = ()> {
        tokio::time::sleep(duration)
    }

    #[inline]
    fn spawn<T: Send + 'static>(
        &self, future: impl std::future::Future<Output = T> + Send + 'static,
    ) -> impl JoinHandle<T> {
        TokioJoinHandle(tokio::spawn(future))
    }

    #[inline]
    fn spawn_blocking<T: Send + 'static>(
        &self, f: impl FnOnce() -> T + Send + 'static,
    ) -> impl JoinHandle<T> {
        TokioJoinHandle(tokio::task::spawn_blocking(f))
    }
}
