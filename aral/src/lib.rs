#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]
#![feature(cfg_match)]
#![feature(type_alias_impl_trait)]

pub mod fs;
pub mod io;
pub mod net;
pub mod task;

pub trait Runtime {
    fn name(&self) -> &'static str;

    fn fs(&self) -> &'static impl fs::Fs;

    fn task(&self) -> &'static impl task::Task;

    fn net(&self) -> &'static impl net::Net;
}


#[inline]
pub fn current_runtime() -> &'static impl Runtime {
    cfg_match! {
        cfg(feature = "runtime-tokio") => {
            use aral_runtime_tokio::TokioRuntime as R;
        }
        _ => {
            use aral_runtime_noop::NoopRuntime as R;
        }
    }
    &R
}
