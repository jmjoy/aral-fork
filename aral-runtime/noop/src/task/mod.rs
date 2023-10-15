use crate::{NoopData, NoopRuntime};
use aral_trait::task::{JoinHandle, Task};
use std::{
    any::Any,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

impl<T> Future for NoopData<T> {
    type Output = Result<T, Box<(dyn Any + Send + 'static)>>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        no_runtime_specified!();
    }
}

impl<T> JoinHandle<T> for NoopData<T> {}

impl Task for NoopRuntime {
    #[inline]
    fn sleep(&self, _duration: std::time::Duration) -> impl std::future::Future<Output = ()> {
        no_runtime_specified!();
        async move {}
    }

    #[inline]
    fn spawn<T: Send + 'static>(
        &self, _future: impl std::future::Future<Output = T> + Send + 'static,
    ) -> impl JoinHandle<T> {
        no_runtime_specified!();
        NoopData(PhantomData)
    }

    #[inline]
    fn spawn_blocking<T: Send + 'static>(
        &self, _f: impl FnOnce() -> T + Send + 'static,
    ) -> impl JoinHandle<T> {
        no_runtime_specified!();
        NoopData(PhantomData)
    }
}
